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
    ProduceRequest(i32, ProduceRequest),
    FetchRequest(i32, FetchRequest),
    ListOffsetsRequest(i32, ListOffsetsRequest),
    MetadataRequest(i32, MetadataRequest),
    LeaderAndIsrRequest(i32, LeaderAndIsrRequest),
    StopReplicaRequest(i32, StopReplicaRequest),
    UpdateMetadataRequest(i32, UpdateMetadataRequest),
    ControlledShutdownRequest(i32, ControlledShutdownRequest),
    OffsetCommitRequest(i32, OffsetCommitRequest),
    OffsetFetchRequest(i32, OffsetFetchRequest),
    FindCoordinatorRequest(i32, FindCoordinatorRequest),
    JoinGroupRequest(i32, JoinGroupRequest),
    HeartbeatRequest(i32, HeartbeatRequest),
    LeaveGroupRequest(i32, LeaveGroupRequest),
    SyncGroupRequest(i32, SyncGroupRequest),
    DescribeGroupsRequest(i32, DescribeGroupsRequest),
    ListGroupsRequest(i32, ListGroupsRequest),
    SaslHandshakeRequest(i32, SaslHandshakeRequest),
    ApiVersionsRequest(i32, ApiVersionsRequest),
    CreateTopicsRequest(i32, CreateTopicsRequest),
    DeleteTopicsRequest(i32, DeleteTopicsRequest),
    DeleteRecordsRequest(i32, DeleteRecordsRequest),
    InitProducerIdRequest(i32, InitProducerIdRequest),
    OffsetForLeaderEpochRequest(i32, OffsetForLeaderEpochRequest),
    AddPartitionsToTxnRequest(i32, AddPartitionsToTxnRequest),
    AddOffsetsToTxnRequest(i32, AddOffsetsToTxnRequest),
    EndTxnRequest(i32, EndTxnRequest),
    WriteTxnMarkersRequest(i32, WriteTxnMarkersRequest),
    TxnOffsetCommitRequest(i32, TxnOffsetCommitRequest),
    DescribeAclsRequest(i32, DescribeAclsRequest),
    CreateAclsRequest(i32, CreateAclsRequest),
    DeleteAclsRequest(i32, DeleteAclsRequest),
    DescribeConfigsRequest(i32, DescribeConfigsRequest),
    AlterConfigsRequest(i32, AlterConfigsRequest),
    AlterReplicaLogDirsRequest(i32, AlterReplicaLogDirsRequest),
    DescribeLogDirsRequest(i32, DescribeLogDirsRequest),
    SaslAuthenticateRequest(i32, SaslAuthenticateRequest),
    CreatePartitionsRequest(i32, CreatePartitionsRequest),
    CreateDelegationTokenRequest(i32, CreateDelegationTokenRequest),
    RenewDelegationTokenRequest(i32, RenewDelegationTokenRequest),
    ExpireDelegationTokenRequest(i32, ExpireDelegationTokenRequest),
    DescribeDelegationTokenRequest(i32, DescribeDelegationTokenRequest),
    DeleteGroupsRequest(i32, DeleteGroupsRequest),
    ElectLeadersRequest(i32, ElectLeadersRequest),
    IncrementalAlterConfigsRequest(i32, IncrementalAlterConfigsRequest),
    AlterPartitionReassignmentsRequest(i32, AlterPartitionReassignmentsRequest),
    ListPartitionReassignmentsRequest(i32, ListPartitionReassignmentsRequest),
    OffsetDeleteRequest(i32, OffsetDeleteRequest),
    DescribeClientQuotasRequest(i32, DescribeClientQuotasRequest),
    AlterClientQuotasRequest(i32, AlterClientQuotasRequest),
    DescribeUserScramCredentialsRequest(i32, DescribeUserScramCredentialsRequest),
    AlterUserScramCredentialsRequest(i32, AlterUserScramCredentialsRequest),
    VoteRequest(i32, VoteRequest),
    BeginQuorumEpochRequest(i32, BeginQuorumEpochRequest),
    EndQuorumEpochRequest(i32, EndQuorumEpochRequest),
    DescribeQuorumRequest(i32, DescribeQuorumRequest),
    AlterPartitionRequest(i32, AlterPartitionRequest),
    UpdateFeaturesRequest(i32, UpdateFeaturesRequest),
    EnvelopeRequest(i32, EnvelopeRequest),
    FetchSnapshotRequest(i32, FetchSnapshotRequest),
    DescribeClusterRequest(i32, DescribeClusterRequest),
    DescribeProducersRequest(i32, DescribeProducersRequest),
    BrokerRegistrationRequest(i32, BrokerRegistrationRequest),
    BrokerHeartbeatRequest(i32, BrokerHeartbeatRequest),
    UnregisterBrokerRequest(i32, UnregisterBrokerRequest),
    DescribeTransactionsRequest(i32, DescribeTransactionsRequest),
    ListTransactionsRequest(i32, ListTransactionsRequest),
    AllocateProducerIdsRequest(i32, AllocateProducerIdsRequest),
    ConsumerGroupHeartbeatRequest(i32, ConsumerGroupHeartbeatRequest),
    ConsumerGroupDescribeRequest(i32, ConsumerGroupDescribeRequest),
    ControllerRegistrationRequest(i32, ControllerRegistrationRequest),
    GetTelemetrySubscriptionsRequest(i32, GetTelemetrySubscriptionsRequest),
    PushTelemetryRequest(i32, PushTelemetryRequest),
    AssignReplicasToDirsRequest(i32, AssignReplicasToDirsRequest),
    ListClientMetricsResourcesRequest(i32, ListClientMetricsResourcesRequest),
    DescribeTopicPartitionsRequest(i32, DescribeTopicPartitionsRequest),
}

