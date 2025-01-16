use crate::layer::{HTTP1Request, HTTP1Response, ProtoHttp1Layer};
use std::marker::PhantomData;
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1MakeLayer<SERVICE>
where
    SERVICE: Service<HTTP1Request, Response = HTTP1Response> + Send + Clone,
{
    phantom_data: PhantomData<SERVICE>,
}

impl<SERVICE> ProtoHttp1MakeLayer<SERVICE>
where
    SERVICE: Service<HTTP1Request, Response = HTTP1Response> + Send + Clone,
{
    /// Create a new instance of the layer
    pub fn new() -> Self {
        ProtoHttp1MakeLayer {
            phantom_data: PhantomData,
        }
    }
}

impl<SERVICE> Layer<SERVICE> for ProtoHttp1MakeLayer<SERVICE>
where
    SERVICE: Service<HTTP1Request, Response = HTTP1Response> + Send + Clone,
{
    type Service = ProtoHttp1Layer<SERVICE>;

    fn layer(&self, inner: SERVICE) -> Self::Service {
        ProtoHttp1Layer::new(inner)
    }
}
