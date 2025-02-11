use crate::data::inner_response::{TrackedKafkaRequest, TrackedKafkaResponse};
use crate::server::layer::ProtoKafkaServerLayer;
use crate::server::KafkaProtoServerConfig;
use std::marker::PhantomData;
use tokio::sync::mpsc::{Receiver, Sender};
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoKafkaServerMakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = ()> + Send + Clone,
{
    phantom_data: PhantomData<SERVICE>,
    config: KafkaProtoServerConfig,
}

impl<SERVICE> ProtoKafkaServerMakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the layer
    pub fn new(config: KafkaProtoServerConfig) -> Self {
        ProtoKafkaServerMakeLayer {
            phantom_data: PhantomData,
            config,
        }
    }
}

impl<SERVICE> Layer<SERVICE> for ProtoKafkaServerMakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = ()> + Send + Clone,
{
    type Service = ProtoKafkaServerLayer<SERVICE>;

    fn layer(&self, inner: SERVICE) -> Self::Service {
        ProtoKafkaServerLayer::new(self.config.clone(), inner)
    }
}
