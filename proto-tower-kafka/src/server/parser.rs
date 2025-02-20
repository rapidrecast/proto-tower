use crate::data::KafkaRequest;
use bytes::Buf;
use kafka_protocol::messages::*;
use kafka_protocol::protocol::buf::ByteBuf;
use kafka_protocol::protocol::Decodable;
use std::collections::BTreeMap;
use std::task::Poll;

/// Not a future, but is noop if there is not enough data (ie Poll::Pending)
/// TODO we need the header request in response to retain protocol info (api header and data versions + correlation_id)
pub fn parse_kafka_request<B: ByteBuf>(buff: &mut B, tracked_requests: &mut BTreeMap<i32, i16>) -> Poll<Result<(RequestHeader, KafkaRequest), String>> {
    if buff.remaining() < 4 {
        return Poll::Pending;
    }
    let size = Buf::try_get_i32(&mut buff.peek_bytes(0..4)).map_err(|e| format!("Size failed: {}", e))? as usize;
    if buff.remaining() < size + 4 {
        return Poll::Pending;
    }
    // Get the size, so it's not in the buffer
    let _size = Buf::try_get_i32(buff).unwrap();
    eprintln!("parse_kafka_request for buf:\n{}", proto_tower_util::debug::debug_hex(buff.chunk()));
    // Peek the first 4 bytes to determine header
    let api_key = Buf::try_get_i16(&mut buff.peek_bytes(0..2)).map_err(|_| "API Key")?;
    let api_version = Buf::try_get_i16(&mut buff.peek_bytes(2..4)).map_err(|_| "API Version")?;

    // Now that we have header info we can retrieve the header
    let api_key = ApiKey::try_from(api_key).map_err(|_| "Invalid API Key")?;
    eprintln!("API Key {:?}", api_key);
    let header_version = api_key.request_header_version(api_version);
    let header = RequestHeader::decode(buff, header_version).map_err(|_| "Invalid API Version")?;
    let correlation_id = header.correlation_id;
    tracked_requests.insert(correlation_id, api_version);
    match api_key {
        ApiKey::Produce => match ProduceRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ProduceRequest(request)))),
            Err(e) => Poll::Ready(Err(format!("Invalid Produce Request: {}", e))),
        },
        ApiKey::Fetch => match FetchRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::FetchRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid Fetch Request".to_string())),
        },
        ApiKey::ListOffsets => match ListOffsetsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ListOffsetsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ListOffsets Request".to_string())),
        },
        ApiKey::Metadata => match MetadataRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::MetadataRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid Metadata Request".to_string())),
        },
        ApiKey::LeaderAndIsr => match LeaderAndIsrRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::LeaderAndIsrRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid LeaderAndIsr Request".to_string())),
        },
        ApiKey::StopReplica => match StopReplicaRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::StopReplicaRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid StopReplica Request".to_string())),
        },
        ApiKey::UpdateMetadata => match UpdateMetadataRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::UpdateMetadataRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid UpdateMetadata Request".to_string())),
        },
        ApiKey::ControlledShutdown => match ControlledShutdownRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ControlledShutdownRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ControlledShutdown Request".to_string())),
        },
        ApiKey::OffsetCommit => match OffsetCommitRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::OffsetCommitRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid OffsetCommit Request".to_string())),
        },
        ApiKey::OffsetFetch => match OffsetFetchRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::OffsetFetchRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid OffsetFetch Request".to_string())),
        },
        ApiKey::FindCoordinator => match FindCoordinatorRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::FindCoordinatorRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid FindCoordinator Request".to_string())),
        },
        ApiKey::JoinGroup => match JoinGroupRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::JoinGroupRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid JoinGroup Request".to_string())),
        },
        ApiKey::Heartbeat => match HeartbeatRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::HeartbeatRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid Heartbeat Request".to_string())),
        },
        ApiKey::LeaveGroup => match LeaveGroupRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::LeaveGroupRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid LeaveGroup Request".to_string())),
        },
        ApiKey::SyncGroup => match SyncGroupRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::SyncGroupRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid SyncGroup Request".to_string())),
        },
        ApiKey::DescribeGroups => match DescribeGroupsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeGroupsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeGroups Request".to_string())),
        },
        ApiKey::ListGroups => match ListGroupsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ListGroupsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ListGroups Request".to_string())),
        },
        ApiKey::SaslHandshake => match SaslHandshakeRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::SaslHandshakeRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid SaslHandshake Request".to_string())),
        },
        ApiKey::ApiVersions => match ApiVersionsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ApiVersionsRequest(request)))),
            Err(e) => Poll::Ready(Err(format!("Invalid ApiVersions Request: {}", e))),
        },
        ApiKey::CreateTopics => match CreateTopicsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::CreateTopicsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid CreateTopics Request".to_string())),
        },
        ApiKey::DeleteTopics => match DeleteTopicsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DeleteTopicsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DeleteTopics Request".to_string())),
        },
        ApiKey::DeleteRecords => match DeleteRecordsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DeleteRecordsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DeleteRecords Request".to_string())),
        },
        ApiKey::InitProducerId => match InitProducerIdRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::InitProducerIdRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid InitProducerId Request".to_string())),
        },
        ApiKey::OffsetForLeaderEpoch => match OffsetForLeaderEpochRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::OffsetForLeaderEpochRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid OffsetForLeaderEpoch Request".to_string())),
        },
        ApiKey::AddPartitionsToTxn => match AddPartitionsToTxnRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AddPartitionsToTxnRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AddPartitionsToTxn Request".to_string())),
        },
        ApiKey::AddOffsetsToTxn => match AddOffsetsToTxnRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AddOffsetsToTxnRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AddOffsetsToTxn Request".to_string())),
        },
        ApiKey::EndTxn => match EndTxnRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::EndTxnRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid EndTxn Request".to_string())),
        },
        ApiKey::WriteTxnMarkers => match WriteTxnMarkersRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::WriteTxnMarkersRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid WriteTxnMarkers Request".to_string())),
        },
        ApiKey::TxnOffsetCommit => match TxnOffsetCommitRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::TxnOffsetCommitRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid TxnOffsetCommit Request".to_string())),
        },
        ApiKey::DescribeAcls => match DescribeAclsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeAclsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeAcls Request".to_string())),
        },
        ApiKey::CreateAcls => match CreateAclsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::CreateAclsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid CreateAcls Request".to_string())),
        },
        ApiKey::DeleteAcls => match DeleteAclsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DeleteAclsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DeleteAcls Request".to_string())),
        },
        ApiKey::DescribeConfigs => match DescribeConfigsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeConfigsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeConfigs Request".to_string())),
        },
        ApiKey::AlterConfigs => match AlterConfigsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AlterConfigsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AlterConfigs Request".to_string())),
        },
        ApiKey::AlterReplicaLogDirs => match AlterReplicaLogDirsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AlterReplicaLogDirsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AlterReplicaLogDirs Request".to_string())),
        },
        ApiKey::DescribeLogDirs => match DescribeLogDirsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeLogDirsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeLogDirs Request".to_string())),
        },
        ApiKey::SaslAuthenticate => match SaslAuthenticateRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::SaslAuthenticateRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid SaslAuthenticate Request".to_string())),
        },
        ApiKey::CreatePartitions => match CreatePartitionsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::CreatePartitionsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid CreatePartitions Request".to_string())),
        },
        ApiKey::CreateDelegationToken => match CreateDelegationTokenRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::CreateDelegationTokenRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid CreateDelegationToken Request".to_string())),
        },
        ApiKey::RenewDelegationToken => match RenewDelegationTokenRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::RenewDelegationTokenRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid RenewDelegationToken Request".to_string())),
        },
        ApiKey::ExpireDelegationToken => match ExpireDelegationTokenRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ExpireDelegationTokenRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ExpireDelegationToken Request".to_string())),
        },
        ApiKey::DescribeDelegationToken => match DescribeDelegationTokenRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeDelegationTokenRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeDelegationToken Request".to_string())),
        },
        ApiKey::DeleteGroups => match DeleteGroupsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DeleteGroupsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DeleteGroups Request".to_string())),
        },
        ApiKey::ElectLeaders => match ElectLeadersRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ElectLeadersRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ElectLeaders Request".to_string())),
        },
        ApiKey::IncrementalAlterConfigs => match IncrementalAlterConfigsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::IncrementalAlterConfigsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid IncrementalAlterConfigs Request".to_string())),
        },
        ApiKey::AlterPartitionReassignments => match AlterPartitionReassignmentsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AlterPartitionReassignmentsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AlterPartitionReassignments Request".to_string())),
        },
        ApiKey::ListPartitionReassignments => match ListPartitionReassignmentsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ListPartitionReassignmentsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ListPartitionReassignments Request".to_string())),
        },
        ApiKey::OffsetDelete => match OffsetDeleteRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::OffsetDeleteRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid OffsetDelete Request".to_string())),
        },
        ApiKey::DescribeClientQuotas => match DescribeClientQuotasRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeClientQuotasRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeClientQuotas Request".to_string())),
        },
        ApiKey::AlterClientQuotas => match AlterClientQuotasRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AlterClientQuotasRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AlterClientQuotas Request".to_string())),
        },
        ApiKey::DescribeUserScramCredentials => match DescribeUserScramCredentialsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeUserScramCredentialsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeUserScramCredentials Request".to_string())),
        },
        ApiKey::AlterUserScramCredentials => match AlterUserScramCredentialsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AlterUserScramCredentialsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AlterUserScramCredentials Request".to_string())),
        },
        ApiKey::Vote => match VoteRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::VoteRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid Vote Request".to_string())),
        },
        ApiKey::BeginQuorumEpoch => match BeginQuorumEpochRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::BeginQuorumEpochRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid BeginQuorumEpoch Request".to_string())),
        },
        ApiKey::EndQuorumEpoch => match EndQuorumEpochRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::EndQuorumEpochRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid EndQuorumEpoch Request".to_string())),
        },
        ApiKey::DescribeQuorum => match DescribeQuorumRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeQuorumRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeQuorum Request".to_string())),
        },
        ApiKey::AlterPartition => match AlterPartitionRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AlterPartitionRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AlterPartition Request".to_string())),
        },
        ApiKey::UpdateFeatures => match UpdateFeaturesRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::UpdateFeaturesRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid UpdateFeatures Request".to_string())),
        },
        ApiKey::Envelope => match EnvelopeRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::EnvelopeRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid Envelope Request".to_string())),
        },
        ApiKey::FetchSnapshot => match FetchSnapshotRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::FetchSnapshotRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid FetchSnapshot Request".to_string())),
        },
        ApiKey::DescribeCluster => match DescribeClusterRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeClusterRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeCluster Request".to_string())),
        },
        ApiKey::DescribeProducers => match DescribeProducersRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeProducersRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeProducers Request".to_string())),
        },
        ApiKey::BrokerRegistration => match BrokerRegistrationRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::BrokerRegistrationRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid BrokerRegistration Request".to_string())),
        },
        ApiKey::BrokerHeartbeat => match BrokerHeartbeatRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::BrokerHeartbeatRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid BrokerHeartbeat Request".to_string())),
        },
        ApiKey::UnregisterBroker => match UnregisterBrokerRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::UnregisterBrokerRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid UnregisterBroker Request".to_string())),
        },
        ApiKey::DescribeTransactions => match DescribeTransactionsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeTransactionsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeTransactions Request".to_string())),
        },
        ApiKey::ListTransactions => match ListTransactionsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ListTransactionsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ListTransactions Request".to_string())),
        },
        ApiKey::AllocateProducerIds => match AllocateProducerIdsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AllocateProducerIdsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AllocateProducerIds Request".to_string())),
        },
        ApiKey::ConsumerGroupHeartbeat => match ConsumerGroupHeartbeatRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ConsumerGroupHeartbeatRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ConsumerGroupHeartbeat Request".to_string())),
        },
        ApiKey::ConsumerGroupDescribe => match ConsumerGroupDescribeRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ConsumerGroupDescribeRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ConsumerGroupDescribe Request".to_string())),
        },
        ApiKey::ControllerRegistration => match ControllerRegistrationRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ControllerRegistrationRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ControllerRegistration Request".to_string())),
        },
        ApiKey::GetTelemetrySubscriptions => match GetTelemetrySubscriptionsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::GetTelemetrySubscriptionsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid GetTelemetrySubscriptions Request".to_string())),
        },
        ApiKey::PushTelemetry => match PushTelemetryRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::PushTelemetryRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid PushTelemetry Request".to_string())),
        },
        ApiKey::AssignReplicasToDirs => match AssignReplicasToDirsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::AssignReplicasToDirsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid AssignReplicasToDirs Request".to_string())),
        },
        ApiKey::ListClientMetricsResources => match ListClientMetricsResourcesRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::ListClientMetricsResourcesRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid ListClientMetricsResources Request".to_string())),
        },
        ApiKey::DescribeTopicPartitions => match DescribeTopicPartitionsRequest::decode(buff, header.request_api_version) {
            Ok(request) => Poll::Ready(Ok((header, KafkaRequest::DescribeTopicPartitionsRequest(request)))),
            Err(_) => Poll::Ready(Err("Invalid DescribeTopicPartitions Request".to_string())),
        },
    }
}

