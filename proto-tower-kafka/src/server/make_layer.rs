use crate::data::inner_response::{TrackedKafkaRequest, TrackedKafkaResponse};
use crate::server::layer::ProtoKafkaServerService;
use crate::server::KafkaProtoServerConfig;
use std::marker::PhantomData;
use tokio::sync::mpsc::{Receiver, Sender};
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoKafkaServerLayer<SERVICE>
where
    SERVICE: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = ()> + Send + Clone,
{
    phantom_data: PhantomData<SERVICE>,
    config: KafkaProtoServerConfig,
}

impl<SERVICE> ProtoKafkaServerLayer<SERVICE>
where
    SERVICE: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the layer
    pub fn new(config: KafkaProtoServerConfig) -> Self {
        ProtoKafkaServerLayer {
            phantom_data: PhantomData,
            config,
        }
    }
}

impl<SERVICE> Layer<SERVICE> for ProtoKafkaServerLayer<SERVICE>
where
    SERVICE: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = ()> + Send + Clone,
{
    type Service = ProtoKafkaServerService<SERVICE>;

    fn layer(&self, inner: SERVICE) -> Self::Service {
        ProtoKafkaServerService::new(self.config.clone(), inner)
    }
}
