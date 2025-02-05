use crate::data::KafkaProtocolError;
use crate::encode_and_write_request;
use async_trait::async_trait;
use bytes::BytesMut;
use kafka_protocol::messages::{
    AddOffsetsToTxnRequest, AddPartitionsToTxnRequest, AllocateProducerIdsRequest, AlterClientQuotasRequest, AlterConfigsRequest, AlterPartitionReassignmentsRequest,
    AlterPartitionRequest, AlterReplicaLogDirsRequest, AlterUserScramCredentialsRequest, ApiKey, ApiVersionsRequest, AssignReplicasToDirsRequest,
    BeginQuorumEpochRequest, BrokerHeartbeatRequest, BrokerRegistrationRequest, ConsumerGroupDescribeRequest, ConsumerGroupHeartbeatRequest, ControlledShutdownRequest,
    ControllerRegistrationRequest, CreateAclsRequest, CreateDelegationTokenRequest, CreatePartitionsRequest, CreateTopicsRequest, DeleteAclsRequest, DeleteGroupsRequest,
    DeleteRecordsRequest, DeleteTopicsRequest, DescribeAclsRequest, DescribeClientQuotasRequest, DescribeClusterRequest, DescribeConfigsRequest,
    DescribeDelegationTokenRequest, DescribeGroupsRequest, DescribeLogDirsRequest, DescribeProducersRequest, DescribeQuorumRequest, DescribeTopicPartitionsRequest,
    DescribeTransactionsRequest, DescribeUserScramCredentialsRequest, ElectLeadersRequest, EndQuorumEpochRequest, EndTxnRequest, EnvelopeRequest,
    ExpireDelegationTokenRequest, FetchRequest, FetchSnapshotRequest, FindCoordinatorRequest, GetTelemetrySubscriptionsRequest, HeartbeatRequest,
    IncrementalAlterConfigsRequest, InitProducerIdRequest, JoinGroupRequest, LeaderAndIsrRequest, LeaveGroupRequest, ListClientMetricsResourcesRequest,
    ListGroupsRequest, ListOffsetsRequest, ListPartitionReassignmentsRequest, ListTransactionsRequest, MetadataRequest, OffsetCommitRequest, OffsetDeleteRequest,
    OffsetFetchRequest, OffsetForLeaderEpochRequest, ProduceRequest, PushTelemetryRequest, RenewDelegationTokenRequest, RequestHeader, SaslAuthenticateRequest,
    SaslHandshakeRequest, StopReplicaRequest, SyncGroupRequest, TxnOffsetCommitRequest, UnregisterBrokerRequest, UpdateFeaturesRequest, UpdateMetadataRequest,
    VoteRequest, WriteTxnMarkersRequest,
};
use kafka_protocol::protocol::{Encodable, Request};
use std::fmt::Debug;
use tokio::io::AsyncWriteExt;

