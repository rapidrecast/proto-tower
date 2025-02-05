use crate::data::{KafkaProtocolError, KafkaRequest, KafkaResponse};
use bytes::{Buf, BytesMut};
use kafka_protocol::messages::{
    AddOffsetsToTxnResponse, AddPartitionsToTxnResponse, AllocateProducerIdsResponse, AlterClientQuotasResponse, AlterConfigsResponse,
    AlterPartitionReassignmentsResponse, AlterPartitionResponse, AlterReplicaLogDirsResponse, AlterUserScramCredentialsResponse, ApiKey, ApiVersionsResponse,
    AssignReplicasToDirsResponse, BeginQuorumEpochResponse, BrokerHeartbeatResponse, BrokerRegistrationResponse, ConsumerGroupDescribeResponse,
    ConsumerGroupHeartbeatResponse, ControlledShutdownResponse, ControllerRegistrationResponse, CreateAclsResponse, CreateDelegationTokenResponse,
    CreatePartitionsResponse, CreateTopicsResponse, DeleteAclsResponse, DeleteGroupsResponse, DeleteRecordsResponse, DeleteTopicsResponse, DescribeAclsResponse,
    DescribeClientQuotasResponse, DescribeClusterResponse, DescribeConfigsResponse, DescribeDelegationTokenResponse, DescribeGroupsResponse, DescribeLogDirsResponse,
    DescribeProducersResponse, DescribeQuorumResponse, DescribeTopicPartitionsResponse, DescribeTransactionsResponse, DescribeUserScramCredentialsResponse,
    ElectLeadersResponse, EndQuorumEpochResponse, EndTxnResponse, EnvelopeResponse, ExpireDelegationTokenResponse, FetchResponse, FetchSnapshotResponse,
    FindCoordinatorResponse, GetTelemetrySubscriptionsResponse, HeartbeatResponse, IncrementalAlterConfigsResponse, InitProducerIdResponse, JoinGroupResponse,
    LeaderAndIsrResponse, LeaveGroupResponse, ListClientMetricsResourcesResponse, ListGroupsResponse, ListOffsetsResponse, ListPartitionReassignmentsResponse,
    ListTransactionsResponse, MetadataResponse, OffsetCommitResponse, OffsetDeleteResponse, OffsetFetchResponse, OffsetForLeaderEpochResponse, ProduceResponse,
    PushTelemetryResponse, RenewDelegationTokenResponse, SaslAuthenticateResponse, SaslHandshakeResponse, StopReplicaResponse, SyncGroupResponse,
    TxnOffsetCommitResponse, UnregisterBrokerResponse, UpdateFeaturesResponse, UpdateMetadataResponse, VoteResponse, WriteTxnMarkersResponse,
};
use kafka_protocol::protocol::buf::ByteBuf;
use kafka_protocol::protocol::Decodable;
use proto_tower_util::WriteTo;
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
pub struct ProtoKafkaClientLayer<Svc, E>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
{
    inner: Svc,
}

impl<Svc, E> ProtoKafkaClientLayer<Svc, E>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
{
    pub fn new(inner: Svc) -> Self {
        ProtoKafkaClientLayer { inner }
    }
}

