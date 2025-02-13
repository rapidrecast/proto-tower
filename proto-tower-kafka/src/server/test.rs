use crate::data::inner_response::{TrackedKafkaRequest, TrackedKafkaResponse};
use crate::data::{KafkaRequest, KafkaResponse};
use crate::server::make_layer::ProtoKafkaServerLayer;
use crate::server::{all_api_versions, KafkaProtoServerConfig};
use bytes::{Bytes, BytesMut};
use kafka_protocol::messages::metadata_request::MetadataRequestTopic;
use kafka_protocol::messages::metadata_response::{MetadataResponseBroker, MetadataResponsePartition, MetadataResponseTopic};
use kafka_protocol::messages::produce_request::{PartitionProduceData, TopicProduceData};
use kafka_protocol::messages::produce_response::{PartitionProduceResponse, TopicProduceResponse};
use kafka_protocol::messages::{ApiVersionsRequest, ApiVersionsResponse, BrokerId, MetadataRequest, MetadataResponse, ProduceRequest, ProduceResponse, TopicName};
use kafka_protocol::protocol::StrBytes;
use kafka_protocol::records::{Compression, Record, RecordBatchDecoder, RecordBatchEncoder, RecordEncodeOptions, TimestampType};
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
                MetadataResponse::default()
                    .with_brokers(vec![MetadataResponseBroker::default()
                        .with_node_id(BrokerId(1))
                        .with_host(StrBytes::from("localhost"))
                        .with_port(39092)])
                    .with_cluster_id(Some(StrBytes::from("q8L0jMpRTAa_LOItNbMVZg")))
                    .with_controller_id(BrokerId(1))
                    .with_topics(vec![MetadataResponseTopic::default()
                        .with_name(Some(TopicName(StrBytes::from("my-topic"))))
                        .with_topic_id(uuid::Uuid::parse_str("b1fa72a9-70f1-4e91-b26f-e3e2111c7251").unwrap())
                        .with_partitions(vec![MetadataResponsePartition::default()
                            .with_leader_id(BrokerId(1))
                            .with_replica_nodes(vec![BrokerId(1)])
                            .with_leader_epoch(0)
                            .with_isr_nodes(vec![BrokerId(1)])])]),
            ),
        },
        // TrackedKafkaResponse {
        //     correlation_id: 3,
        //     response: KafkaResponse::MetadataResponse(
        //         MetadataResponse::default()
        //             .with_brokers(vec![MetadataResponseBroker::default().with_host(StrBytes::from("localhost")).with_port(39092)])
        //             .with_topics(vec![MetadataResponseTopic::default().with_name(Some(TopicName::from(StrBytes::from("my-topic"))))]),
        //     ),
        // },
        TrackedKafkaResponse {
            correlation_id: 3,
            response: KafkaResponse::ProduceResponse(ProduceResponse::default().with_responses(vec![
                TopicProduceResponse::default().with_name(TopicName::from(StrBytes::from("my-topic"))).with_partition_responses(vec![
                    PartitionProduceResponse::default().with_base_offset(0)
                        .with_log_append_time_ms(-1)
                ]),
            ])),
        },
    ]);
    let (task, port) = bind_and_serve(BindPort::UsingPort(39092), kafka_service.clone()).await;
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
    assert_eq!(requests.len(), 3);
    assert_eq!(
        requests.iter().cloned().take(2).collect::<Vec<_>>(),
        vec![
            TrackedKafkaRequest {
                correlation_id: 1,
                request: KafkaRequest::ApiVersionsRequest(
                    ApiVersionsRequest::default()
                        .with_client_software_version(StrBytes::from("2.3.0"))
                        .with_client_software_name(StrBytes::from("librdkafka"))
                ),
            },
            TrackedKafkaRequest {
                correlation_id: 2,
                request: KafkaRequest::MetadataRequest(
                    MetadataRequest::default().with_topics(Some(vec![MetadataRequestTopic::default().with_name(Some(TopicName(StrBytes::from("my-topic"))))])),
                ),
            },
        ]
    );
    if false {
        assert_eq!(
            requests.iter().skip(2).cloned().next().unwrap(),
            TrackedKafkaRequest {
                correlation_id: 3,
                request: KafkaRequest::ProduceRequest(
                    ProduceRequest::default()
                        .with_acks(-1)
                        .with_timeout_ms(30000)
                        .with_topic_data(vec![TopicProduceData::default()
                            .with_name(TopicName(StrBytes::from("my-topic")))
                            .with_partition_data(vec![PartitionProduceData::default().with_records(Some(encoded_records(
                                &[Record {
                                    transactional: false,
                                    control: false,
                                    partition_leader_epoch: 0,
                                    producer_id: -1,
                                    producer_epoch: -1,
                                    timestamp_type: TimestampType::Creation,
                                    offset: 0,
                                    sequence: -1,
                                    timestamp: 1739412128732, // TODO fuuuucked
                                    key: Some(Bytes::from(StrBytes::from("some key"))),
                                    value: Some(Bytes::from(StrBytes::from("hello"))),
                                    headers: Default::default(),
                                },],
                                &RecordEncodeOptions {
                                    version: 2,
                                    compression: Compression::None,
                                }
                            )))])])
                ),
            },
        );
    }
    let responses = kafka_service.responses.lock().await;
    assert_eq!(responses.len(), 0);
    client_res.unwrap();
}

#[test]
fn test_record_data() {
    let d  = b"\0\0\0\0\0\0\0\0\0\0\0E\0\0\0\0\x02Q\xd2\xe1\xa3\0\0\0\0\0\0\0\0\x01\x94\xfd\n\xc3\xdc\0\0\x01\x94\xfd\n\xc3\xdc\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\0\0\0\x01&\0\0\0\x10some key\nhello\0";
    let mut buff = BytesMut::from(&d[..]);
    let recs = RecordBatchDecoder::decode(&mut buff).unwrap();
    assert_eq!(recs, vec![])
}

fn encoded_records(records: &[Record], options: &RecordEncodeOptions) -> Bytes {
    let mut buf_mut = BytesMut::new();
    RecordBatchEncoder::encode(&mut buf_mut, records, options).unwrap();
    buf_mut.freeze()
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
