use crate::layer::{Http2Request, Http2Response, ProtoH2CLayer, ProtoHttp2Config};
use crate::{Http2Request, Http2Response, ProtoHttp2Config};
use std::marker::PhantomData;
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1MakeLayer<SERVICE>
where
    SERVICE: Service<Http2Request, Response=Http2Response> + Send + Clone,
{
    phantom_data: PhantomData<SERVICE>,
    config: ProtoHttp2Config,
}

impl<SERVICE> ProtoHttp1MakeLayer<SERVICE>
where
    SERVICE: Service<Http2Request, Response=Http2Response> + Send + Clone,
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
    SERVICE: Service<Http2Request, Response=Http2Response> + Send + Clone,
{
    type Service = ProtoH2CLayer<SERVICE>;

    fn layer(&self, inner: SERVICE) -> Self::Service {
        ProtoH2CLayer::new(self.config.clone(), inner)
    }
}