#[derive(Debug, PartialEq, Clone)]
pub enum KafkaRequest {
    ProduceRequest(Box<ProduceRequest>),
    FetchRequest(Box<FetchRequest>),
    ListOffsetsRequest(Box<ListOffsetsRequest>),
    MetadataRequest(Box<MetadataRequest>),
    LeaderAndIsrRequest(Box<LeaderAndIsrRequest>),
    StopReplicaRequest(Box<StopReplicaRequest>),
    UpdateMetadataRequest(Box<UpdateMetadataRequest>),
    ControlledShutdownRequest(Box<ControlledShutdownRequest>),
    OffsetCommitRequest(Box<OffsetCommitRequest>),
    OffsetFetchRequest(Box<OffsetFetchRequest>),
    FindCoordinatorRequest(Box<FindCoordinatorRequest>),
    JoinGroupRequest(Box<JoinGroupRequest>),
    HeartbeatRequest(Box<HeartbeatRequest>),
    LeaveGroupRequest(Box<LeaveGroupRequest>),
    SyncGroupRequest(Box<SyncGroupRequest>),
    DescribeGroupsRequest(Box<DescribeGroupsRequest>),
    ListGroupsRequest(Box<ListGroupsRequest>),
    SaslHandshakeRequest(Box<SaslHandshakeRequest>),
    ApiVersionsRequest(Box<ApiVersionsRequest>),
    CreateTopicsRequest(Box<CreateTopicsRequest>),
    DeleteTopicsRequest(Box<DeleteTopicsRequest>),
    DeleteRecordsRequest(Box<DeleteRecordsRequest>),
    InitProducerIdRequest(Box<InitProducerIdRequest>),
    OffsetForLeaderEpochRequest(Box<OffsetForLeaderEpochRequest>),
    AddPartitionsToTxnRequest(Box<AddPartitionsToTxnRequest>),
    AddOffsetsToTxnRequest(Box<AddOffsetsToTxnRequest>),
    EndTxnRequest(Box<EndTxnRequest>),
    WriteTxnMarkersRequest(Box<WriteTxnMarkersRequest>),
    TxnOffsetCommitRequest(Box<TxnOffsetCommitRequest>),
    DescribeAclsRequest(Box<DescribeAclsRequest>),
    CreateAclsRequest(Box<CreateAclsRequest>),
    DeleteAclsRequest(Box<DeleteAclsRequest>),
    DescribeConfigsRequest(Box<DescribeConfigsRequest>),
    AlterConfigsRequest(Box<AlterConfigsRequest>),
    AlterReplicaLogDirsRequest(Box<AlterReplicaLogDirsRequest>),
    DescribeLogDirsRequest(Box<DescribeLogDirsRequest>),
    SaslAuthenticateRequest(Box<SaslAuthenticateRequest>),
    CreatePartitionsRequest(Box<CreatePartitionsRequest>),
    CreateDelegationTokenRequest(Box<CreateDelegationTokenRequest>),
    RenewDelegationTokenRequest(Box<RenewDelegationTokenRequest>),
    ExpireDelegationTokenRequest(Box<ExpireDelegationTokenRequest>),
    DescribeDelegationTokenRequest(Box<DescribeDelegationTokenRequest>),
    DeleteGroupsRequest(Box<DeleteGroupsRequest>),
    ElectLeadersRequest(Box<ElectLeadersRequest>),
    IncrementalAlterConfigsRequest(Box<IncrementalAlterConfigsRequest>),
    AlterPartitionReassignmentsRequest(Box<AlterPartitionReassignmentsRequest>),
    ListPartitionReassignmentsRequest(Box<ListPartitionReassignmentsRequest>),
    OffsetDeleteRequest(Box<OffsetDeleteRequest>),
    DescribeClientQuotasRequest(Box<DescribeClientQuotasRequest>),
    AlterClientQuotasRequest(Box<AlterClientQuotasRequest>),
    DescribeUserScramCredentialsRequest(Box<DescribeUserScramCredentialsRequest>),
    AlterUserScramCredentialsRequest(Box<AlterUserScramCredentialsRequest>),
    VoteRequest(Box<VoteRequest>),
    BeginQuorumEpochRequest(Box<BeginQuorumEpochRequest>),
    EndQuorumEpochRequest(Box<EndQuorumEpochRequest>),
    DescribeQuorumRequest(Box<DescribeQuorumRequest>),
    AlterPartitionRequest(Box<AlterPartitionRequest>),
    UpdateFeaturesRequest(Box<UpdateFeaturesRequest>),
    EnvelopeRequest(Box<EnvelopeRequest>),
    FetchSnapshotRequest(Box<FetchSnapshotRequest>),
    DescribeClusterRequest(Box<DescribeClusterRequest>),
    DescribeProducersRequest(Box<DescribeProducersRequest>),
    BrokerRegistrationRequest(Box<BrokerRegistrationRequest>),
    BrokerHeartbeatRequest(Box<BrokerHeartbeatRequest>),
    UnregisterBrokerRequest(Box<UnregisterBrokerRequest>),
    DescribeTransactionsRequest(Box<DescribeTransactionsRequest>),
    ListTransactionsRequest(Box<ListTransactionsRequest>),
    AllocateProducerIdsRequest(Box<AllocateProducerIdsRequest>),
    ConsumerGroupHeartbeatRequest(Box<ConsumerGroupHeartbeatRequest>),
    ConsumerGroupDescribeRequest(Box<ConsumerGroupDescribeRequest>),
    ControllerRegistrationRequest(Box<ControllerRegistrationRequest>),
    GetTelemetrySubscriptionsRequest(Box<GetTelemetrySubscriptionsRequest>),
    PushTelemetryRequest(Box<PushTelemetryRequest>),
    AssignReplicasToDirsRequest(Box<AssignReplicasToDirsRequest>),
    ListClientMetricsResourcesRequest(Box<ListClientMetricsResourcesRequest>),
    DescribeTopicPartitionsRequest(Box<DescribeTopicPartitionsRequest>),
}