impl KafkaRequest {
    pub fn api_key(&self) -> ApiKey {
        match self {
            KafkaRequest::ProduceRequest(_, _) => ApiKey::Produce,
            KafkaRequest::FetchRequest(_, _) => ApiKey::Fetch,
            KafkaRequest::ListOffsetsRequest(_, _) => ApiKey::ListOffsets,
            KafkaRequest::MetadataRequest(_, _) => ApiKey::Metadata,
            KafkaRequest::LeaderAndIsrRequest(_, _) => ApiKey::LeaderAndIsr,
            KafkaRequest::StopReplicaRequest(_, _) => ApiKey::StopReplica,
            KafkaRequest::UpdateMetadataRequest(_, _) => ApiKey::UpdateMetadata,
            KafkaRequest::ControlledShutdownRequest(_, _) => ApiKey::ControlledShutdown,
            KafkaRequest::OffsetCommitRequest(_, _) => ApiKey::OffsetCommit,
            KafkaRequest::OffsetFetchRequest(_, _) => ApiKey::OffsetFetch,
            KafkaRequest::FindCoordinatorRequest(_, _) => ApiKey::FindCoordinator,
            KafkaRequest::JoinGroupRequest(_, _) => ApiKey::JoinGroup,
            KafkaRequest::HeartbeatRequest(_, _) => ApiKey::Heartbeat,
            KafkaRequest::LeaveGroupRequest(_, _) => ApiKey::LeaveGroup,
            KafkaRequest::SyncGroupRequest(_, _) => ApiKey::SyncGroup,
            KafkaRequest::DescribeGroupsRequest(_, _) => ApiKey::DescribeGroups,
            KafkaRequest::ListGroupsRequest(_, _) => ApiKey::ListGroups,
            KafkaRequest::SaslHandshakeRequest(_, _) => ApiKey::SaslHandshake,
            KafkaRequest::ApiVersionsRequest(_, _) => ApiKey::ApiVersions,
            KafkaRequest::CreateTopicsRequest(_, _) => ApiKey::CreateTopics,
            KafkaRequest::DeleteTopicsRequest(_, _) => ApiKey::DeleteTopics,
            KafkaRequest::DeleteRecordsRequest(_, _) => ApiKey::DeleteRecords,
            KafkaRequest::InitProducerIdRequest(_, _) => ApiKey::InitProducerId,
            KafkaRequest::OffsetForLeaderEpochRequest(_, _) => ApiKey::OffsetForLeaderEpoch,
            KafkaRequest::AddPartitionsToTxnRequest(_, _) => ApiKey::AddPartitionsToTxn,
            KafkaRequest::AddOffsetsToTxnRequest(_, _) => ApiKey::AddOffsetsToTxn,
            KafkaRequest::EndTxnRequest(_, _) => ApiKey::EndTxn,
            KafkaRequest::WriteTxnMarkersRequest(_, _) => ApiKey::WriteTxnMarkers,
            KafkaRequest::TxnOffsetCommitRequest(_, _) => ApiKey::TxnOffsetCommit,
            KafkaRequest::DescribeAclsRequest(_, _) => ApiKey::DescribeAcls,
            KafkaRequest::CreateAclsRequest(_, _) => ApiKey::CreateAcls,
            KafkaRequest::DeleteAclsRequest(_, _) => ApiKey::DeleteAcls,
            KafkaRequest::DescribeConfigsRequest(_, _) => ApiKey::DescribeConfigs,
            KafkaRequest::AlterConfigsRequest(_, _) => ApiKey::AlterConfigs,
            KafkaRequest::AlterReplicaLogDirsRequest(_, _) => ApiKey::AlterReplicaLogDirs,
            KafkaRequest::DescribeLogDirsRequest(_, _) => ApiKey::DescribeLogDirs,
            KafkaRequest::SaslAuthenticateRequest(_, _) => ApiKey::SaslAuthenticate,
            KafkaRequest::CreatePartitionsRequest(_, _) => ApiKey::CreatePartitions,
            KafkaRequest::CreateDelegationTokenRequest(_, _) => ApiKey::CreateDelegationToken,
            KafkaRequest::RenewDelegationTokenRequest(_, _) => ApiKey::RenewDelegationToken,
            KafkaRequest::ExpireDelegationTokenRequest(_, _) => ApiKey::ExpireDelegationToken,
            KafkaRequest::DescribeDelegationTokenRequest(_, _) => ApiKey::DescribeDelegationToken,
            KafkaRequest::DeleteGroupsRequest(_, _) => ApiKey::DeleteGroups,
            KafkaRequest::ElectLeadersRequest(_, _) => ApiKey::ElectLeaders,
            KafkaRequest::IncrementalAlterConfigsRequest(_, _) => ApiKey::IncrementalAlterConfigs,
            KafkaRequest::AlterPartitionReassignmentsRequest(_, _) => ApiKey::AlterPartitionReassignments,
            KafkaRequest::ListPartitionReassignmentsRequest(_, _) => ApiKey::ListPartitionReassignments,
            KafkaRequest::OffsetDeleteRequest(_, _) => ApiKey::OffsetDelete,
            KafkaRequest::DescribeClientQuotasRequest(_, _) => ApiKey::DescribeClientQuotas,
            KafkaRequest::AlterClientQuotasRequest(_, _) => ApiKey::AlterClientQuotas,
            KafkaRequest::DescribeUserScramCredentialsRequest(_, _) => ApiKey::DescribeUserScramCredentials,
            KafkaRequest::AlterUserScramCredentialsRequest(_, _) => ApiKey::AlterUserScramCredentials,
            KafkaRequest::VoteRequest(_, _) => ApiKey::Vote,
            KafkaRequest::BeginQuorumEpochRequest(_, _) => ApiKey::BeginQuorumEpoch,
            KafkaRequest::EndQuorumEpochRequest(_, _) => ApiKey::EndQuorumEpoch,
            KafkaRequest::DescribeQuorumRequest(_, _) => ApiKey::DescribeQuorum,
            KafkaRequest::AlterPartitionRequest(_, _) => ApiKey::AlterPartition,
            KafkaRequest::UpdateFeaturesRequest(_, _) => ApiKey::UpdateFeatures,
            KafkaRequest::EnvelopeRequest(_, _) => ApiKey::Envelope,
            KafkaRequest::FetchSnapshotRequest(_, _) => ApiKey::FetchSnapshot,
            KafkaRequest::DescribeClusterRequest(_, _) => ApiKey::DescribeCluster,
            KafkaRequest::DescribeProducersRequest(_, _) => ApiKey::DescribeProducers,
            KafkaRequest::BrokerRegistrationRequest(_, _) => ApiKey::BrokerRegistration,
            KafkaRequest::BrokerHeartbeatRequest(_, _) => ApiKey::BrokerHeartbeat,
            KafkaRequest::UnregisterBrokerRequest(_, _) => ApiKey::UnregisterBroker,
            KafkaRequest::DescribeTransactionsRequest(_, _) => ApiKey::DescribeTransactions,
            KafkaRequest::ListTransactionsRequest(_, _) => ApiKey::ListTransactions,
            KafkaRequest::AllocateProducerIdsRequest(_, _) => ApiKey::AllocateProducerIds,
            KafkaRequest::ConsumerGroupHeartbeatRequest(_, _) => ApiKey::ConsumerGroupHeartbeat,
            KafkaRequest::ConsumerGroupDescribeRequest(_, _) => ApiKey::ConsumerGroupDescribe,
            KafkaRequest::ControllerRegistrationRequest(_, _) => ApiKey::ControllerRegistration,
            KafkaRequest::GetTelemetrySubscriptionsRequest(_, _) => ApiKey::GetTelemetrySubscriptions,
            KafkaRequest::PushTelemetryRequest(_, _) => ApiKey::PushTelemetry,
            KafkaRequest::AssignReplicasToDirsRequest(_, _) => ApiKey::AssignReplicasToDirs,
            KafkaRequest::ListClientMetricsResourcesRequest(_, _) => ApiKey::ListClientMetricsResources,
            KafkaRequest::DescribeTopicPartitionsRequest(_, _) => ApiKey::DescribeTopicPartitions,
        }
    }
}

