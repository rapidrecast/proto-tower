use crate::data::inner_response::{TrackedKafkaRequest, TrackedKafkaResponse};
use crate::data::{KafkaRequest, KafkaResponse};
use crate::server::make_layer::ProtoKafkaServerLayer;
use crate::server::{all_api_versions, KafkaProtoServerConfig};
use kafka_protocol::messages::metadata_response::MetadataResponseBroker;
use kafka_protocol::messages::{ApiVersionsRequest, ApiVersionsResponse, MetadataResponse};
use kafka_protocol::protocol::StrBytes;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::collections::VecDeque;
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
    let kafka_service = MockKafkaService::new(vec![
        TrackedKafkaResponse {
            correlation_id: 1,
            response: KafkaResponse::ApiVersionsResponse(ApiVersionsResponse::default().with_api_keys(all_api_versions())),
        },
        TrackedKafkaResponse {
            correlation_id: 2,
            response: KafkaResponse::MetadataResponse(
                MetadataResponse::default().with_brokers(vec![MetadataResponseBroker::default().with_host(StrBytes::from("localhost")).with_port(39092)]),
            ),
        },
    ]);
    let (task, port) = bind_and_serve(BindPort::RandomPort, kafka_service.clone()).await;
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

enum BindPort {
    UsingDockerInstead(u16),
    UsingPort(u16),
    RandomPort,
}

async fn bind_and_serve(bind_port: BindPort, kafka_service: MockKafkaService) -> (JoinHandle<()>, u16) {
    let desired_port = match bind_port {
        BindPort::UsingDockerInstead(d) => d,
        BindPort::UsingPort(d) => d,
        BindPort::RandomPort => 0,
    };
    let listener = match bind_port {
        BindPort::UsingDockerInstead(_) => None,
        BindPort::UsingPort(_) | BindPort::RandomPort => Some(TcpListener::bind(("0.0.0.0", desired_port)).await.unwrap()),
    };
    let bound_port = listener.as_ref().map(|l| l.local_addr().unwrap().port());
    let task = tokio::spawn(async move {
        match listener {
            None => loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
            },
            Some(listener) => {
                let (stream, _addr) = listener.accept().await.unwrap();
                let (read, write) = stream.into_split();
                let mut svc = ServiceBuilder::new()
                    .layer(ProtoKafkaServerLayer::new(KafkaProtoServerConfig {
                        timeout: Duration::from_millis(2000),
                    }))
                    .service(kafka_service);
                svc.call((read, write)).await.unwrap();
            }
        }
    });
    (task, bound_port.unwrap_or(desired_port))
}

#[derive(Clone)]
pub struct MockKafkaService {
    requests: Arc<Mutex<Vec<TrackedKafkaRequest>>>,
    responses: Arc<Mutex<VecDeque<TrackedKafkaResponse>>>,
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
            responses: Arc::new(Mutex::new(VecDeque::from(responses))),
        }
    }
    async fn inner_call(
        (mut receiver, sender): (Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>),
        requests: Arc<Mutex<Vec<TrackedKafkaRequest>>>,
        responses: Arc<Mutex<VecDeque<TrackedKafkaResponse>>>,
    ) -> Result<(), ()> {
        eprintln!("Waiting for next message in MockKafkaService");
        while let Some(request) = receiver.recv().await {
            eprintln!("Received request in MockKafkaService: {:?}", request);
            {
                let mut requests_lock = requests.lock().await;
                requests_lock.push(request);
            }
            let mut responses_lock = responses.lock().await;
            if let Some(response) = responses_lock.pop_front() {
                eprintln!("Sending response from MockKafkaService: {:?}", response);
                sender.send(response).await.map_err(|_| ())?;
            } else {
                break;
            }
            eprintln!("Waiting for next message in MockKafkaService");
        }
        eprintln!("Done with messages in MockKafkaService");
        Ok(())
    }
}
