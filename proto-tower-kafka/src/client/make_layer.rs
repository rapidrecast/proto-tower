use crate::client::layer::ProtoKafkaClientLayer;
use std::fmt::Debug;
use tokio::io::{ReadHalf, SimplexStream, WriteHalf};
use tower::Layer;

pub struct ProtoKafkaClientMakeLayer {}

impl ProtoKafkaClientMakeLayer {
    pub fn new() -> Self {
        ProtoKafkaClientMakeLayer {}
    }
}

impl<Service, E> Layer<Service> for ProtoKafkaClientMakeLayer
where
    Service: tower::Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = E> + Send + Clone + 'static,
    E: Debug + Send + 'static,
{
    type Service = ProtoKafkaClientLayer<Service, E>;

    fn layer(&self, inner: Service) -> Self::Service {
        ProtoKafkaClientLayer::new(inner)
    }
}
