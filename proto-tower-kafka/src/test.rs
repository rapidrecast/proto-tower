use crate::client::make_layer::ProtoKafkaClientMakeLayer;
use crate::client::KafkaProtoClientConfig;
use crate::data::{KafkaRequest, KafkaResponse};
use crate::server::make_layer::ProtoKafkaServerMakeLayer;
use crate::server::test::MockKafkaService;
use crate::server::KafkaProtoServerConfig;
use kafka_protocol::messages::{ApiVersionsRequest, ApiVersionsResponse};
use kafka_protocol::protocol::StrBytes;
use std::time::Duration;
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_client() {
    let mut client = ServiceBuilder::new()
        .layer(ProtoKafkaClientMakeLayer::new(KafkaProtoClientConfig {
            timeout: Duration::from_millis(200),
            client_id: None,
        }))
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
    let mut res: KafkaResponse = KafkaResponse::ApiVersionsResponse(Default::default());
    tokio::select! {
        result = read.recv() => {
            match result {
                None => {
                    // It looks like the task failed
                    task.await.unwrap().unwrap();
                    panic!("The receiver was closed but the task succeeded?")
                }
                Some(r) => {
                    res = r;
                }
            }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
            if task.is_finished() {
                task.await.unwrap().unwrap();
            }
            panic!("Timeout")
        }
    }
    drop(write);
    drop(read);
    assert_eq!(res, KafkaResponse::ApiVersionsResponse(Default::default()));
    task.await.unwrap().unwrap();
}
