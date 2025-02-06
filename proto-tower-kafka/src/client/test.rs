use crate::client::make_layer::ProtoKafkaClientMakeLayer;
use crate::client::KafkaProtoClientConfig;
use crate::data::KafkaRequest;
use kafka_protocol::messages::ApiVersionsRequest;
use proto_tower_util::TestIoService;
use tower::{Service, ServiceBuilder};

const API_VERSIONS_RESPONSE_RAW: &[u8] = &[
    0x00, 0x00, 0x01, 0xc8, // Size
    0x00, 0x00, 0x00, 0x01, // Correlation ID
    0x00, 0x00, // Error code
    0x3f, // Compact array size (0x3f == 63)
    // Array
    0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x0c,
    0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x0b, 0x00, 0x00,
    0x00, 0x09, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x0f, 0x00,
    0x00, 0x00, 0x05, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x13,
    0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00,
    0x17, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x04, 0x00,
    0x00, 0x1b, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x03,
    0x00, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x25, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x26, 0x00, 0x00,
    0x00, 0x03, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x29, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x2a, 0x00,
    0x00, 0x00, 0x02, 0x00, 0x00, 0x2b, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x31, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x39, 0x00, 0x00, 0x00, 0x01, 0x00,
    0x00, 0x3a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x42, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x43, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[tokio::test]
async fn test_client_raw() {
    let raw_i32 = 1_056_964_608i32;
    let la_data: [u8; 4] = raw_i32.to_le_bytes();
    eprintln!("la_data: {:?}", la_data);
    // assert_eq!(la_data, [0x01, 0x00, 0x00, 0x3f]);
    let test_net = TestIoService::new(Vec::from(API_VERSIONS_RESPONSE_RAW));
    let mut service = ServiceBuilder::default()
        .layer(ProtoKafkaClientMakeLayer::new(
            rand::rngs::OsRng::default(),
            KafkaProtoClientConfig {
                timeout: Default::default(),
                client_id: None,
            },
        ))
        .service(test_net.clone());

    let ((rx_svc, sx_svc), (mut rx, sx)) = proto_tower_util::sx_rx_chans();
    let task = tokio::spawn(service.call((rx_svc, sx_svc)));
    sx.send(KafkaRequest::ApiVersionsRequest(Box::new(
        ApiVersionsRequest::default()
            .with_client_software_name("test-client".into())
            .with_client_software_version("1.2.3".into()),
    )))
    .await
    .unwrap();
    let resp = rx.recv().await;
    drop(sx);
    drop(rx);
    task.await.unwrap().unwrap();
    let resp = resp.unwrap();
}