impl KafkaRequest {
    pub fn api_key(&self) -> ApiKey {
        match self {
            KafkaRequest::ProduceRequest(_) => ApiKey::Produce,
            KafkaRequest::FetchRequest(_) => ApiKey::Fetch,
            KafkaRequest::ListOffsetsRequest(_) => ApiKey::ListOffsets,
            KafkaRequest::MetadataRequest(_) => ApiKey::Metadata,
            KafkaRequest::LeaderAndIsrRequest(_) => ApiKey::LeaderAndIsr,
            KafkaRequest::StopReplicaRequest(_) => ApiKey::StopReplica,
            KafkaRequest::UpdateMetadataRequest(_) => ApiKey::UpdateMetadata,
            KafkaRequest::ControlledShutdownRequest(_) => ApiKey::ControlledShutdown,
            KafkaRequest::OffsetCommitRequest(_) => ApiKey::OffsetCommit,
            KafkaRequest::OffsetFetchRequest(_) => ApiKey::OffsetFetch,
            KafkaRequest::FindCoordinatorRequest(_) => ApiKey::FindCoordinator,
            KafkaRequest::JoinGroupRequest(_) => ApiKey::JoinGroup,
            KafkaRequest::HeartbeatRequest(_) => ApiKey::Heartbeat,
            KafkaRequest::LeaveGroupRequest(_) => ApiKey::LeaveGroup,
            KafkaRequest::SyncGroupRequest(_) => ApiKey::SyncGroup,
            KafkaRequest::DescribeGroupsRequest(_) => ApiKey::DescribeGroups,
            KafkaRequest::ListGroupsRequest(_) => ApiKey::ListGroups,
            KafkaRequest::SaslHandshakeRequest(_) => ApiKey::SaslHandshake,
            KafkaRequest::ApiVersionsRequest(_) => ApiKey::ApiVersions,
            KafkaRequest::CreateTopicsRequest(_) => ApiKey::CreateTopics,
            KafkaRequest::DeleteTopicsRequest(_) => ApiKey::DeleteTopics,
            KafkaRequest::DeleteRecordsRequest(_) => ApiKey::DeleteRecords,
            KafkaRequest::InitProducerIdRequest(_) => ApiKey::InitProducerId,
            KafkaRequest::OffsetForLeaderEpochRequest(_) => ApiKey::OffsetForLeaderEpoch,
            KafkaRequest::AddPartitionsToTxnRequest(_) => ApiKey::AddPartitionsToTxn,
            KafkaRequest::AddOffsetsToTxnRequest(_) => ApiKey::AddOffsetsToTxn,
            KafkaRequest::EndTxnRequest(_) => ApiKey::EndTxn,
            KafkaRequest::WriteTxnMarkersRequest(_) => ApiKey::WriteTxnMarkers,
            KafkaRequest::TxnOffsetCommitRequest(_) => ApiKey::TxnOffsetCommit,
            KafkaRequest::DescribeAclsRequest(_) => ApiKey::DescribeAcls,
            KafkaRequest::CreateAclsRequest(_) => ApiKey::CreateAcls,
            KafkaRequest::DeleteAclsRequest(_) => ApiKey::DeleteAcls,
            KafkaRequest::DescribeConfigsRequest(_) => ApiKey::DescribeConfigs,
            KafkaRequest::AlterConfigsRequest(_) => ApiKey::AlterConfigs,
            KafkaRequest::AlterReplicaLogDirsRequest(_) => ApiKey::AlterReplicaLogDirs,
            KafkaRequest::DescribeLogDirsRequest(_) => ApiKey::DescribeLogDirs,
            KafkaRequest::SaslAuthenticateRequest(_) => ApiKey::SaslAuthenticate,
            KafkaRequest::CreatePartitionsRequest(_) => ApiKey::CreatePartitions,
            KafkaRequest::CreateDelegationTokenRequest(_) => ApiKey::CreateDelegationToken,
            KafkaRequest::RenewDelegationTokenRequest(_) => ApiKey::RenewDelegationToken,
            KafkaRequest::ExpireDelegationTokenRequest(_) => ApiKey::ExpireDelegationToken,
            KafkaRequest::DescribeDelegationTokenRequest(_) => ApiKey::DescribeDelegationToken,
            KafkaRequest::DeleteGroupsRequest(_) => ApiKey::DeleteGroups,
            KafkaRequest::ElectLeadersRequest(_) => ApiKey::ElectLeaders,
            KafkaRequest::IncrementalAlterConfigsRequest(_) => ApiKey::IncrementalAlterConfigs,
            KafkaRequest::AlterPartitionReassignmentsRequest(_) => ApiKey::AlterPartitionReassignments,
            KafkaRequest::ListPartitionReassignmentsRequest(_) => ApiKey::ListPartitionReassignments,
            KafkaRequest::OffsetDeleteRequest(_) => ApiKey::OffsetDelete,
            KafkaRequest::DescribeClientQuotasRequest(_) => ApiKey::DescribeClientQuotas,
            KafkaRequest::AlterClientQuotasRequest(_) => ApiKey::AlterClientQuotas,
            KafkaRequest::DescribeUserScramCredentialsRequest(_) => ApiKey::DescribeUserScramCredentials,
            KafkaRequest::AlterUserScramCredentialsRequest(_) => ApiKey::AlterUserScramCredentials,
            KafkaRequest::VoteRequest(_) => ApiKey::Vote,
            KafkaRequest::BeginQuorumEpochRequest(_) => ApiKey::BeginQuorumEpoch,
            KafkaRequest::EndQuorumEpochRequest(_) => ApiKey::EndQuorumEpoch,
            KafkaRequest::DescribeQuorumRequest(_) => ApiKey::DescribeQuorum,
            KafkaRequest::AlterPartitionRequest(_) => ApiKey::AlterPartition,
            KafkaRequest::UpdateFeaturesRequest(_) => ApiKey::UpdateFeatures,
            KafkaRequest::EnvelopeRequest(_) => ApiKey::Envelope,
            KafkaRequest::FetchSnapshotRequest(_) => ApiKey::FetchSnapshot,
            KafkaRequest::DescribeClusterRequest(_) => ApiKey::DescribeCluster,
            KafkaRequest::DescribeProducersRequest(_) => ApiKey::DescribeProducers,
            KafkaRequest::BrokerRegistrationRequest(_) => ApiKey::BrokerRegistration,
            KafkaRequest::BrokerHeartbeatRequest(_) => ApiKey::BrokerHeartbeat,
            KafkaRequest::UnregisterBrokerRequest(_) => ApiKey::UnregisterBroker,
            KafkaRequest::DescribeTransactionsRequest(_) => ApiKey::DescribeTransactions,
            KafkaRequest::ListTransactionsRequest(_) => ApiKey::ListTransactions,
            KafkaRequest::AllocateProducerIdsRequest(_) => ApiKey::AllocateProducerIds,
            KafkaRequest::ConsumerGroupHeartbeatRequest(_) => ApiKey::ConsumerGroupHeartbeat,
            KafkaRequest::ConsumerGroupDescribeRequest(_) => ApiKey::ConsumerGroupDescribe,
            KafkaRequest::ControllerRegistrationRequest(_) => ApiKey::ControllerRegistration,
            KafkaRequest::GetTelemetrySubscriptionsRequest(_) => ApiKey::GetTelemetrySubscriptions,
            KafkaRequest::PushTelemetryRequest(_) => ApiKey::PushTelemetry,
            KafkaRequest::AssignReplicasToDirsRequest(_) => ApiKey::AssignReplicasToDirs,
            KafkaRequest::ListClientMetricsResourcesRequest(_) => ApiKey::ListClientMetricsResources,
            KafkaRequest::DescribeTopicPartitionsRequest(_) => ApiKey::DescribeTopicPartitions,
        }
    }
}

