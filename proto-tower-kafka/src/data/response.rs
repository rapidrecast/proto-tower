use crate::data::KafkaProtocolError;
use crate::encode_and_write;
use async_trait::async_trait;
use bytes::BytesMut;
use kafka_protocol::messages::*;
use kafka_protocol::protocol::Encodable;
use std::fmt::Debug;
use tokio::io::AsyncWriteExt;

#[macro_export]
macro_rules! encode_and_write {
    ($inner:ident, $buff_mut:ident, $writer:ident, $version:ident) => {{
        $inner
            .encode(&mut $buff_mut, $version)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response encode failure"))?;
        let sz = $buff_mut.len() as i32;
        let sz_bytes: [u8; 4] = sz.to_be_bytes();
        $writer
            .write_all(&sz_bytes)
            .await
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response size write failure"))?;
        $writer
            .write_all(&$buff_mut)
            .await
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response payload write failure"))?;
        Ok(())
    }};
}

#[derive(Debug, Clone, PartialEq)]
pub enum KafkaResponse {
    ProduceResponse(ProduceResponse),
    FetchResponse(FetchResponse),
    ListOffsetsResponse(ListOffsetsResponse),
    MetadataResponse(MetadataResponse),
    LeaderAndIsrResponse(LeaderAndIsrResponse),
    StopReplicaResponse(StopReplicaResponse),
    UpdateMetadataResponse(UpdateMetadataResponse),
    ControlledShutdownResponse(ControlledShutdownResponse),
    OffsetCommitResponse(OffsetCommitResponse),
    OffsetFetchResponse(OffsetFetchResponse),
    FindCoordinatorResponse(FindCoordinatorResponse),
    JoinGroupResponse(JoinGroupResponse),
    HeartbeatResponse(HeartbeatResponse),
    LeaveGroupResponse(LeaveGroupResponse),
    SyncGroupResponse(SyncGroupResponse),
    DescribeGroupsResponse(DescribeGroupsResponse),
    ListGroupsResponse(ListGroupsResponse),
    SaslHandshakeResponse(SaslHandshakeResponse),
    ApiVersionsResponse(ApiVersionsResponse),
    CreateTopicsResponse(CreateTopicsResponse),
    DeleteTopicsResponse(DeleteTopicsResponse),
    DeleteRecordsResponse(DeleteRecordsResponse),
    InitProducerIdResponse(InitProducerIdResponse),
    OffsetForLeaderEpochResponse(OffsetForLeaderEpochResponse),
    AddPartitionsToTxnResponse(AddPartitionsToTxnResponse),
    AddOffsetsToTxnResponse(AddOffsetsToTxnResponse),
    EndTxnResponse(EndTxnResponse),
    WriteTxnMarkersResponse(WriteTxnMarkersResponse),
    TxnOffsetCommitResponse(TxnOffsetCommitResponse),
    DescribeAclsResponse(DescribeAclsResponse),
    CreateAclsResponse(CreateAclsResponse),
    DeleteAclsResponse(DeleteAclsResponse),
    DescribeConfigsResponse(DescribeConfigsResponse),
    AlterConfigsResponse(AlterConfigsResponse),
    AlterReplicaLogDirsResponse(AlterReplicaLogDirsResponse),
    DescribeLogDirsResponse(DescribeLogDirsResponse),
    SaslAuthenticateResponse(SaslAuthenticateResponse),
    CreatePartitionsResponse(CreatePartitionsResponse),
    CreateDelegationTokenResponse(CreateDelegationTokenResponse),
    RenewDelegationTokenResponse(RenewDelegationTokenResponse),
    ExpireDelegationTokenResponse(ExpireDelegationTokenResponse),
    DescribeDelegationTokenResponse(DescribeDelegationTokenResponse),
    DeleteGroupsResponse(DeleteGroupsResponse),
    ElectLeadersResponse(ElectLeadersResponse),
    IncrementalAlterConfigsResponse(IncrementalAlterConfigsResponse),
    AlterPartitionReassignmentsResponse(AlterPartitionReassignmentsResponse),
    ListPartitionReassignmentsResponse(ListPartitionReassignmentsResponse),
    OffsetDeleteResponse(OffsetDeleteResponse),
    DescribeClientQuotasResponse(DescribeClientQuotasResponse),
    AlterClientQuotasResponse(AlterClientQuotasResponse),
    DescribeUserScramCredentialsResponse(DescribeUserScramCredentialsResponse),
    AlterUserScramCredentialsResponse(AlterUserScramCredentialsResponse),
    VoteResponse(VoteResponse),
    BeginQuorumEpochResponse(BeginQuorumEpochResponse),
    EndQuorumEpochResponse(EndQuorumEpochResponse),
    DescribeQuorumResponse(DescribeQuorumResponse),
    AlterPartitionResponse(AlterPartitionResponse),
    UpdateFeaturesResponse(UpdateFeaturesResponse),
    EnvelopeResponse(EnvelopeResponse),
    FetchSnapshotResponse(FetchSnapshotResponse),
    DescribeClusterResponse(DescribeClusterResponse),
    DescribeProducersResponse(DescribeProducersResponse),
    BrokerRegistrationResponse(BrokerRegistrationResponse),
    BrokerHeartbeatResponse(BrokerHeartbeatResponse),
    UnregisterBrokerResponse(UnregisterBrokerResponse),
    DescribeTransactionsResponse(DescribeTransactionsResponse),
    ListTransactionsResponse(ListTransactionsResponse),
    AllocateProducerIdsResponse(AllocateProducerIdsResponse),
    ConsumerGroupHeartbeatResponse(ConsumerGroupHeartbeatResponse),
    ConsumerGroupDescribeResponse(ConsumerGroupDescribeResponse),
    ControllerRegistrationResponse(ControllerRegistrationResponse),
    GetTelemetrySubscriptionsResponse(GetTelemetrySubscriptionsResponse),
    PushTelemetryResponse(PushTelemetryResponse),
    AssignReplicasToDirsResponse(AssignReplicasToDirsResponse),
    ListClientMetricsResourcesResponse(ListClientMetricsResourcesResponse),
    DescribeTopicPartitionsResponse(DescribeTopicPartitionsResponse),
}

