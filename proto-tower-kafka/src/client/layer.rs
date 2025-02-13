use crate::client::KafkaProtoClientConfig;
use crate::data::{KafkaProtocolError, KafkaRequest, KafkaResponse};
use bytes::{Buf, BytesMut};
use kafka_protocol::messages::*;
use kafka_protocol::protocol::buf::ByteBuf;
use kafka_protocol::protocol::Decodable;
use paste::paste;
use proto_tower_util::WriteTo;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, ReadHalf, SimplexStream, WriteHalf};
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tower::Service;

#[derive(Clone)]
pub struct ProtoKafkaClientService<Svc, E>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
{
    inner: Svc,
    config: KafkaProtoClientConfig,
}

impl<Svc, E> ProtoKafkaClientService<Svc, E>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
{
    pub fn new(inner: Svc, config: KafkaProtoClientConfig) -> Self {
        ProtoKafkaClientService { inner, config }
    }
}

impl<Svc, E, SvcFut> Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>)> for ProtoKafkaClientService<Svc, E>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<(), E>> + Send + 'static,
    E: Debug + Send + 'static,
{
    type Response = ();
    type Error = KafkaProtocolError<E>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| KafkaProtocolError::InternalServiceError(e))
    }

    fn call(&mut self, (mut rx_chan, sx_chan): (Receiver<KafkaRequest>, Sender<KafkaResponse>)) -> Self::Future {
        let mut inner = self.inner.clone();
        let config = self.config.clone();
        Box::pin(async move {
            let mut next_correlation_id = 1;
            // Tracked requests
            // correlation_id -> (api_key, header_version)
            let mut tracked_requests = BTreeMap::<i32, (ApiKey, i16)>::new();
            // Create channel
            let (svc_read, mut write) = tokio::io::simplex(1024);
            let (mut read, svc_write) = tokio::io::simplex(1024);
            // Spawn handler
            let f = inner.call((svc_read, svc_write));
            let task = tokio::spawn(f);
            let mut buf_mut = BytesMut::with_capacity(1024);
            loop {
                eprintln!("Client loop tick");
                select!(
                    r = rx_chan.recv() => {
                        eprintln!("Client request read: {:?}", r);
                        match r {
                            None => {
                                return Ok(())
                            }
                            Some(req) => {
                                // Generate correlation_id
                                let correlation_id: i32 = next_correlation_id;
                                next_correlation_id += 1;
                                let version = 3;
                                tracked_requests.insert(correlation_id, (req.api_key(), version));
                                req.into_full(version, correlation_id, config.client_id.clone()).write_to(&mut write).await?;
                            }
                        }
                    }
                    d = read.read_buf(&mut buf_mut) => {
                        eprintln!("Client response read: {:?}", d);
                        match d {
                            Ok(sz) => {
                                if sz == 0 {
                                    eprintln!("Received size 0, breaking");
                                    break;
                                }
                                let resp = parse_response(&mut buf_mut, &mut tracked_requests).await?;
                                if let Some(resp) = resp {
                                    eprintln!("Client read Response: {:?}", resp);
                                    if let Err(_) = sx_chan.send(resp).await {
                                        eprintln!("Error sending response");
                                        return Err(KafkaProtocolError::UnhandledImplementation("Error sending response"))
                                    }
                                }
                            }
                            Err(a) => {
                                eprintln!("Error reading from stream: {:?}", a);
                                return Err(KafkaProtocolError::UnhandledImplementation("Error reading from stream"));
                            }
                        };
                    }
                    _ = tokio::time::sleep(config.timeout) => {
                        eprintln!("Client timeout");
                        if config.fail_on_inactivity {
                            return Err(KafkaProtocolError::UnhandledImplementation("Client Timeout"));
                        } else {
                            break;
                        }
                    }
                );
                if task.is_finished() {
                    break;
                }
            }
            drop(write);
            drop(read);
            drop(rx_chan);
            drop(sx_chan);
            eprintln!("Client loop finished");
            task.await
                .map_err(|_| KafkaProtocolError::InternalServiceClosed)?
                .map_err(|e| KafkaProtocolError::InternalServiceError(e))
        })
    }
}

