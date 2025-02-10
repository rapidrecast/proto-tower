use crate::data::KafkaProtocolError;
use crate::encode_and_write_response;
use async_trait::async_trait;
use bytes::BytesMut;
use kafka_protocol::messages::*;
use kafka_protocol::protocol::Encodable;
use std::fmt::Debug;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, PartialEq)]
pub enum KafkaResponse {
    ProduceResponse(i32, ProduceResponse),
    FetchResponse(i32, FetchResponse),
    ListOffsetsResponse(i32, ListOffsetsResponse),
    MetadataResponse(i32, MetadataResponse),
    LeaderAndIsrResponse(i32, LeaderAndIsrResponse),
    StopReplicaResponse(i32, StopReplicaResponse),
    UpdateMetadataResponse(i32, UpdateMetadataResponse),
    ControlledShutdownResponse(i32, ControlledShutdownResponse),
    OffsetCommitResponse(i32, OffsetCommitResponse),
    OffsetFetchResponse(i32, OffsetFetchResponse),
    FindCoordinatorResponse(i32, FindCoordinatorResponse),
    JoinGroupResponse(i32, JoinGroupResponse),
    HeartbeatResponse(i32, HeartbeatResponse),
    LeaveGroupResponse(i32, LeaveGroupResponse),
    SyncGroupResponse(i32, SyncGroupResponse),
    DescribeGroupsResponse(i32, DescribeGroupsResponse),
    ListGroupsResponse(i32, ListGroupsResponse),
    SaslHandshakeResponse(i32, SaslHandshakeResponse),
    ApiVersionsResponse(i32, ApiVersionsResponse),
    CreateTopicsResponse(i32, CreateTopicsResponse),
    DeleteTopicsResponse(i32, DeleteTopicsResponse),
    DeleteRecordsResponse(i32, DeleteRecordsResponse),
    InitProducerIdResponse(i32, InitProducerIdResponse),
    OffsetForLeaderEpochResponse(i32, OffsetForLeaderEpochResponse),
    AddPartitionsToTxnResponse(i32, AddPartitionsToTxnResponse),
    AddOffsetsToTxnResponse(i32, AddOffsetsToTxnResponse),
    EndTxnResponse(i32, EndTxnResponse),
    WriteTxnMarkersResponse(i32, WriteTxnMarkersResponse),
    TxnOffsetCommitResponse(i32, TxnOffsetCommitResponse),
    DescribeAclsResponse(i32, DescribeAclsResponse),
    CreateAclsResponse(i32, CreateAclsResponse),
    DeleteAclsResponse(i32, DeleteAclsResponse),
    DescribeConfigsResponse(i32, DescribeConfigsResponse),
    AlterConfigsResponse(i32, AlterConfigsResponse),
    AlterReplicaLogDirsResponse(i32, AlterReplicaLogDirsResponse),
    DescribeLogDirsResponse(i32, DescribeLogDirsResponse),
    SaslAuthenticateResponse(i32, SaslAuthenticateResponse),
    CreatePartitionsResponse(i32, CreatePartitionsResponse),
    CreateDelegationTokenResponse(i32, CreateDelegationTokenResponse),
    RenewDelegationTokenResponse(i32, RenewDelegationTokenResponse),
    ExpireDelegationTokenResponse(i32, ExpireDelegationTokenResponse),
    DescribeDelegationTokenResponse(i32, DescribeDelegationTokenResponse),
    DeleteGroupsResponse(i32, DeleteGroupsResponse),
    ElectLeadersResponse(i32, ElectLeadersResponse),
    IncrementalAlterConfigsResponse(i32, IncrementalAlterConfigsResponse),
    AlterPartitionReassignmentsResponse(i32, AlterPartitionReassignmentsResponse),
    ListPartitionReassignmentsResponse(i32, ListPartitionReassignmentsResponse),
    OffsetDeleteResponse(i32, OffsetDeleteResponse),
    DescribeClientQuotasResponse(i32, DescribeClientQuotasResponse),
    AlterClientQuotasResponse(i32, AlterClientQuotasResponse),
    DescribeUserScramCredentialsResponse(i32, DescribeUserScramCredentialsResponse),
    AlterUserScramCredentialsResponse(i32, AlterUserScramCredentialsResponse),
    VoteResponse(i32, VoteResponse),
    BeginQuorumEpochResponse(i32, BeginQuorumEpochResponse),
    EndQuorumEpochResponse(i32, EndQuorumEpochResponse),
    DescribeQuorumResponse(i32, DescribeQuorumResponse),
    AlterPartitionResponse(i32, AlterPartitionResponse),
    UpdateFeaturesResponse(i32, UpdateFeaturesResponse),
    EnvelopeResponse(i32, EnvelopeResponse),
    FetchSnapshotResponse(i32, FetchSnapshotResponse),
    DescribeClusterResponse(i32, DescribeClusterResponse),
    DescribeProducersResponse(i32, DescribeProducersResponse),
    BrokerRegistrationResponse(i32, BrokerRegistrationResponse),
    BrokerHeartbeatResponse(i32, BrokerHeartbeatResponse),
    UnregisterBrokerResponse(i32, UnregisterBrokerResponse),
    DescribeTransactionsResponse(i32, DescribeTransactionsResponse),
    ListTransactionsResponse(i32, ListTransactionsResponse),
    AllocateProducerIdsResponse(i32, AllocateProducerIdsResponse),
    ConsumerGroupHeartbeatResponse(i32, ConsumerGroupHeartbeatResponse),
    ConsumerGroupDescribeResponse(i32, ConsumerGroupDescribeResponse),
    ControllerRegistrationResponse(i32, ControllerRegistrationResponse),
    GetTelemetrySubscriptionsResponse(i32, GetTelemetrySubscriptionsResponse),
    PushTelemetryResponse(i32, PushTelemetryResponse),
    AssignReplicasToDirsResponse(i32, AssignReplicasToDirsResponse),
    ListClientMetricsResourcesResponse(i32, ListClientMetricsResourcesResponse),
    DescribeTopicPartitionsResponse(i32, DescribeTopicPartitionsResponse),
}

