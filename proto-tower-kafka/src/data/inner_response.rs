use crate::data::{KafkaProtocolError, KafkaRequest, KafkaResponse, ProtoInfo};
use crate::encode_and_write_response;
use async_trait::async_trait;
#[allow(unused)]
use bytes::BytesMut;
#[allow(unused)]
use kafka_protocol::messages::ResponseHeader;
#[allow(unused)]
use kafka_protocol::protocol::Encodable;
use std::fmt::Debug;
#[allow(unused)]
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, PartialEq)]
pub struct TrackedKafkaRequest {
    pub correlation_id: i32,
    pub request: KafkaRequest,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackedKafkaResponse {
    pub correlation_id: i32,
    pub response: KafkaResponse,
}

impl TrackedKafkaResponse {
    pub fn into_inner(self, api_version: i16) -> InnerKafkaResponse {
        InnerKafkaResponse {
            correlation_id: self.correlation_id,
            header_version: self.response.api_key().response_header_version(api_version),
            api_version,
            response: self.response,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InnerKafkaResponse {
    correlation_id: i32,
    header_version: i16,
    api_version: i16,
    response: KafkaResponse,
}

#[async_trait]
impl<Writer: tokio::io::AsyncWrite + Send + Unpin + 'static, E: Debug> proto_tower_util::WriteTo<Writer, KafkaProtocolError<E>> for InnerKafkaResponse {
    async fn write_to(&self, writer: &mut Writer) -> Result<(), KafkaProtocolError<E>> {
        let proto_info = ProtoInfo {
            correlation_id: self.correlation_id,
            header_version: self.header_version,
            api_version: self.api_version,
        };
        match &self.response {
            KafkaResponse::ProduceResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::FetchResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListOffsetsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::MetadataResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::LeaderAndIsrResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::StopReplicaResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::UpdateMetadataResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ControlledShutdownResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::OffsetCommitResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::OffsetFetchResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::FindCoordinatorResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::JoinGroupResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::HeartbeatResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::LeaveGroupResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::SyncGroupResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeGroupsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListGroupsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::SaslHandshakeResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ApiVersionsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::CreateTopicsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DeleteTopicsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DeleteRecordsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::InitProducerIdResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::OffsetForLeaderEpochResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AddPartitionsToTxnResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AddOffsetsToTxnResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::EndTxnResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::WriteTxnMarkersResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::TxnOffsetCommitResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeAclsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::CreateAclsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DeleteAclsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeConfigsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterConfigsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterReplicaLogDirsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeLogDirsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::SaslAuthenticateResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::CreatePartitionsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::CreateDelegationTokenResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::RenewDelegationTokenResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ExpireDelegationTokenResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeDelegationTokenResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DeleteGroupsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ElectLeadersResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::IncrementalAlterConfigsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterPartitionReassignmentsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListPartitionReassignmentsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::OffsetDeleteResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeClientQuotasResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterClientQuotasResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeUserScramCredentialsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterUserScramCredentialsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::VoteResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::BeginQuorumEpochResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::EndQuorumEpochResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeQuorumResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AlterPartitionResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::UpdateFeaturesResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::EnvelopeResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::FetchSnapshotResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeClusterResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeProducersResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::BrokerRegistrationResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::BrokerHeartbeatResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::UnregisterBrokerResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeTransactionsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListTransactionsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AllocateProducerIdsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ConsumerGroupHeartbeatResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ConsumerGroupDescribeResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ControllerRegistrationResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::GetTelemetrySubscriptionsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::PushTelemetryResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::AssignReplicasToDirsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::ListClientMetricsResourcesResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
            KafkaResponse::DescribeTopicPartitionsResponse(inner) => {
                encode_and_write_response!(proto_info, inner, writer)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::{KafkaProtocolError, KafkaResponse};
    use crate::server::all_api_versions;
    use kafka_protocol::messages::ApiVersionsResponse;
    use kafka_protocol::protocol::Encodable;
    use proto_tower_util::debug::debug_hex;
    use proto_tower_util::WriteTo;
    use tokio::io::{simplex, AsyncReadExt};

    #[tokio::test]
    async fn test_api_version_response() {
        if false {
            //delete
            let mut d = bytes::BytesMut::new();
            let a = ApiVersionsResponse::default().encode(&mut d, 3);
        }
        let d = &[
            0x00, 0x00, 0x01, 0xc8, // 0x00 Size
            0x00, 0x00, 0x00, 0x01, // 0x04 Correlation ID
            0x00, 0x00, // 0x08 Error Code
            0x3f, // 0x0a Array size (Compact array unsigned var int (u32) < 0x80)
            0x00, 0x00, // 0x0b API Key - Versions
            0x00, 0x00, // 0x0d Min version
            0x00, 0x0b, // 0x0f Max version
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x04, 0x00, 0x00,
            0x00, 0x07, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x08,
            0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x09, 0x00,
            0x00, 0x0c, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00,
            0x05, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x13, 0x00,
            0x00, 0x00, 0x07, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00,
            0x17, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x04,
            0x00, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x1e, 0x00, 0x00,
            0x00, 0x03, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x22,
            0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x25, 0x00, 0x00, 0x00, 0x03, 0x00,
            0x00, 0x26, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x29, 0x00, 0x00, 0x00,
            0x03, 0x00, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x2b, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x2d, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x31, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x03,
            0x00, 0x00, 0x39, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x3a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x3d, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x43, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];

        let resp = KafkaResponse::ApiVersionsResponse(ApiVersionsResponse::default().with_api_keys(all_api_versions()))
            .into_tracked(1)
            .into_inner(3);
        let (mut read, mut write) = simplex(1024);
        let r: Result<(), KafkaProtocolError<()>> = resp.write_to(&mut write).await;
        r.unwrap();
        let mut buf = vec![0u8; 1024];
        let sz = read.read(&mut buf).await.unwrap();
        assert_eq!(debug_hex(&buf[..sz]), debug_hex(d),);
    }
}