async fn parse_response<E: Debug>(buf_mut: &mut BytesMut, tracked_requests: &mut BTreeMap<i32, (ApiKey, i16)>) -> Result<Option<KafkaResponse>, KafkaProtocolError<E>> {
    if buf_mut.len() < 4 {
        eprintln!("Not enough data to read size");
        return Ok(None);
    }
    let sz = Buf::try_get_i32(&mut buf_mut.peek_bytes(0..4)).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading size"))?;
    if buf_mut.len() < sz as usize + 4 {
        eprintln!("Not enough data to read (expecting {} but have {})", sz, buf_mut.len() + 4);
        return Ok(None);
    }
    eprintln!("Size is: {}", sz);
    let _sz = Buf::try_get_i32(buf_mut).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading size"))?;
    let correlation_id = buf_mut.get(0..4);
    eprintln!("Reading correlation_id: {:?}", correlation_id);
    let correlation_id = Buf::try_get_i32(&mut buf_mut.peek_bytes(0..4)).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading correlation id"))?;
    eprintln!("Correlation id: {}", correlation_id);
    let (api, version) = tracked_requests.remove(&correlation_id).ok_or(KafkaProtocolError::UnhandledImplementation(
        "Encountered correlation id in response that isn't tracked on client",
    ))?;
    let header_version = api.response_header_version(version);
    eprintln!("Api {:?}, version {}, header version {}", api, version, header_version);
    let _header = ResponseHeader::decode(buf_mut, header_version).map_err(|_| KafkaProtocolError::UnhandledImplementation("Unable to deserialise response header"))?;
    eprintln!(
        "After reading header: {:?}",
        buf_mut.get(0..16).unwrap_or(buf_mut.as_ref()).iter().collect::<Vec<_>>()
    );
    eprintln!("Decoding response for API: {:?} with version {}", api, version);
    let resp: KafkaResponse = parse_response_internal(api, buf_mut, version)?;
    Ok(Some(resp))
}

macro_rules! handle_api_match {
    ($api:expr, $buf_mut:expr, $version:expr, $( $api_key:ident ),* ) => {
        match $api {
            $(
                ApiKey::$api_key => {
                    paste! {
                    [<$api_key Response>]::decode(&mut $buf_mut, $version)
                        .map(KafkaResponse::[<$api_key Response>])
                        .map_err(|e| {
                            eprintln!("Error decoding {} response: {:?}", stringify!($api_key), e);
                            KafkaProtocolError::UnhandledImplementation(concat!("Error decoding ", stringify!($api_key), " response"))
                        })?
                    }
                },
            )*
        }
    };
}

fn parse_response_internal<E: Debug>(api: ApiKey, mut buf_mut: &mut BytesMut, version: i16) -> Result<KafkaResponse, KafkaProtocolError<E>> {
    Ok(handle_api_match!(
        api,
        buf_mut,
        version,
        Produce,
        Fetch,
        ListOffsets,
        Metadata,
        LeaderAndIsr,
        StopReplica,
        UpdateMetadata,
        ControlledShutdown,
        OffsetCommit,
        OffsetFetch,
        FindCoordinator,
        JoinGroup,
        Heartbeat,
        LeaveGroup,
        SyncGroup,
        DescribeGroups,
        ListGroups,
        SaslHandshake,
        ApiVersions,
        CreateTopics,
        DeleteTopics,
        DeleteRecords,
        InitProducerId,
        OffsetForLeaderEpoch,
        AddPartitionsToTxn,
        AddOffsetsToTxn,
        EndTxn,
        WriteTxnMarkers,
        TxnOffsetCommit,
        DescribeAcls,
        CreateAcls,
        DeleteAcls,
        DescribeConfigs,
        AlterConfigs,
        AlterReplicaLogDirs,
        DescribeLogDirs,
        SaslAuthenticate,
        CreatePartitions,
        CreateDelegationToken,
        RenewDelegationToken,
        ExpireDelegationToken,
        DescribeDelegationToken,
        DeleteGroups,
        ElectLeaders,
        IncrementalAlterConfigs,
        AlterPartitionReassignments,
        ListPartitionReassignments,
        OffsetDelete,
        DescribeClientQuotas,
        AlterClientQuotas,
        DescribeUserScramCredentials,
        AlterUserScramCredentials,
        Vote,
        BeginQuorumEpoch,
        EndQuorumEpoch,
        DescribeQuorum,
        AlterPartition,
        UpdateFeatures,
        Envelope,
        FetchSnapshot,
        DescribeCluster,
        DescribeProducers,
        BrokerRegistration,
        BrokerHeartbeat,
        UnregisterBroker,
        DescribeTransactions,
        ListTransactions,
        AllocateProducerIds,
        ConsumerGroupHeartbeat,
        ConsumerGroupDescribe,
        ControllerRegistration,
        GetTelemetrySubscriptions,
        PushTelemetry,
        AssignReplicasToDirs,
        ListClientMetricsResources,
        DescribeTopicPartitions
    ))
}

