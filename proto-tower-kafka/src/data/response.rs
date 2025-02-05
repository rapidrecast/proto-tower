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
    ProduceResponse(Box<ProduceResponse>),
    FetchResponse(Box<FetchResponse>),
    ListOffsetsResponse(Box<ListOffsetsResponse>),
    MetadataResponse(Box<MetadataResponse>),
    LeaderAndIsrResponse(Box<LeaderAndIsrResponse>),
    StopReplicaResponse(Box<StopReplicaResponse>),
    UpdateMetadataResponse(Box<UpdateMetadataResponse>),
    ControlledShutdownResponse(Box<ControlledShutdownResponse>),
    OffsetCommitResponse(Box<OffsetCommitResponse>),
    OffsetFetchResponse(Box<OffsetFetchResponse>),
    FindCoordinatorResponse(Box<FindCoordinatorResponse>),
    JoinGroupResponse(Box<JoinGroupResponse>),
    HeartbeatResponse(Box<HeartbeatResponse>),
    LeaveGroupResponse(Box<LeaveGroupResponse>),
    SyncGroupResponse(Box<SyncGroupResponse>),
    DescribeGroupsResponse(Box<DescribeGroupsResponse>),
    ListGroupsResponse(Box<ListGroupsResponse>),
    SaslHandshakeResponse(Box<SaslHandshakeResponse>),
    ApiVersionsResponse(Box<ApiVersionsResponse>),
    CreateTopicsResponse(Box<CreateTopicsResponse>),
    DeleteTopicsResponse(Box<DeleteTopicsResponse>),
    DeleteRecordsResponse(Box<DeleteRecordsResponse>),
    InitProducerIdResponse(Box<InitProducerIdResponse>),
    OffsetForLeaderEpochResponse(Box<OffsetForLeaderEpochResponse>),
    AddPartitionsToTxnResponse(Box<AddPartitionsToTxnResponse>),
    AddOffsetsToTxnResponse(Box<AddOffsetsToTxnResponse>),
    EndTxnResponse(Box<EndTxnResponse>),
    WriteTxnMarkersResponse(Box<WriteTxnMarkersResponse>),
    TxnOffsetCommitResponse(Box<TxnOffsetCommitResponse>),
    DescribeAclsResponse(Box<DescribeAclsResponse>),
    CreateAclsResponse(Box<CreateAclsResponse>),
    DeleteAclsResponse(Box<DeleteAclsResponse>),
    DescribeConfigsResponse(Box<DescribeConfigsResponse>),
    AlterConfigsResponse(Box<AlterConfigsResponse>),
    AlterReplicaLogDirsResponse(Box<AlterReplicaLogDirsResponse>),
    DescribeLogDirsResponse(Box<DescribeLogDirsResponse>),
    SaslAuthenticateResponse(Box<SaslAuthenticateResponse>),
    CreatePartitionsResponse(Box<CreatePartitionsResponse>),
    CreateDelegationTokenResponse(Box<CreateDelegationTokenResponse>),
    RenewDelegationTokenResponse(Box<RenewDelegationTokenResponse>),
    ExpireDelegationTokenResponse(Box<ExpireDelegationTokenResponse>),
    DescribeDelegationTokenResponse(Box<DescribeDelegationTokenResponse>),
    DeleteGroupsResponse(Box<DeleteGroupsResponse>),
    ElectLeadersResponse(Box<ElectLeadersResponse>),
    IncrementalAlterConfigsResponse(Box<IncrementalAlterConfigsResponse>),
    AlterPartitionReassignmentsResponse(Box<AlterPartitionReassignmentsResponse>),
    ListPartitionReassignmentsResponse(Box<ListPartitionReassignmentsResponse>),
    OffsetDeleteResponse(Box<OffsetDeleteResponse>),
    DescribeClientQuotasResponse(Box<DescribeClientQuotasResponse>),
    AlterClientQuotasResponse(Box<AlterClientQuotasResponse>),
    DescribeUserScramCredentialsResponse(Box<DescribeUserScramCredentialsResponse>),
    AlterUserScramCredentialsResponse(Box<AlterUserScramCredentialsResponse>),
    VoteResponse(Box<VoteResponse>),
    BeginQuorumEpochResponse(Box<BeginQuorumEpochResponse>),
    EndQuorumEpochResponse(Box<EndQuorumEpochResponse>),
    DescribeQuorumResponse(Box<DescribeQuorumResponse>),
    AlterPartitionResponse(Box<AlterPartitionResponse>),
    UpdateFeaturesResponse(Box<UpdateFeaturesResponse>),
    EnvelopeResponse(Box<EnvelopeResponse>),
    FetchSnapshotResponse(Box<FetchSnapshotResponse>),
    DescribeClusterResponse(Box<DescribeClusterResponse>),
    DescribeProducersResponse(Box<DescribeProducersResponse>),
    BrokerRegistrationResponse(Box<BrokerRegistrationResponse>),
    BrokerHeartbeatResponse(Box<BrokerHeartbeatResponse>),
    UnregisterBrokerResponse(Box<UnregisterBrokerResponse>),
    DescribeTransactionsResponse(Box<DescribeTransactionsResponse>),
    ListTransactionsResponse(Box<ListTransactionsResponse>),
    AllocateProducerIdsResponse(Box<AllocateProducerIdsResponse>),
    ConsumerGroupHeartbeatResponse(Box<ConsumerGroupHeartbeatResponse>),
    ConsumerGroupDescribeResponse(Box<ConsumerGroupDescribeResponse>),
    ControllerRegistrationResponse(Box<ControllerRegistrationResponse>),
    GetTelemetrySubscriptionsResponse(Box<GetTelemetrySubscriptionsResponse>),
    PushTelemetryResponse(Box<PushTelemetryResponse>),
    AssignReplicasToDirsResponse(Box<AssignReplicasToDirsResponse>),
    ListClientMetricsResourcesResponse(Box<ListClientMetricsResourcesResponse>),
    DescribeTopicPartitionsResponse(Box<DescribeTopicPartitionsResponse>),
}