impl<Svc, E, SvcFut> Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>)> for ProtoKafkaClientLayer<Svc, E>
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
        Box::pin(async move {
            // Create channel
            let (svc_read, mut write) = tokio::io::simplex(1024);
            let (mut read, svc_write) = tokio::io::simplex(1024);
            // Spawn handler
            let f = inner.call((svc_read, svc_write));
            let task = tokio::spawn(f);
            let mut buf_mut = BytesMut::new();
            while true {
                select!(
                    r = rx_chan.recv() => {
                        match r {
                            None => {
                                return Ok(())
                            }
                            Some(req) => {
                                req.write_to(&mut write).await?;
                            }
                        }
                    }
                    d = read.read_buf(&mut buf_mut) => {
                        match d {
                             Ok(sz) => {
                                if sz == 0 {
                                    eprintln!("Received size 0, breaking");
                                    break;
                                }
                                let sz = Buf::try_get_i32(&mut buf_mut.peek_bytes(0..4)).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading size"))?;
                                if buf_mut.len() < sz as usize + 4{
                                    eprintln!("Not enough data to read");
                                    continue;
                                }
                                let sz = Buf::try_get_i32(&mut buf_mut).map_err(|_| KafkaProtocolError::UnhandledImplementation("Error reading size"))?;
                                let api = Buf::try_get_i16(&mut buf_mut.peek_bytes(0..2)).map_err(|_ | KafkaProtocolError::UnhandledImplementation("Error reading API Key"))?;
                                let version = Buf::try_get_i16(&mut buf_mut.peek_bytes(2..4)).map_err(|_ | KafkaProtocolError::UnhandledImplementation("Error reading version"))?;
                                let api = ApiKey::try_from(api).map_err(|_| KafkaProtocolError::UnhandledImplementation("Invalid API Key"))?;
                                let resp: KafkaResponse = parse_response(api, &mut buf_mut, version)?;
                                match sx_chan.send(resp).await {
                                    Ok(_) => {}
                                    Err(_) => {
                                        return Err(KafkaProtocolError::UnhandledImplementation("Error sending response"));
                                    }
                                }
                            }
                            Err(a) => {
                                eprintln!("Error reading from stream: {:?}", a);
                                return Err(KafkaProtocolError::UnhandledImplementation("Error reading from stream"));
                            }
                        }
                    }
                );
                if task.is_finished() {
                    break;
                }
            }
            task.await
                .map_err(|_| KafkaProtocolError::InternalServiceClosed)?
                .map_err(|e| KafkaProtocolError::InternalServiceError(e))
        })
    }
}

