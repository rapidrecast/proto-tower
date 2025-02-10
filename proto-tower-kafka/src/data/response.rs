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
        let mut buff_mut = BytesMut::new();
        match self {
            KafkaResponse::ProduceResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FetchResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListOffsetsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::MetadataResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::LeaderAndIsrResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::StopReplicaResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UpdateMetadataResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ControlledShutdownResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetCommitResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetFetchResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FindCoordinatorResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::JoinGroupResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::HeartbeatResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::LeaveGroupResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SyncGroupResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeGroupsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListGroupsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SaslHandshakeResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ApiVersionsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateTopicsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteTopicsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteRecordsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::InitProducerIdResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetForLeaderEpochResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AddPartitionsToTxnResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AddOffsetsToTxnResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EndTxnResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::WriteTxnMarkersResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::TxnOffsetCommitResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeAclsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateAclsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteAclsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeConfigsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterConfigsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterReplicaLogDirsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeLogDirsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SaslAuthenticateResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreatePartitionsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateDelegationTokenResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::RenewDelegationTokenResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ExpireDelegationTokenResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeDelegationTokenResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteGroupsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ElectLeadersResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::IncrementalAlterConfigsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterPartitionReassignmentsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListPartitionReassignmentsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetDeleteResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeClientQuotasResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterClientQuotasResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeUserScramCredentialsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterUserScramCredentialsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::VoteResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BeginQuorumEpochResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EndQuorumEpochResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeQuorumResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterPartitionResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UpdateFeaturesResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EnvelopeResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FetchSnapshotResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeClusterResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeProducersResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BrokerRegistrationResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BrokerHeartbeatResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UnregisterBrokerResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeTransactionsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListTransactionsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AllocateProducerIdsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ConsumerGroupHeartbeatResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ConsumerGroupDescribeResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ControllerRegistrationResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::GetTelemetrySubscriptionsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::PushTelemetryResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AssignReplicasToDirsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListClientMetricsResourcesResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeTopicPartitionsResponse(_correlation_id, inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
        }
    }
}
