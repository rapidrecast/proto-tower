use crate::data::KafkaProtocolError;
use crate::encode_and_write_response;
use async_trait::async_trait;
use bytes::BytesMut;
use kafka_protocol::messages::*;
use kafka_protocol::protocol::Encodable;
use std::fmt::Debug;
use tokio::io::AsyncWriteExt;

#[derive(Clone, Debug, PartialEq)]
pub struct ProtoInfo {
    pub(crate) correlation_id: i32,
    pub(crate) api_version: i16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KafkaResponse {
    ProduceResponse(ProtoInfo, ProduceResponse),
    FetchResponse(ProtoInfo, FetchResponse),
    ListOffsetsResponse(ProtoInfo, ListOffsetsResponse),
    MetadataResponse(ProtoInfo, MetadataResponse),
    LeaderAndIsrResponse(ProtoInfo, LeaderAndIsrResponse),
    StopReplicaResponse(ProtoInfo, StopReplicaResponse),
    UpdateMetadataResponse(ProtoInfo, UpdateMetadataResponse),
    ControlledShutdownResponse(ProtoInfo, ControlledShutdownResponse),
    OffsetCommitResponse(ProtoInfo, OffsetCommitResponse),
    OffsetFetchResponse(ProtoInfo, OffsetFetchResponse),
    FindCoordinatorResponse(ProtoInfo, FindCoordinatorResponse),
    JoinGroupResponse(ProtoInfo, JoinGroupResponse),
    HeartbeatResponse(ProtoInfo, HeartbeatResponse),
    LeaveGroupResponse(ProtoInfo, LeaveGroupResponse),
    SyncGroupResponse(ProtoInfo, SyncGroupResponse),
    DescribeGroupsResponse(ProtoInfo, DescribeGroupsResponse),
    ListGroupsResponse(ProtoInfo, ListGroupsResponse),
    SaslHandshakeResponse(ProtoInfo, SaslHandshakeResponse),
    ApiVersionsResponse(ProtoInfo, ApiVersionsResponse),
    CreateTopicsResponse(ProtoInfo, CreateTopicsResponse),
    DeleteTopicsResponse(ProtoInfo, DeleteTopicsResponse),
    DeleteRecordsResponse(ProtoInfo, DeleteRecordsResponse),
    InitProducerIdResponse(ProtoInfo, InitProducerIdResponse),
    OffsetForLeaderEpochResponse(ProtoInfo, OffsetForLeaderEpochResponse),
    AddPartitionsToTxnResponse(ProtoInfo, AddPartitionsToTxnResponse),
    AddOffsetsToTxnResponse(ProtoInfo, AddOffsetsToTxnResponse),
    EndTxnResponse(ProtoInfo, EndTxnResponse),
    WriteTxnMarkersResponse(ProtoInfo, WriteTxnMarkersResponse),
    TxnOffsetCommitResponse(ProtoInfo, TxnOffsetCommitResponse),
    DescribeAclsResponse(ProtoInfo, DescribeAclsResponse),
    CreateAclsResponse(ProtoInfo, CreateAclsResponse),
    DeleteAclsResponse(ProtoInfo, DeleteAclsResponse),
    DescribeConfigsResponse(ProtoInfo, DescribeConfigsResponse),
    AlterConfigsResponse(ProtoInfo, AlterConfigsResponse),
    AlterReplicaLogDirsResponse(ProtoInfo, AlterReplicaLogDirsResponse),
    DescribeLogDirsResponse(ProtoInfo, DescribeLogDirsResponse),
    SaslAuthenticateResponse(ProtoInfo, SaslAuthenticateResponse),
    CreatePartitionsResponse(ProtoInfo, CreatePartitionsResponse),
    CreateDelegationTokenResponse(ProtoInfo, CreateDelegationTokenResponse),
    RenewDelegationTokenResponse(ProtoInfo, RenewDelegationTokenResponse),
    ExpireDelegationTokenResponse(ProtoInfo, ExpireDelegationTokenResponse),
    DescribeDelegationTokenResponse(ProtoInfo, DescribeDelegationTokenResponse),
    DeleteGroupsResponse(ProtoInfo, DeleteGroupsResponse),
    ElectLeadersResponse(ProtoInfo, ElectLeadersResponse),
    IncrementalAlterConfigsResponse(ProtoInfo, IncrementalAlterConfigsResponse),
    AlterPartitionReassignmentsResponse(ProtoInfo, AlterPartitionReassignmentsResponse),
    ListPartitionReassignmentsResponse(ProtoInfo, ListPartitionReassignmentsResponse),
    OffsetDeleteResponse(ProtoInfo, OffsetDeleteResponse),
    DescribeClientQuotasResponse(ProtoInfo, DescribeClientQuotasResponse),
    AlterClientQuotasResponse(ProtoInfo, AlterClientQuotasResponse),
    DescribeUserScramCredentialsResponse(ProtoInfo, DescribeUserScramCredentialsResponse),
    AlterUserScramCredentialsResponse(ProtoInfo, AlterUserScramCredentialsResponse),
    VoteResponse(ProtoInfo, VoteResponse),
    BeginQuorumEpochResponse(ProtoInfo, BeginQuorumEpochResponse),
    EndQuorumEpochResponse(ProtoInfo, EndQuorumEpochResponse),
    DescribeQuorumResponse(ProtoInfo, DescribeQuorumResponse),
    AlterPartitionResponse(ProtoInfo, AlterPartitionResponse),
    UpdateFeaturesResponse(ProtoInfo, UpdateFeaturesResponse),
    EnvelopeResponse(ProtoInfo, EnvelopeResponse),
    FetchSnapshotResponse(ProtoInfo, FetchSnapshotResponse),
    DescribeClusterResponse(ProtoInfo, DescribeClusterResponse),
    DescribeProducersResponse(ProtoInfo, DescribeProducersResponse),
    BrokerRegistrationResponse(ProtoInfo, BrokerRegistrationResponse),
    BrokerHeartbeatResponse(ProtoInfo, BrokerHeartbeatResponse),
    UnregisterBrokerResponse(ProtoInfo, UnregisterBrokerResponse),
    DescribeTransactionsResponse(ProtoInfo, DescribeTransactionsResponse),
    ListTransactionsResponse(ProtoInfo, ListTransactionsResponse),
    AllocateProducerIdsResponse(ProtoInfo, AllocateProducerIdsResponse),
    ConsumerGroupHeartbeatResponse(ProtoInfo, ConsumerGroupHeartbeatResponse),
    ConsumerGroupDescribeResponse(ProtoInfo, ConsumerGroupDescribeResponse),
    ControllerRegistrationResponse(ProtoInfo, ControllerRegistrationResponse),
    GetTelemetrySubscriptionsResponse(ProtoInfo, GetTelemetrySubscriptionsResponse),
    PushTelemetryResponse(ProtoInfo, PushTelemetryResponse),
    AssignReplicasToDirsResponse(ProtoInfo, AssignReplicasToDirsResponse),
    ListClientMetricsResourcesResponse(ProtoInfo, ListClientMetricsResourcesResponse),
    DescribeTopicPartitionsResponse(ProtoInfo, DescribeTopicPartitionsResponse),
}

#[async_trait]
impl<Writer: tokio::io::AsyncWrite + Send + Unpin + 'static, E: Debug> proto_tower_util::WriteTo<Writer, KafkaProtocolError<E>> for KafkaResponse {
    async fn write_to(&self, writer: &mut Writer) -> Result<(), KafkaProtocolError<E>> {
        match self {
            KafkaResponse::ProduceResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::FetchResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListOffsetsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::MetadataResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::LeaderAndIsrResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::StopReplicaResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::UpdateMetadataResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ControlledShutdownResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::OffsetCommitResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::OffsetFetchResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::FindCoordinatorResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::JoinGroupResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::HeartbeatResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::LeaveGroupResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::SyncGroupResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeGroupsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListGroupsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::SaslHandshakeResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ApiVersionsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::CreateTopicsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DeleteTopicsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DeleteRecordsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::InitProducerIdResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::OffsetForLeaderEpochResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AddPartitionsToTxnResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AddOffsetsToTxnResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::EndTxnResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::WriteTxnMarkersResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::TxnOffsetCommitResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeAclsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::CreateAclsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DeleteAclsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeConfigsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterConfigsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterReplicaLogDirsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeLogDirsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::SaslAuthenticateResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::CreatePartitionsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::CreateDelegationTokenResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::RenewDelegationTokenResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ExpireDelegationTokenResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeDelegationTokenResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DeleteGroupsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ElectLeadersResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::IncrementalAlterConfigsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterPartitionReassignmentsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListPartitionReassignmentsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::OffsetDeleteResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeClientQuotasResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterClientQuotasResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeUserScramCredentialsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterUserScramCredentialsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::VoteResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::BeginQuorumEpochResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::EndQuorumEpochResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeQuorumResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterPartitionResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::UpdateFeaturesResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::EnvelopeResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::FetchSnapshotResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeClusterResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeProducersResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::BrokerRegistrationResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::BrokerHeartbeatResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::UnregisterBrokerResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeTransactionsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListTransactionsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AllocateProducerIdsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ConsumerGroupHeartbeatResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ConsumerGroupDescribeResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ControllerRegistrationResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::GetTelemetrySubscriptionsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::PushTelemetryResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AssignReplicasToDirsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListClientMetricsResourcesResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeTopicPartitionsResponse(proto_info, inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
        }
    }
}
