use crate::client::layer::ProtoHttp1ClientLayer;
use crate::client::ProtoHttp1ClientConfig;
use std::marker::PhantomData;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1ClientMakeLayer<Svc, Reader, Writer>
where
    Svc: Service<(Reader, Writer), Response = ()> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    config: ProtoHttp1ClientConfig,
    phantom_service: PhantomData<Svc>,
    phantom_reader: PhantomData<Reader>,
    phantom_writer: PhantomData<Writer>,
}

impl<Svc, Reader, Writer> ProtoHttp1ClientMakeLayer<Svc, Reader, Writer>
where
    Svc: Service<(Reader, Writer), Response = ()> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    /// Create a new instance of the layer
    pub fn new(config: ProtoHttp1ClientConfig) -> Self {
        ProtoHttp1ClientMakeLayer {
            phantom_service: PhantomData,
            config,
            phantom_reader: PhantomData,
            phantom_writer: PhantomData,
        }
    }
}

impl<Svc, Reader, Writer> Layer<Svc> for ProtoHttp1ClientMakeLayer<Svc, Reader, Writer>
where
    Svc: Service<(Reader, Writer), Response = ()> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    type Service = ProtoHttp1ClientLayer<Svc, Reader, Writer>;

    fn layer(&self, inner: Svc) -> Self::Service {
        ProtoHttp1ClientLayer::new(self.config.clone(), inner)
    }
}
