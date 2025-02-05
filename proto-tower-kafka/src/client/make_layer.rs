use crate::client::layer::ProtoKafkaClientLayer;
use crate::client::KafkaProtoClientConfig;
use std::fmt::Debug;
use tokio::io::{ReadHalf, SimplexStream, WriteHalf};
use tower::Layer;

pub struct ProtoKafkaClientMakeLayer<RNG>
where
    RNG: rand::TryRngCore + Send + Clone + 'static,
{
    pub config: KafkaProtoClientConfig,
    pub rng: RNG,
}

impl<RNG> ProtoKafkaClientMakeLayer<RNG>
where
    RNG: rand::TryRngCore + Send + Clone + 'static,
{
    pub fn new(rng: RNG, config: KafkaProtoClientConfig) -> Self {
        ProtoKafkaClientMakeLayer { config, rng }
    }
}

impl<Service, E, RNG> Layer<Service> for ProtoKafkaClientMakeLayer<RNG>
where
    Service: tower::Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
    RNG: rand::TryRngCore + Send + Clone + 'static,
{
    type Service = ProtoKafkaClientLayer<Service, E, RNG>;

    fn layer(&self, inner: Service) -> Self::Service {
        ProtoKafkaClientLayer::new(inner, self.rng.clone(), self.config.clone())
    }
}
