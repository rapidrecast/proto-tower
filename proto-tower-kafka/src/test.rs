use crate::client::make_layer::ProtoKafkaClientMakeLayer;
use crate::data::{KafkaRequest, KafkaResponse};
use crate::server::make_layer::ProtoKafkaServerMakeLayer;
use crate::server::test::MockKafkaService;
use crate::server::KafkaProtoServerConfig;
use kafka_protocol::messages::ApiVersionsRequest;
use kafka_protocol::protocol::StrBytes;
use tower::{Layer, Service, ServiceBuilder};

#[tokio::test]
async fn test_client() {
    let mut client = ServiceBuilder::new()
        .layer(ProtoKafkaClientMakeLayer::new())
        .layer(ProtoKafkaServerMakeLayer::new(KafkaProtoServerConfig { timeout: Default::default() }))
        .service(MockKafkaService::new(vec![]));
    let ((read_svc, write_svc), (mut read, write)) = proto_tower_util::sx_rx_chans::<KafkaRequest, KafkaResponse>();
    let task = tokio::spawn(client.call((read_svc, write_svc)));

    write
        .send(KafkaRequest::ApiVersionsRequest(
            ApiVersionsRequest::default().with_client_software_name(StrBytes::from("test-client")),
        ))
        .await
        .unwrap();
    let res = read.recv().await.unwrap();
    assert_eq!(res, KafkaResponse::ApiVersionsResponse(Default::default()));
    task.await.unwrap().unwrap();
}
