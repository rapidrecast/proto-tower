use crate::client::layer::ProtoKafkaClientService;
use crate::client::KafkaProtoClientConfig;
use std::fmt::Debug;
use tokio::io::{ReadHalf, SimplexStream, WriteHalf};
use tower::Layer;

pub struct ProtoKafkaClientLayer {
    pub config: KafkaProtoClientConfig,
}

impl ProtoKafkaClientLayer {
    pub fn new(config: KafkaProtoClientConfig) -> Self {
        ProtoKafkaClientLayer { config }
    }
}

impl<Service, E> Layer<Service> for ProtoKafkaClientLayer
where
    Service: tower::Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
{
    type Service = ProtoKafkaClientService<Service, E>;

    fn layer(&self, inner: Service) -> Self::Service {
        ProtoKafkaClientService::new(inner, self.config.clone())
    }
}