#[cfg(test)]
mod test {
    use crate::client::layer::parse_response;
    use crate::data::{KafkaProtocolError, KafkaResponse};
    use bytes::BytesMut;
    use kafka_protocol::messages::metadata_response::{MetadataResponseBroker, MetadataResponsePartition, MetadataResponseTopic};
    use kafka_protocol::messages::{ApiKey, BrokerId, MetadataResponse, TopicName};
    use kafka_protocol::protocol::StrBytes;
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn test_metadata() {
        let metadata_response: &[u8] = &[
            0x00u8, 0x00, 0x00, 0x77, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x0a, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x68, 0x6f,
            0x73, 0x74, 0x00, 0x00, 0x71, 0xa4, 0x00, 0x00, 0x17, 0x71, 0x38, 0x4c, 0x30, 0x6a, 0x4d, 0x70, 0x52, 0x54, 0x41, 0x61, 0x5f, 0x4c, 0x4f, 0x49, 0x74, 0x4e,
            0x62, 0x4d, 0x56, 0x5a, 0x67, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x09, 0x6d, 0x79, 0x2d, 0x74, 0x6f, 0x70, 0x69, 0x63, 0xb1, 0xfa, 0x72, 0xa9, 0x70,
            0xf1, 0x4e, 0x91, 0xb2, 0x6f, 0xe3, 0xe2, 0x11, 0x1c, 0x72, 0x51, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut bytes_mut = BytesMut::from(metadata_response);
        let mut tracked_requests = BTreeMap::new();
        let correlation_id = 3;
        let api_version = 12;
        tracked_requests.insert(correlation_id, (ApiKey::Metadata, api_version));
        let resp: Result<_, KafkaProtocolError<()>> = parse_response(&mut bytes_mut, &mut tracked_requests).await;
        let resp = resp.unwrap();
        let resp = resp.unwrap();
        assert_eq!(
            resp,
            KafkaResponse::MetadataResponse(
                MetadataResponse::default()
                    .with_brokers(vec![MetadataResponseBroker::default()
                        .with_node_id(BrokerId(1))
                        .with_host(StrBytes::from("localhost"))
                        .with_port(29092)])
                    .with_cluster_id(Some(StrBytes::from("q8L0jMpRTAa_LOItNbMVZg")))
                    .with_controller_id(BrokerId(1))
                    .with_topics(vec![MetadataResponseTopic::default()
                        .with_name(Some(TopicName(StrBytes::from("my-topic"))))
                        .with_topic_id(uuid::Uuid::parse_str("b1fa72a9-70f1-4e91-b26f-e3e2111c7251").unwrap())
                        .with_partitions(vec![MetadataResponsePartition::default()
                            .with_leader_id(BrokerId(1))
                            .with_replica_nodes(vec![BrokerId(1)])
                            .with_leader_epoch(0)
                            .with_isr_nodes(vec![BrokerId(1)])])])
            )
        );
    }
}