impl KafkaRequest {
    pub(crate) fn into_full(self, api_version: i16, correlation_id: i32, client_id: Option<String>) -> FullKafkaRequest {
        FullKafkaRequest {
            api_version,
            message: self,
            correlation_id,
            client_id,
        }
    }
}

pub struct FullKafkaRequest {
    pub api_version: i16,
    pub correlation_id: i32,
    pub message: KafkaRequest,
    pub client_id: Option<String>,
}

#[async_trait]
impl<Writer: tokio::io::AsyncWrite + Send + Unpin + 'static, E: Debug> proto_tower_util::WriteTo<Writer, KafkaProtocolError<E>> for FullKafkaRequest {
    async fn write_to(&self, writer: &mut Writer) -> Result<(), KafkaProtocolError<E>> {
        let version = self.api_version;
        let mut buff_mut = BytesMut::new();
        let correlation_id = self.correlation_id;
        let client_id = self.client_id.clone();
        match &self.message {
            KafkaRequest::ProduceRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::FetchRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ListOffsetsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::MetadataRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::LeaderAndIsrRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::StopReplicaRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::UpdateMetadataRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ControlledShutdownRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::OffsetCommitRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::OffsetFetchRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::FindCoordinatorRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::JoinGroupRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::HeartbeatRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::LeaveGroupRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::SyncGroupRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeGroupsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ListGroupsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::SaslHandshakeRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ApiVersionsRequest(inner) => {
                let key = get_api_key(inner.as_ref());
                let header = RequestHeader::default()
                    .with_request_api_key(key)
                    .with_request_api_version(version)
                    .with_correlation_id(correlation_id)
                    .with_client_id(client_id.map(|x| x.into()));

                header
                    .encode(&mut buff_mut, version)
                    .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response header encode failure"))?;

                inner
                    .encode(&mut buff_mut, version)
                    .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response body encode failure"))?;

                let sz = buff_mut.len() as i32;
                let sz_bytes: [u8; 4] = sz.to_be_bytes();
                writer
                    .write_all(&sz_bytes)
                    .await
                    .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response size write failure"))?;
                writer
                    .write_all(&buff_mut)
                    .await
                    .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response payload write failure"))?;
                Ok(())
            }
            KafkaRequest::CreateTopicsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DeleteTopicsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DeleteRecordsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::InitProducerIdRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::OffsetForLeaderEpochRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AddPartitionsToTxnRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AddOffsetsToTxnRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::EndTxnRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::WriteTxnMarkersRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::TxnOffsetCommitRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeAclsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::CreateAclsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DeleteAclsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeConfigsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AlterConfigsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AlterReplicaLogDirsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeLogDirsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::SaslAuthenticateRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::CreatePartitionsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::CreateDelegationTokenRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::RenewDelegationTokenRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ExpireDelegationTokenRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeDelegationTokenRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DeleteGroupsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ElectLeadersRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::IncrementalAlterConfigsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AlterPartitionReassignmentsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ListPartitionReassignmentsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::OffsetDeleteRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeClientQuotasRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AlterClientQuotasRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeUserScramCredentialsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AlterUserScramCredentialsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::VoteRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::BeginQuorumEpochRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::EndQuorumEpochRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeQuorumRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AlterPartitionRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::UpdateFeaturesRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::EnvelopeRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::FetchSnapshotRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeClusterRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeProducersRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::BrokerRegistrationRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::BrokerHeartbeatRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::UnregisterBrokerRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeTransactionsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ListTransactionsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AllocateProducerIdsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ConsumerGroupHeartbeatRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ConsumerGroupDescribeRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ControllerRegistrationRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::GetTelemetrySubscriptionsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::PushTelemetryRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::AssignReplicasToDirsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::ListClientMetricsResourcesRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
            KafkaRequest::DescribeTopicPartitionsRequest(inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id)
            }
        }
    }
}

