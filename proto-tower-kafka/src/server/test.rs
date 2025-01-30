use crate::data::{KafkaRequest, KafkaResponse};
use crate::server::make_layer::ProtoKafkaServerMakeLayer;
use crate::server::KafkaProtoServerConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_rdkafka() {
    let (task, port, kafka_service) = bind_and_serve(None).await;
    let producer: FutureProducer = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", format!("localhost:{port}"))
        .set("message.timeout.ms", "5000")
        .create()
        .unwrap();

    producer
        .send(
            FutureRecord::<str, str>::to("my-topic").key("some key").payload("hello"),
            Timeout::After(Duration::from_secs(1)),
        )
        .await
        .unwrap();

    task.abort();

    let requests = kafka_service.requests.lock().await;
    assert_eq!(*requests, vec![]);
    let responses = kafka_service.responses.lock().await;
    assert_eq!(responses.len(), 0);
}

async fn bind_and_serve(port: Option<u16>) -> (JoinHandle<()>, u16, MockKafkaService) {
    let port = port.unwrap_or_default();
    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let kafka_service = MockKafkaService::new(vec![]);
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
    requests: Arc<Mutex<Vec<KafkaRequest>>>,
    responses: Arc<Mutex<Vec<KafkaResponse>>>,
}

impl Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>)> for MockKafkaService {
    type Response = ();
    type Error = ();
    type Future = Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, (receiver, sender): (Receiver<KafkaRequest>, Sender<KafkaResponse>)) -> Self::Future {
        Box::pin(Self::inner_call((receiver, sender), self.requests.clone(), self.responses.clone()))
    }
}

impl MockKafkaService {
    fn new(responses: Vec<KafkaResponse>) -> Self {
        MockKafkaService {
            requests: Arc::new(Mutex::new(Vec::new())),
            responses: Arc::new(Mutex::new(responses)),
        }
    }
    async fn inner_call(
        (mut receiver, sender): (Receiver<KafkaRequest>, Sender<KafkaResponse>),
        requests: Arc<Mutex<Vec<KafkaRequest>>>,
        responses: Arc<Mutex<Vec<KafkaResponse>>>,
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
