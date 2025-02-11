use crate::data::inner_response::{TrackedKafkaRequest, TrackedKafkaResponse};
use crate::data::{KafkaRequest, KafkaResponse};
use crate::server::make_layer::ProtoKafkaServerMakeLayer;
use crate::server::KafkaProtoServerConfig;
use bytes::BytesMut;
use kafka_protocol::messages::{ApiVersionsRequest, ApiVersionsResponse};
use kafka_protocol::protocol::{Decodable, StrBytes};
use proto_tower_util::debug::debug_hex;
use proto_tower_util::{AsyncReadToBuf, ZeroReadBehaviour};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_rdkafka() {
    let (task, port, kafka_service) = bind_and_serve(Some(39092)).await;
    // let port = 29092;
    let producer: FutureProducer = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", format!("localhost:{port}"))
        .set("message.timeout.ms", "5000")
        .create()
        .unwrap();

    let client_res = producer
        .send(
            FutureRecord::<str, str>::to("my-topic").key("some key").payload("hello"),
            Timeout::After(Duration::from_secs(1)),
        )
        .await;

    task.abort();

    let requests = kafka_service.requests.lock().await;
    assert_eq!(
        *requests,
        vec![TrackedKafkaRequest {
            correlation_id: 1,
            request: KafkaRequest::ApiVersionsRequest(
                ApiVersionsRequest::default()
                    .with_client_software_version(StrBytes::from("2.3.0"))
                    .with_client_software_name(StrBytes::from("librdkafka"))
            ),
        },]
    );
    let responses = kafka_service.responses.lock().await;
    assert_eq!(responses.len(), 0);
    client_res.unwrap();
}

#[tokio::test]
async fn test_raw() {
    let mock_kafka_service = MockKafkaService::new(vec![TrackedKafkaResponse {
        correlation_id: 1,
        response: KafkaResponse::ApiVersionsResponse(ApiVersionsResponse::default()),
    }]);
    let mut kafka_service = ServiceBuilder::new()
        .layer(ProtoKafkaServerMakeLayer::new(KafkaProtoServerConfig {
            timeout: Duration::from_millis(2000),
        }))
        .service(mock_kafka_service.clone());
    let (read, write_svc) = tokio::io::duplex(1024);
    let (read_svc, mut write) = tokio::io::split(read);
    let (mut read, write_svc) = tokio::io::split(write_svc);
    let task = tokio::spawn(async move {
        kafka_service.call((read_svc, write_svc)).await.unwrap();
    });
    let payload = [
        0x00u8, 0x00, 0x00, 0x24, 0x00, 0x12, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x07, 0x72, 0x64, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x00, 0x0b, 0x6c, 0x69, 0x62,
        0x72, 0x64, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x06, 0x32, 0x2e, 0x33, 0x2e, 0x30, 0x00,
    ];
    write.write_all(&payload).await.unwrap();
    let reader = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let mut buf = reader.read_with_timeout(&mut read, Duration::from_secs(1), None).await;
    assert!(!buf.is_empty());
    let sz = buf.drain(0..4).map(|b| b as usize).fold(0, |acc, x| acc * 256 + x);
    assert_eq!(buf.len(), sz);
    let mut byte_buf = BytesMut::new();
    byte_buf.extend_from_slice(&buf);
    eprintln!("Decoding:\n{}", debug_hex(&byte_buf));
    let _res = ApiVersionsResponse::decode(&mut byte_buf, 3).unwrap();
    assert_eq!(&buf, &[0x00, 0xff]);
    task.await.unwrap();
}

async fn bind_and_serve(port: Option<u16>) -> (JoinHandle<()>, u16, MockKafkaService) {
    let port = port.unwrap_or_default();
    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let kafka_service = MockKafkaService::new(vec![TrackedKafkaResponse {
        correlation_id: 1,
        response: KafkaResponse::ApiVersionsResponse(ApiVersionsResponse::default()),
    }]);
    let inner_kafka_service = kafka_service.clone();
    let task = tokio::spawn(async move {
        let (stream, _addr) = listener.accept().await.unwrap();
        let (read, write) = stream.into_split();
        let mut svc = ServiceBuilder::new()
            .layer(ProtoKafkaServerMakeLayer::new(KafkaProtoServerConfig {
                timeout: Duration::from_millis(2000),
            }))
            .service(inner_kafka_service);
        svc.call((read, write)).await.unwrap();
    });
    (task, port, kafka_service)
}

#[derive(Clone)]
pub struct MockKafkaService {
    requests: Arc<Mutex<Vec<TrackedKafkaRequest>>>,
    responses: Arc<Mutex<Vec<TrackedKafkaResponse>>>,
}

impl Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>)> for MockKafkaService {
    type Response = ();
    type Error = ();
    type Future = Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, (receiver, sender): (Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>)) -> Self::Future {
        Box::pin(Self::inner_call((receiver, sender), self.requests.clone(), self.responses.clone()))
    }
}

impl MockKafkaService {
    pub fn new(responses: Vec<TrackedKafkaResponse>) -> Self {
        MockKafkaService {
            requests: Arc::new(Mutex::new(Vec::new())),
            responses: Arc::new(Mutex::new(responses)),
        }
    }
    async fn inner_call(
        (mut receiver, sender): (Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>),
        requests: Arc<Mutex<Vec<TrackedKafkaRequest>>>,
        responses: Arc<Mutex<Vec<TrackedKafkaResponse>>>,
    ) -> Result<(), ()> {
        while let Some(request) = receiver.recv().await {
            let mut requests_lock = requests.lock().await;
            requests_lock.push(request);
            let mut responses_lock = responses.lock().await;
            if let Some(response) = responses_lock.pop() {
                sender.send(response).await.unwrap();
            } else {
                break;
            }
        }
        Ok(())
    }
}
