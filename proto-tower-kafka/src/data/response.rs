use kafka_protocol::messages::*;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProtoInfo {
    pub(crate) correlation_id: i32,
    pub(crate) api_version: i16,
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
