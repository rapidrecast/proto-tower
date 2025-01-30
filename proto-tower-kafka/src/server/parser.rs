use crate::data::KafkaRequest;
use bytes::Buf;
use kafka_protocol::messages::*;
use kafka_protocol::protocol::buf::ByteBuf;
use kafka_protocol::protocol::Decodable;

pub fn parse_kafka_request<B: ByteBuf>(buff: &mut B) -> Result<KafkaRequest, &'static str> {
    eprintln!("parse_kafka_request for buf:\n{}", proto_tower_util::debug::debug_hex(buff.chunk()));
    // Peek the first 4 bytes to determine header
    let api_key = buff.peek_bytes(0..2).try_get_i16().map_err(|_| "API Key")?;
    let api_version = buff.peek_bytes(2..4).try_get_i16().map_err(|_| "API Version")?;

    // Now that we have header info we can retrieve the header
    let api_key = ApiKey::try_from(api_key).map_err(|_| "Invalid API Key")?;
    let header_version = api_key.request_header_version(api_version);
    let header = RequestHeader::decode(buff, header_version).map_err(|_| "Invalid API Version")?;
    match api_key {
        ApiKey::Produce => match ProduceRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ProduceRequest(request)),
            Err(_) => Err("Invalid Produce Request"),
        },
        ApiKey::Fetch => match FetchRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::FetchRequest(request)),
            Err(_) => Err("Invalid Fetch Request"),
        },
        ApiKey::ListOffsets => match ListOffsetsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ListOffsetsRequest(request)),
            Err(_) => Err("Invalid ListOffsets Request"),
        },
        ApiKey::Metadata => match MetadataRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::MetadataRequest(request)),
            Err(_) => Err("Invalid Metadata Request"),
        },
        ApiKey::LeaderAndIsr => match LeaderAndIsrRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::LeaderAndIsrRequest(request)),
            Err(_) => Err("Invalid LeaderAndIsr Request"),
        },
        ApiKey::StopReplica => match StopReplicaRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::StopReplicaRequest(request)),
            Err(_) => Err("Invalid StopReplica Request"),
        },
        ApiKey::UpdateMetadata => match UpdateMetadataRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::UpdateMetadataRequest(request)),
            Err(_) => Err("Invalid UpdateMetadata Request"),
        },
        ApiKey::ControlledShutdown => match ControlledShutdownRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ControlledShutdownRequest(request)),
            Err(_) => Err("Invalid ControlledShutdown Request"),
        },
        ApiKey::OffsetCommit => match OffsetCommitRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::OffsetCommitRequest(request)),
            Err(_) => Err("Invalid OffsetCommit Request"),
        },
        ApiKey::OffsetFetch => match OffsetFetchRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::OffsetFetchRequest(request)),
            Err(_) => Err("Invalid OffsetFetch Request"),
        },
        ApiKey::FindCoordinator => match FindCoordinatorRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::FindCoordinatorRequest(request)),
            Err(_) => Err("Invalid FindCoordinator Request"),
        },
        ApiKey::JoinGroup => match JoinGroupRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::JoinGroupRequest(request)),
            Err(_) => Err("Invalid JoinGroup Request"),
        },
        ApiKey::Heartbeat => match HeartbeatRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::HeartbeatRequest(request)),
            Err(_) => Err("Invalid Heartbeat Request"),
        },
        ApiKey::LeaveGroup => match LeaveGroupRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::LeaveGroupRequest(request)),
            Err(_) => Err("Invalid LeaveGroup Request"),
        },
        ApiKey::SyncGroup => match SyncGroupRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::SyncGroupRequest(request)),
            Err(_) => Err("Invalid SyncGroup Request"),
        },
        ApiKey::DescribeGroups => match DescribeGroupsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeGroupsRequest(request)),
            Err(_) => Err("Invalid DescribeGroups Request"),
        },
        ApiKey::ListGroups => match ListGroupsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ListGroupsRequest(request)),
            Err(_) => Err("Invalid ListGroups Request"),
        },
        ApiKey::SaslHandshake => match SaslHandshakeRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::SaslHandshakeRequest(request)),
            Err(_) => Err("Invalid SaslHandshake Request"),
        },
        ApiKey::ApiVersions => match ApiVersionsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ApiVersionsRequest(request)),
            Err(_) => Err("Invalid ApiVersions Request"),
        },
        ApiKey::CreateTopics => match CreateTopicsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::CreateTopicsRequest(request)),
            Err(_) => Err("Invalid CreateTopics Request"),
        },
        ApiKey::DeleteTopics => match DeleteTopicsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DeleteTopicsRequest(request)),
            Err(_) => Err("Invalid DeleteTopics Request"),
        },
        ApiKey::DeleteRecords => match DeleteRecordsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DeleteRecordsRequest(request)),
            Err(_) => Err("Invalid DeleteRecords Request"),
        },
        ApiKey::InitProducerId => match InitProducerIdRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::InitProducerIdRequest(request)),
            Err(_) => Err("Invalid InitProducerId Request"),
        },
        ApiKey::OffsetForLeaderEpoch => match OffsetForLeaderEpochRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::OffsetForLeaderEpochRequest(request)),
            Err(_) => Err("Invalid OffsetForLeaderEpoch Request"),
        },
        ApiKey::AddPartitionsToTxn => match AddPartitionsToTxnRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AddPartitionsToTxnRequest(request)),
            Err(_) => Err("Invalid AddPartitionsToTxn Request"),
        },
        ApiKey::AddOffsetsToTxn => match AddOffsetsToTxnRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AddOffsetsToTxnRequest(request)),
            Err(_) => Err("Invalid AddOffsetsToTxn Request"),
        },
        ApiKey::EndTxn => match EndTxnRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::EndTxnRequest(request)),
            Err(_) => Err("Invalid EndTxn Request"),
        },
        ApiKey::WriteTxnMarkers => match WriteTxnMarkersRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::WriteTxnMarkersRequest(request)),
            Err(_) => Err("Invalid WriteTxnMarkers Request"),
        },
        ApiKey::TxnOffsetCommit => match TxnOffsetCommitRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::TxnOffsetCommitRequest(request)),
            Err(_) => Err("Invalid TxnOffsetCommit Request"),
        },
        ApiKey::DescribeAcls => match DescribeAclsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeAclsRequest(request)),
            Err(_) => Err("Invalid DescribeAcls Request"),
        },
        ApiKey::CreateAcls => match CreateAclsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::CreateAclsRequest(request)),
            Err(_) => Err("Invalid CreateAcls Request"),
        },
        ApiKey::DeleteAcls => match DeleteAclsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DeleteAclsRequest(request)),
            Err(_) => Err("Invalid DeleteAcls Request"),
        },
        ApiKey::DescribeConfigs => match DescribeConfigsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeConfigsRequest(request)),
            Err(_) => Err("Invalid DescribeConfigs Request"),
        },
        ApiKey::AlterConfigs => match AlterConfigsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AlterConfigsRequest(request)),
            Err(_) => Err("Invalid AlterConfigs Request"),
        },
        ApiKey::AlterReplicaLogDirs => match AlterReplicaLogDirsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AlterReplicaLogDirsRequest(request)),
            Err(_) => Err("Invalid AlterReplicaLogDirs Request"),
        },
        ApiKey::DescribeLogDirs => match DescribeLogDirsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeLogDirsRequest(request)),
            Err(_) => Err("Invalid DescribeLogDirs Request"),
        },
        ApiKey::SaslAuthenticate => match SaslAuthenticateRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::SaslAuthenticateRequest(request)),
            Err(_) => Err("Invalid SaslAuthenticate Request"),
        },
        ApiKey::CreatePartitions => match CreatePartitionsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::CreatePartitionsRequest(request)),
            Err(_) => Err("Invalid CreatePartitions Request"),
        },
        ApiKey::CreateDelegationToken => match CreateDelegationTokenRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::CreateDelegationTokenRequest(request)),
            Err(_) => Err("Invalid CreateDelegationToken Request"),
        },
        ApiKey::RenewDelegationToken => match RenewDelegationTokenRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::RenewDelegationTokenRequest(request)),
            Err(_) => Err("Invalid RenewDelegationToken Request"),
        },
        ApiKey::ExpireDelegationToken => match ExpireDelegationTokenRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ExpireDelegationTokenRequest(request)),
            Err(_) => Err("Invalid ExpireDelegationToken Request"),
        },
        ApiKey::DescribeDelegationToken => match DescribeDelegationTokenRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeDelegationTokenRequest(request)),
            Err(_) => Err("Invalid DescribeDelegationToken Request"),
        },
        ApiKey::DeleteGroups => match DeleteGroupsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DeleteGroupsRequest(request)),
            Err(_) => Err("Invalid DeleteGroups Request"),
        },
        ApiKey::ElectLeaders => match ElectLeadersRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ElectLeadersRequest(request)),
            Err(_) => Err("Invalid ElectLeaders Request"),
        },
        ApiKey::IncrementalAlterConfigs => match IncrementalAlterConfigsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::IncrementalAlterConfigsRequest(request)),
            Err(_) => Err("Invalid IncrementalAlterConfigs Request"),
        },
        ApiKey::AlterPartitionReassignments => match AlterPartitionReassignmentsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AlterPartitionReassignmentsRequest(request)),
            Err(_) => Err("Invalid AlterPartitionReassignments Request"),
        },
        ApiKey::ListPartitionReassignments => match ListPartitionReassignmentsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ListPartitionReassignmentsRequest(request)),
            Err(_) => Err("Invalid ListPartitionReassignments Request"),
        },
        ApiKey::OffsetDelete => match OffsetDeleteRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::OffsetDeleteRequest(request)),
            Err(_) => Err("Invalid OffsetDelete Request"),
        },
        ApiKey::DescribeClientQuotas => match DescribeClientQuotasRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeClientQuotasRequest(request)),
            Err(_) => Err("Invalid DescribeClientQuotas Request"),
        },
        ApiKey::AlterClientQuotas => match AlterClientQuotasRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AlterClientQuotasRequest(request)),
            Err(_) => Err("Invalid AlterClientQuotas Request"),
        },
        ApiKey::DescribeUserScramCredentials => match DescribeUserScramCredentialsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeUserScramCredentialsRequest(request)),
            Err(_) => Err("Invalid DescribeUserScramCredentials Request"),
        },
        ApiKey::AlterUserScramCredentials => match AlterUserScramCredentialsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AlterUserScramCredentialsRequest(request)),
            Err(_) => Err("Invalid AlterUserScramCredentials Request"),
        },
        ApiKey::Vote => match VoteRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::VoteRequest(request)),
            Err(_) => Err("Invalid Vote Request"),
        },
        ApiKey::BeginQuorumEpoch => match BeginQuorumEpochRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::BeginQuorumEpochRequest(request)),
            Err(_) => Err("Invalid BeginQuorumEpoch Request"),
        },
        ApiKey::EndQuorumEpoch => match EndQuorumEpochRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::EndQuorumEpochRequest(request)),
            Err(_) => Err("Invalid EndQuorumEpoch Request"),
        },
        ApiKey::DescribeQuorum => match DescribeQuorumRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeQuorumRequest(request)),
            Err(_) => Err("Invalid DescribeQuorum Request"),
        },
        ApiKey::AlterPartition => match AlterPartitionRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AlterPartitionRequest(request)),
            Err(_) => Err("Invalid AlterPartition Request"),
        },
        ApiKey::UpdateFeatures => match UpdateFeaturesRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::UpdateFeaturesRequest(request)),
            Err(_) => Err("Invalid UpdateFeatures Request"),
        },
        ApiKey::Envelope => match EnvelopeRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::EnvelopeRequest(request)),
            Err(_) => Err("Invalid Envelope Request"),
        },
        ApiKey::FetchSnapshot => match FetchSnapshotRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::FetchSnapshotRequest(request)),
            Err(_) => Err("Invalid FetchSnapshot Request"),
        },
        ApiKey::DescribeCluster => match DescribeClusterRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeClusterRequest(request)),
            Err(_) => Err("Invalid DescribeCluster Request"),
        },
        ApiKey::DescribeProducers => match DescribeProducersRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeProducersRequest(request)),
            Err(_) => Err("Invalid DescribeProducers Request"),
        },
        ApiKey::BrokerRegistration => match BrokerRegistrationRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::BrokerRegistrationRequest(request)),
            Err(_) => Err("Invalid BrokerRegistration Request"),
        },
        ApiKey::BrokerHeartbeat => match BrokerHeartbeatRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::BrokerHeartbeatRequest(request)),
            Err(_) => Err("Invalid BrokerHeartbeat Request"),
        },
        ApiKey::UnregisterBroker => match UnregisterBrokerRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::UnregisterBrokerRequest(request)),
            Err(_) => Err("Invalid UnregisterBroker Request"),
        },
        ApiKey::DescribeTransactions => match DescribeTransactionsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeTransactionsRequest(request)),
            Err(_) => Err("Invalid DescribeTransactions Request"),
        },
        ApiKey::ListTransactions => match ListTransactionsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ListTransactionsRequest(request)),
            Err(_) => Err("Invalid ListTransactions Request"),
        },
        ApiKey::AllocateProducerIds => match AllocateProducerIdsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AllocateProducerIdsRequest(request)),
            Err(_) => Err("Invalid AllocateProducerIds Request"),
        },
        ApiKey::ConsumerGroupHeartbeat => match ConsumerGroupHeartbeatRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ConsumerGroupHeartbeatRequest(request)),
            Err(_) => Err("Invalid ConsumerGroupHeartbeat Request"),
        },
        ApiKey::ConsumerGroupDescribe => match ConsumerGroupDescribeRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ConsumerGroupDescribeRequest(request)),
            Err(_) => Err("Invalid ConsumerGroupDescribe Request"),
        },
        ApiKey::ControllerRegistration => match ControllerRegistrationRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ControllerRegistrationRequest(request)),
            Err(_) => Err("Invalid ControllerRegistration Request"),
        },
        ApiKey::GetTelemetrySubscriptions => match GetTelemetrySubscriptionsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::GetTelemetrySubscriptionsRequest(request)),
            Err(_) => Err("Invalid GetTelemetrySubscriptions Request"),
        },
        ApiKey::PushTelemetry => match PushTelemetryRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::PushTelemetryRequest(request)),
            Err(_) => Err("Invalid PushTelemetry Request"),
        },
        ApiKey::AssignReplicasToDirs => match AssignReplicasToDirsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::AssignReplicasToDirsRequest(request)),
            Err(_) => Err("Invalid AssignReplicasToDirs Request"),
        },
        ApiKey::ListClientMetricsResources => match ListClientMetricsResourcesRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::ListClientMetricsResourcesRequest(request)),
            Err(_) => Err("Invalid ListClientMetricsResources Request"),
        },
        ApiKey::DescribeTopicPartitions => match DescribeTopicPartitionsRequest::decode(buff, api_version) {
            Ok(request) => Ok(KafkaRequest::DescribeTopicPartitionsRequest(request)),
            Err(_) => Err("Invalid DescribeTopicPartitions Request"),
        },
    }
}