#[async_trait]
impl<Writer: tokio::io::AsyncWrite + Send + Unpin + 'static, E: Debug> proto_tower_util::WriteTo<Writer, KafkaProtocolError<E>> for KafkaResponse {
    async fn write_to(&self, writer: &mut Writer) -> Result<(), KafkaProtocolError<E>> {
        let version = 666;
        let mut buff_mut = BytesMut::new();
        match self {
            KafkaResponse::ProduceResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FetchResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListOffsetsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::MetadataResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::LeaderAndIsrResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::StopReplicaResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UpdateMetadataResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ControlledShutdownResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetCommitResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetFetchResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FindCoordinatorResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::JoinGroupResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::HeartbeatResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::LeaveGroupResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SyncGroupResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeGroupsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListGroupsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SaslHandshakeResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ApiVersionsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateTopicsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteTopicsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteRecordsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::InitProducerIdResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetForLeaderEpochResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AddPartitionsToTxnResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AddOffsetsToTxnResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EndTxnResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::WriteTxnMarkersResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::TxnOffsetCommitResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeAclsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateAclsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteAclsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeConfigsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterConfigsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterReplicaLogDirsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeLogDirsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SaslAuthenticateResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreatePartitionsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateDelegationTokenResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::RenewDelegationTokenResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ExpireDelegationTokenResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeDelegationTokenResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteGroupsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ElectLeadersResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::IncrementalAlterConfigsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterPartitionReassignmentsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListPartitionReassignmentsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetDeleteResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeClientQuotasResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterClientQuotasResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeUserScramCredentialsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterUserScramCredentialsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::VoteResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BeginQuorumEpochResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EndQuorumEpochResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeQuorumResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterPartitionResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UpdateFeaturesResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EnvelopeResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FetchSnapshotResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeClusterResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeProducersResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BrokerRegistrationResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BrokerHeartbeatResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UnregisterBrokerResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeTransactionsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListTransactionsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AllocateProducerIdsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ConsumerGroupHeartbeatResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ConsumerGroupDescribeResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ControllerRegistrationResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::GetTelemetrySubscriptionsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::PushTelemetryResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AssignReplicasToDirsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListClientMetricsResourcesResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeTopicPartitionsResponse(inner) => {
                encode_and_write_response!(inner, buff_mut, writer, version)
            }
        }
    }
}