fn get_api_key<R: Request>(_inner: &R) -> i16 {
    R::KEY
}

#[cfg(test)]
mod test {
    use crate::data::{KafkaProtocolError, KafkaRequest};
    use kafka_protocol::messages::ApiVersionsRequest;
    use kafka_protocol::protocol::StrBytes;
    use proto_tower_util::WriteTo;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn test_api_request() {
        let api_request = KafkaRequest::ApiVersionsRequest(Box::new(
            ApiVersionsRequest::default()
                .with_client_software_name(StrBytes::from("librdkafka"))
                .with_client_software_version(StrBytes::from("2.3.0")),
        ))
        .into_full(3, 1, Some("rdkafka".to_string()));
        let (mut read, mut write) = tokio::io::duplex(1024);
        let res: Result<(), KafkaProtocolError<()>> = api_request.write_to(&mut write).await;
        res.unwrap();
        let mut buff = [0u8; 1024];
        let r = read.read(&mut buff).await.unwrap();
        let real_buff = &buff[..r];

        let expected: Vec<u8> = [
            "\x00\x00\x00\x24".as_bytes(),
            "\x00\x12".as_bytes(),
            "\x00\x03".as_bytes(),
            "\x00\x00\x00\x01".as_bytes(),
            "\x00\x07rdkafka".as_bytes(),
            "\x00\x0blibrdkafka\x062.3.0\x00".as_bytes(),
        ]
        .into_iter()
        .flatten()
        .map(|x| *x)
        .collect();

        assert_eq!(real_buff, expected);
    }
}
