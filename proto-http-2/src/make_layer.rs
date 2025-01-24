use crate::layer::ProtoH2CLayer;
use crate::parser::Http2Frame;
use crate::ProtoHttp2Config;
use std::marker::PhantomData;
use tokio::sync::mpsc::{Receiver, Sender};
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1MakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = ()> + Send + Clone,
{
    phantom_data: PhantomData<SERVICE>,
    config: ProtoHttp2Config,
}

impl<SERVICE> ProtoHttp1MakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the layer
    pub fn new(config: ProtoHttp2Config) -> Self {
        ProtoHttp1MakeLayer {
            phantom_data: PhantomData,
            config,
        }
    }
}

impl<SERVICE> Layer<SERVICE> for ProtoHttp1MakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = ()> + Send + Clone,
{
    type Service = ProtoH2CLayer<SERVICE>;

    fn layer(&self, inner: SERVICE) -> Self::Service {
        ProtoH2CLayer::new(self.config.clone(), inner)
    }
}
