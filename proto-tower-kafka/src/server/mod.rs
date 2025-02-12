use kafka_protocol::messages::api_versions_response::ApiVersion;
#[allow(unused_imports)]
use kafka_protocol::messages::ApiKey;

pub mod layer;
pub mod make_layer;
mod parser;
#[cfg(test)]
pub(crate) mod test;

#[derive(Clone, Debug)]
pub struct KafkaProtoServerConfig {
    pub timeout: std::time::Duration,
}

macro_rules! api_version {
    ($api_key:ident) => {
        ApiVersion::default()
            .with_api_key(ApiKey::$api_key as i16)
            .with_min_version(ApiKey::$api_key.valid_versions().min)
            .with_max_version(ApiKey::$api_key.valid_versions().max)
    };
}

pub fn all_api_versions() -> Vec<ApiVersion> {
    vec![
        api_version!(Produce),
        api_version!(Fetch),
        api_version!(ListOffsets),
        api_version!(Metadata),
        api_version!(LeaderAndIsr),
        api_version!(StopReplica),
        api_version!(UpdateMetadata),
        api_version!(ControlledShutdown),
        api_version!(OffsetCommit),
        api_version!(OffsetFetch),
        api_version!(FindCoordinator),
        api_version!(JoinGroup),
        api_version!(Heartbeat),
        api_version!(LeaveGroup),
        api_version!(SyncGroup),
        api_version!(DescribeGroups),
        api_version!(ListGroups),
        api_version!(SaslHandshake),
        api_version!(ApiVersions),
        api_version!(CreateTopics),
        api_version!(DeleteTopics),
        api_version!(DeleteRecords),
        api_version!(InitProducerId),
        api_version!(OffsetForLeaderEpoch),
        api_version!(AddPartitionsToTxn),
        api_version!(AddOffsetsToTxn),
        api_version!(EndTxn),
        api_version!(WriteTxnMarkers),
        api_version!(TxnOffsetCommit),
        api_version!(DescribeAcls),
        api_version!(CreateAcls),
        api_version!(DeleteAcls),
        api_version!(DescribeConfigs),
        api_version!(AlterConfigs),
        api_version!(AlterReplicaLogDirs),
        api_version!(DescribeLogDirs),
        api_version!(SaslAuthenticate),
        api_version!(CreatePartitions),
        api_version!(CreateDelegationToken),
        api_version!(RenewDelegationToken),
        api_version!(ExpireDelegationToken),
        api_version!(DescribeDelegationToken),
        api_version!(DeleteGroups),
        api_version!(ElectLeaders),
        api_version!(IncrementalAlterConfigs),
        api_version!(AlterPartitionReassignments),
        api_version!(ListPartitionReassignments),
        api_version!(OffsetDelete),
        api_version!(DescribeClientQuotas),
        api_version!(AlterClientQuotas),
        api_version!(DescribeUserScramCredentials),
        api_version!(AlterUserScramCredentials),
        api_version!(Vote),
        api_version!(BeginQuorumEpoch),
        api_version!(EndQuorumEpoch),
        api_version!(DescribeQuorum),
        api_version!(AlterPartition),
        api_version!(UpdateFeatures),
        api_version!(Envelope),
        api_version!(FetchSnapshot),
        api_version!(DescribeCluster),
        api_version!(DescribeProducers),
        api_version!(BrokerRegistration),
        api_version!(BrokerHeartbeat),
        api_version!(UnregisterBroker),
        api_version!(DescribeTransactions),
        api_version!(ListTransactions),
        api_version!(AllocateProducerIds),
        api_version!(ConsumerGroupHeartbeat),
        api_version!(ConsumerGroupDescribe),
        api_version!(ControllerRegistration),
        api_version!(GetTelemetrySubscriptions),
        api_version!(PushTelemetry),
        api_version!(AssignReplicasToDirs),
        api_version!(ListClientMetricsResources),
        api_version!(DescribeTopicPartitions),
    ]
}
