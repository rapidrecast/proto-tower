use crate::client::layer::ProtoKafkaClientLayer;
use crate::client::KafkaProtoClientConfig;
use std::fmt::Debug;
use tokio::io::{ReadHalf, SimplexStream, WriteHalf};
use tower::Layer;

pub struct ProtoKafkaClientMakeLayer {
    pub config: KafkaProtoClientConfig,
}

impl ProtoKafkaClientMakeLayer {
    pub fn new(config: KafkaProtoClientConfig) -> Self {
        ProtoKafkaClientMakeLayer { config }
    }
}

impl<Service, E> Layer<Service> for ProtoKafkaClientMakeLayer
where
    Service: tower::Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
{
    type Service = ProtoKafkaClientLayer<Service, E>;

    fn layer(&self, inner: Service) -> Self::Service {
        ProtoKafkaClientLayer::new(inner, self.config.clone())
    }
}
