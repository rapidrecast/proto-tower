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
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, ReadHalf, SimplexStream, WriteHalf};
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tower::Service;

#[derive(Clone)]
pub struct ProtoKafkaClientLayer<Svc, E, RNG>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
    RNG: rand::TryRngCore + Send + Clone + 'static,
{
    inner: Svc,
    config: KafkaProtoClientConfig,
    rng: RNG,
}

impl<Svc, E, RNG> ProtoKafkaClientLayer<Svc, E, RNG>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
    RNG: rand::TryRngCore + Send + Clone + 'static,
{
    pub fn new(inner: Svc, rng: RNG, config: KafkaProtoClientConfig) -> Self {
        ProtoKafkaClientLayer { inner, rng, config }
    }
}

impl<Svc, E, SvcFut, RNG> Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>)> for ProtoKafkaClientLayer<Svc, E, RNG>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<(), E>> + Send + 'static,
    E: Debug + Send + 'static,
    RNG: rand::TryRngCore + Send + Clone + 'static,
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
        let mut random = self.rng.clone();
        Box::pin(async move {
            // Tracked requests
            let tracked_requests = Arc::new(RwLock::new(BTreeMap::<i32, ApiKey>::new()));
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
                                let mut req_lock = tracked_requests.write().await;
                                let mut correlation_id: i32 = rand_i32(&mut random);
                                while req_lock.contains_key(&correlation_id) {
                                    correlation_id = rand_i32(&mut random);
                                }
                                req_lock.insert(correlation_id, req.api_key());
                                req.into_full(3, correlation_id, config.client_id.clone()).write_to(&mut write).await?;
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
                                let resp = parse_response(&mut buf_mut, &tracked_requests).await?;
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
                        return Err(KafkaProtocolError::UnhandledImplementation("Client Timeout"));
                    }
                );
                if task.is_finished() {
                    break;
                }
            }
            eprintln!("Client loop finished");
            task.await
                .map_err(|_| KafkaProtocolError::InternalServiceClosed)?
                .map_err(|e| KafkaProtocolError::InternalServiceError(e))
        })
    }
}

fn rand_i32<RNG: rand::TryRngCore>(rng: &mut RNG) -> i32 {
    let mut bytes = [0u8; 4];
    rng.try_fill_bytes(&mut bytes).unwrap();
    i32::from_be_bytes(bytes)
}

async fn parse_response<E: Debug>(buf_mut: &mut BytesMut, tracked_requests: &Arc<RwLock<BTreeMap<i32, ApiKey>>>) -> Result<Option<KafkaResponse>, KafkaProtocolError<E>> {
    let sz = Buf::try_get_i32(&mut buf_mut.peek_bytes(0..4)).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading size"))?;
    if buf_mut.len() < sz as usize + 4 {
        eprintln!("Not enough data to read (expecting {} but have {})", sz, buf_mut.len() + 4);
        return Ok(None);
    }
    let _sz = Buf::try_get_i32(buf_mut).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading size"))?;
    let version = Buf::try_get_i16(&mut buf_mut.peek_bytes(2..4)).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading version"))?;
    eprintln!("Reading response with version: {}", version);
    let header = ResponseHeader::decode(buf_mut, version).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading header"))?;
    let mut req_lock = tracked_requests.write().await;
    let api = req_lock.remove(&header.correlation_id).ok_or(KafkaProtocolError::UnhandledImplementation(
        "Encountered correlation id in response that isnt tracked on client",
    ))?;
    drop(req_lock);
    eprintln!("Decoding response for API: {:?}", api);
    let resp: KafkaResponse = parse_response_internal(api, buf_mut, version)?;
    Ok(Some(resp))
}

macro_rules! handle_api_match {
    ($api:expr, $buf_mut:expr, $version:expr, $( $api_key:ident ),* ) => {
        match $api {
            $(
                ApiKey::$api_key => paste! {
                    [<$api_key Response>]::decode(&mut $buf_mut, $version)
                        .map(Box::new)
                        .map(KafkaResponse::[<$api_key Response>])
                        .map_err(|_| KafkaProtocolError::UnhandledImplementation(concat!("Error decoding ", stringify!($api_key), " response")))?
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