#[async_trait]
impl<Writer: tokio::io::AsyncWrite + Send + Unpin + 'static, E: Debug> proto_tower_util::WriteTo<Writer, KafkaProtocolError<E>> for KafkaResponse {
    async fn write_to(&self, writer: &mut Writer) -> Result<(), KafkaProtocolError<E>> {
        let version = 666;
        let mut buff_mut = BytesMut::new();
        match self {
            KafkaResponse::ProduceResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FetchResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListOffsetsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::MetadataResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::LeaderAndIsrResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::StopReplicaResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UpdateMetadataResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ControlledShutdownResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetCommitResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetFetchResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FindCoordinatorResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::JoinGroupResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::HeartbeatResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::LeaveGroupResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SyncGroupResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeGroupsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListGroupsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SaslHandshakeResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ApiVersionsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateTopicsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteTopicsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteRecordsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::InitProducerIdResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetForLeaderEpochResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AddPartitionsToTxnResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AddOffsetsToTxnResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EndTxnResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::WriteTxnMarkersResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::TxnOffsetCommitResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeAclsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateAclsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteAclsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeConfigsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterConfigsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterReplicaLogDirsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeLogDirsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::SaslAuthenticateResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreatePartitionsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::CreateDelegationTokenResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::RenewDelegationTokenResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ExpireDelegationTokenResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeDelegationTokenResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DeleteGroupsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ElectLeadersResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::IncrementalAlterConfigsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterPartitionReassignmentsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListPartitionReassignmentsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::OffsetDeleteResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeClientQuotasResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterClientQuotasResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeUserScramCredentialsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterUserScramCredentialsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::VoteResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BeginQuorumEpochResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EndQuorumEpochResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeQuorumResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AlterPartitionResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UpdateFeaturesResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::EnvelopeResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::FetchSnapshotResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeClusterResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeProducersResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BrokerRegistrationResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::BrokerHeartbeatResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::UnregisterBrokerResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeTransactionsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListTransactionsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AllocateProducerIdsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ConsumerGroupHeartbeatResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ConsumerGroupDescribeResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ControllerRegistrationResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::GetTelemetrySubscriptionsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::PushTelemetryResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::AssignReplicasToDirsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::ListClientMetricsResourcesResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
            KafkaResponse::DescribeTopicPartitionsResponse(inner) => {
                encode_and_write!(inner, buff_mut, writer, version)
            }
        }
    }
}
