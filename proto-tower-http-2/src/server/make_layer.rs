use crate::server::layer::ProtoHttp2Layer;
use crate::server::parser::Http2Frame;
use crate::ProtoHttp2Config;
use std::marker::PhantomData;
use tokio::sync::mpsc::{Receiver, Sender};
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp2MakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = ()> + Send + Clone,
{
    phantom_data: PhantomData<SERVICE>,
    config: ProtoHttp2Config,
}

impl<SERVICE> ProtoHttp2MakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the layer
    pub fn new(config: ProtoHttp2Config) -> Self {
        ProtoHttp2MakeLayer {
            phantom_data: PhantomData,
            config,
        }
    }
}

impl<SERVICE> Layer<SERVICE> for ProtoHttp2MakeLayer<SERVICE>
where
    SERVICE: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = ()> + Send + Clone,
{
    type Service = ProtoHttp2Layer<SERVICE>;

    fn layer(&self, inner: SERVICE) -> Self::Service {
        ProtoHttp2Layer::new(self.config.clone(), inner)
    }
}
