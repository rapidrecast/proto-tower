use crate::client::layer::ProtoHttp1ClientLayer;
use crate::client::ProtoHttp1ClientConfig;
use std::marker::PhantomData;
use tokio::io::DuplexStream;
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1ClientMakeLayer<Svc>
where
    Svc: Service<(DuplexStream, DuplexStream), Response = ()> + Send + Clone,
{
    config: ProtoHttp1ClientConfig,
    phantom_service: PhantomData<Svc>,
}

impl<Svc> ProtoHttp1ClientMakeLayer<Svc>
where
    Svc: Service<(DuplexStream, DuplexStream), Response = ()> + Send + Clone,
{
    /// Create a new instance of the layer
    pub fn new(config: ProtoHttp1ClientConfig) -> Self {
        ProtoHttp1ClientMakeLayer {
            phantom_service: PhantomData,
            config,
        }
    }
}

impl<Svc> Layer<Svc> for ProtoHttp1ClientMakeLayer<Svc>
where
    Svc: Service<(DuplexStream, DuplexStream), Response = ()> + Send + Clone,
{
    type Service = ProtoHttp1ClientLayer<Svc>;

    fn layer(&self, inner: Svc) -> Self::Service {
        ProtoHttp1ClientLayer::new(self.config.clone(), inner)
    }
}