#[cfg(test)]
mod test {
    use crate::data::KafkaRequest;
    use crate::server::parser::parse_kafka_request;
    use bytes::BytesMut;
    use kafka_protocol::messages::ApiVersionsRequest;
    use kafka_protocol::protocol::StrBytes;
    use std::collections::BTreeMap;
    use std::task::Poll;

    #[test]
    pub fn test_api_version_request() {
        let payload = [
            0x00u8, 0x00, 0x00, 0x24, 0x00, 0x12, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x07, 0x72, 0x64, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x00, 0x0b, 0x6c, 0x69, 0x62,
            0x72, 0x64, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x06, 0x32, 0x2e, 0x33, 0x2e, 0x30, 0x00,
        ];
        let mut bbuff = BytesMut::new();
        bbuff.extend_from_slice(&payload);
        let inner = ApiVersionsRequest::default()
            .with_client_software_name(StrBytes::from("librdkafka"))
            .with_client_software_version(StrBytes::from("2.3.0"));
        let expected = KafkaRequest::ApiVersionsRequest(inner);
        let mut tracked_reqs = BTreeMap::new();
        let res = parse_kafka_request(&mut bbuff, &mut tracked_reqs);
        let res = match res {
            Poll::Ready(r) => r,
            Poll::Pending => panic!(),
        };
        let (_head, res) = res.unwrap();
        assert_eq!(res, expected);
    }
}