fn parse_response<E: Debug>(api: ApiKey, mut buf_mut: &mut BytesMut, version: i16) -> Result<KafkaResponse, KafkaProtocolError<E>> {
    Ok(match api {
        ApiKey::Produce => ProduceResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ProduceResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::Fetch => FetchResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::FetchResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ListOffsets => ListOffsetsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ListOffsetsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::Metadata => MetadataResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::MetadataResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::LeaderAndIsr => LeaderAndIsrResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::LeaderAndIsrResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::StopReplica => StopReplicaResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::StopReplicaResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::UpdateMetadata => UpdateMetadataResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::UpdateMetadataResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ControlledShutdown => ControlledShutdownResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ControlledShutdownResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::OffsetCommit => OffsetCommitResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::OffsetCommitResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::OffsetFetch => OffsetFetchResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::OffsetFetchResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::FindCoordinator => FindCoordinatorResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::FindCoordinatorResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::JoinGroup => JoinGroupResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::JoinGroupResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::Heartbeat => HeartbeatResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::HeartbeatResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::LeaveGroup => LeaveGroupResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::LeaveGroupResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::SyncGroup => SyncGroupResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::SyncGroupResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeGroups => DescribeGroupsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeGroupsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ListGroups => ListGroupsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ListGroupsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::SaslHandshake => SaslHandshakeResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::SaslHandshakeResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ApiVersions => ApiVersionsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ApiVersionsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::CreateTopics => CreateTopicsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::CreateTopicsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DeleteTopics => DeleteTopicsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DeleteTopicsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DeleteRecords => DeleteRecordsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DeleteRecordsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::InitProducerId => InitProducerIdResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::InitProducerIdResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::OffsetForLeaderEpoch => OffsetForLeaderEpochResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::OffsetForLeaderEpochResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AddPartitionsToTxn => AddPartitionsToTxnResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AddPartitionsToTxnResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AddOffsetsToTxn => AddOffsetsToTxnResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AddOffsetsToTxnResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::EndTxn => EndTxnResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::EndTxnResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::WriteTxnMarkers => WriteTxnMarkersResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::WriteTxnMarkersResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::TxnOffsetCommit => TxnOffsetCommitResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::TxnOffsetCommitResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeAcls => DescribeAclsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeAclsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::CreateAcls => CreateAclsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::CreateAclsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DeleteAcls => DeleteAclsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DeleteAclsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeConfigs => DescribeConfigsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeConfigsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AlterConfigs => AlterConfigsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AlterConfigsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AlterReplicaLogDirs => AlterReplicaLogDirsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AlterReplicaLogDirsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeLogDirs => DescribeLogDirsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeLogDirsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::SaslAuthenticate => SaslAuthenticateResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::SaslAuthenticateResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::CreatePartitions => CreatePartitionsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::CreatePartitionsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::CreateDelegationToken => CreateDelegationTokenResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::CreateDelegationTokenResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::RenewDelegationToken => RenewDelegationTokenResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::RenewDelegationTokenResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ExpireDelegationToken => ExpireDelegationTokenResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ExpireDelegationTokenResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeDelegationToken => DescribeDelegationTokenResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeDelegationTokenResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DeleteGroups => DeleteGroupsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DeleteGroupsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ElectLeaders => ElectLeadersResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ElectLeadersResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::IncrementalAlterConfigs => IncrementalAlterConfigsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::IncrementalAlterConfigsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AlterPartitionReassignments => AlterPartitionReassignmentsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AlterPartitionReassignmentsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ListPartitionReassignments => ListPartitionReassignmentsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ListPartitionReassignmentsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::OffsetDelete => OffsetDeleteResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::OffsetDeleteResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeClientQuotas => DescribeClientQuotasResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeClientQuotasResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AlterClientQuotas => AlterClientQuotasResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AlterClientQuotasResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeUserScramCredentials => DescribeUserScramCredentialsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeUserScramCredentialsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AlterUserScramCredentials => AlterUserScramCredentialsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AlterUserScramCredentialsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::Vote => VoteResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::VoteResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::BeginQuorumEpoch => BeginQuorumEpochResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::BeginQuorumEpochResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::EndQuorumEpoch => EndQuorumEpochResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::EndQuorumEpochResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeQuorum => DescribeQuorumResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeQuorumResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AlterPartition => AlterPartitionResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AlterPartitionResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::UpdateFeatures => UpdateFeaturesResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::UpdateFeaturesResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::Envelope => EnvelopeResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::EnvelopeResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::FetchSnapshot => FetchSnapshotResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::FetchSnapshotResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeCluster => DescribeClusterResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeClusterResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeProducers => DescribeProducersResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeProducersResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::BrokerRegistration => BrokerRegistrationResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::BrokerRegistrationResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::BrokerHeartbeat => BrokerHeartbeatResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::BrokerHeartbeatResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::UnregisterBroker => UnregisterBrokerResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::UnregisterBrokerResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeTransactions => DescribeTransactionsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeTransactionsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ListTransactions => ListTransactionsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ListTransactionsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AllocateProducerIds => AllocateProducerIdsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AllocateProducerIdsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ConsumerGroupHeartbeat => ConsumerGroupHeartbeatResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ConsumerGroupHeartbeatResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ConsumerGroupDescribe => ConsumerGroupDescribeResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ConsumerGroupDescribeResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ControllerRegistration => ControllerRegistrationResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ControllerRegistrationResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::GetTelemetrySubscriptions => GetTelemetrySubscriptionsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::GetTelemetrySubscriptionsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::PushTelemetry => PushTelemetryResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::PushTelemetryResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::AssignReplicasToDirs => AssignReplicasToDirsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::AssignReplicasToDirsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::ListClientMetricsResources => ListClientMetricsResourcesResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::ListClientMetricsResourcesResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
        ApiKey::DescribeTopicPartitions => DescribeTopicPartitionsResponse::decode(&mut buf_mut, version)
            .map(KafkaResponse::DescribeTopicPartitionsResponse)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Error decoding response"))?,
    })
}