impl KafkaRequest {
    pub(crate) fn into_full(self, api_version: i16, client_id: Option<String>) -> FullKafkaRequest {
        FullKafkaRequest {
            api_version,
            message: self,
            client_id,
        }
    }
}

pub struct FullKafkaRequest {
    pub api_version: i16,
    pub message: KafkaRequest,
    pub client_id: Option<String>,
}

#[async_trait]
impl<Writer: tokio::io::AsyncWrite + Send + Unpin + 'static, E: Debug> proto_tower_util::WriteTo<Writer, KafkaProtocolError<E>> for FullKafkaRequest {
    async fn write_to(&self, writer: &mut Writer) -> Result<(), KafkaProtocolError<E>> {
        let version = self.api_version;
        let client_id = self.client_id.clone();
        match &self.message {
            KafkaRequest::ProduceRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::FetchRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ListOffsetsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::MetadataRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::LeaderAndIsrRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::StopReplicaRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::UpdateMetadataRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ControlledShutdownRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::OffsetCommitRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::OffsetFetchRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::FindCoordinatorRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::JoinGroupRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::HeartbeatRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::LeaveGroupRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::SyncGroupRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeGroupsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ListGroupsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::SaslHandshakeRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ApiVersionsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::CreateTopicsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DeleteTopicsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DeleteRecordsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::InitProducerIdRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::OffsetForLeaderEpochRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AddPartitionsToTxnRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AddOffsetsToTxnRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::EndTxnRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::WriteTxnMarkersRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::TxnOffsetCommitRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeAclsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::CreateAclsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DeleteAclsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeConfigsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AlterConfigsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AlterReplicaLogDirsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeLogDirsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::SaslAuthenticateRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::CreatePartitionsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::CreateDelegationTokenRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::RenewDelegationTokenRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ExpireDelegationTokenRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeDelegationTokenRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DeleteGroupsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ElectLeadersRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::IncrementalAlterConfigsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AlterPartitionReassignmentsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ListPartitionReassignmentsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::OffsetDeleteRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeClientQuotasRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AlterClientQuotasRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeUserScramCredentialsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AlterUserScramCredentialsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::VoteRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::BeginQuorumEpochRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::EndQuorumEpochRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeQuorumRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AlterPartitionRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::UpdateFeaturesRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::EnvelopeRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::FetchSnapshotRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeClusterRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeProducersRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::BrokerRegistrationRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::BrokerHeartbeatRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::UnregisterBrokerRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeTransactionsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ListTransactionsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AllocateProducerIdsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ConsumerGroupHeartbeatRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ConsumerGroupDescribeRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ControllerRegistrationRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::GetTelemetrySubscriptionsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::PushTelemetryRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::AssignReplicasToDirsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::ListClientMetricsResourcesRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
            }
            KafkaRequest::DescribeTopicPartitionsRequest(correlation_id, inner) => {
                encode_and_write_request!(inner, writer, version, correlation_id, client_id)
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
        let api_request = KafkaRequest::ApiVersionsRequest(
            1,
            ApiVersionsRequest::default()
                .with_client_software_name(StrBytes::from("librdkafka"))
                .with_client_software_version(StrBytes::from("2.3.0")),
        )
        .into_full(3, Some("rdkafka".to_string()));
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