#[async_trait]
impl<Writer: tokio::io::AsyncWrite + Send + Unpin + 'static, E: Debug> proto_tower_util::WriteTo<Writer, KafkaProtocolError<E>> for KafkaResponse {
    async fn write_to(&self, writer: &mut Writer) -> Result<(), KafkaProtocolError<E>> {
        let version = 666;
        match self {
            KafkaResponse::ProduceResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::FetchResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ListOffsetsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::MetadataResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::LeaderAndIsrResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::StopReplicaResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::UpdateMetadataResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ControlledShutdownResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::OffsetCommitResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::OffsetFetchResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::FindCoordinatorResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::JoinGroupResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::HeartbeatResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::LeaveGroupResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::SyncGroupResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeGroupsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ListGroupsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::SaslHandshakeResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ApiVersionsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::CreateTopicsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DeleteTopicsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DeleteRecordsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::InitProducerIdResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::OffsetForLeaderEpochResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AddPartitionsToTxnResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AddOffsetsToTxnResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::EndTxnResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::WriteTxnMarkersResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::TxnOffsetCommitResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeAclsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::CreateAclsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DeleteAclsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeConfigsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AlterConfigsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AlterReplicaLogDirsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeLogDirsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::SaslAuthenticateResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::CreatePartitionsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::CreateDelegationTokenResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::RenewDelegationTokenResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ExpireDelegationTokenResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeDelegationTokenResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DeleteGroupsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ElectLeadersResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::IncrementalAlterConfigsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AlterPartitionReassignmentsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ListPartitionReassignmentsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::OffsetDeleteResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeClientQuotasResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AlterClientQuotasResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeUserScramCredentialsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AlterUserScramCredentialsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::VoteResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::BeginQuorumEpochResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::EndQuorumEpochResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeQuorumResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AlterPartitionResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::UpdateFeaturesResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::EnvelopeResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::FetchSnapshotResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeClusterResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeProducersResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::BrokerRegistrationResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::BrokerHeartbeatResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::UnregisterBrokerResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeTransactionsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ListTransactionsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AllocateProducerIdsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ConsumerGroupHeartbeatResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ConsumerGroupDescribeResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ControllerRegistrationResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::GetTelemetrySubscriptionsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::PushTelemetryResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::AssignReplicasToDirsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::ListClientMetricsResourcesResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
            KafkaResponse::DescribeTopicPartitionsResponse(correlation_id, inner) => {
                encode_and_write_response!(correlation_id, inner, writer, version)
            }
        }
    }
}
