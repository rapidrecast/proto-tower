use crate::client::make_layer::ProtoKafkaClientMakeLayer;
use crate::client::KafkaProtoClientConfig;
use crate::data::{KafkaRequest, KafkaResponse};
use crate::server::make_layer::ProtoKafkaServerMakeLayer;
use crate::server::test::MockKafkaService;
use crate::server::KafkaProtoServerConfig;
use kafka_protocol::messages::{ApiVersionsRequest, ApiVersionsResponse};
use kafka_protocol::protocol::StrBytes;
use rand::rngs::OsRng;
use std::time::Duration;
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_client() {
    let mut client = ServiceBuilder::new()
        .layer(ProtoKafkaClientMakeLayer::new(
            OsRng::default(),
            KafkaProtoClientConfig {
                timeout: Duration::from_millis(200),
                client_id: None,
            },
        ))
        .layer(proto_tower_util::DebugIoLayer {})
        .layer(ProtoKafkaServerMakeLayer::new(KafkaProtoServerConfig {
            timeout: Duration::from_millis(200),
        }))
        .service(MockKafkaService::new(vec![KafkaResponse::ApiVersionsResponse(Box::new(
            ApiVersionsResponse::default(),
        ))]));
    let ((read_svc, write_svc), (mut read, write)) = proto_tower_util::sx_rx_chans::<KafkaRequest, KafkaResponse>();
    let task = tokio::spawn(client.call((read_svc, write_svc)));

    write
        .send(KafkaRequest::ApiVersionsRequest(Box::new(
            ApiVersionsRequest::default()
                .with_client_software_name(StrBytes::from("test-client"))
                .with_client_software_version(StrBytes::from("2.3.0")),
        )))
        .await
        .unwrap();
    // Set some value that is incorrect but we will change
    let res = tokio::time::timeout(Duration::from_secs(2), read.recv()).await.unwrap();
    let res = res.unwrap();

    drop(write);
    drop(read);
    assert_eq!(res, KafkaResponse::ApiVersionsResponse(Default::default()));
    task.await.unwrap().unwrap();
}
