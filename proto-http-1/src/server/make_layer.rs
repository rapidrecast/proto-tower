use crate::data::{HTTP1ServerEvent, Http1ServerResponseEvent};
use crate::server::layer::ProtoHttp1ServerLayer;
use crate::server::ProtoHttp1ServerConfig;
use std::marker::PhantomData;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1ServerMakeLayer<Svc, Reader, Writer>
where
    Svc: Service<HTTP1ServerEvent<Reader, Writer>, Response = Http1ServerResponseEvent> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    config: ProtoHttp1ServerConfig,
    phantom_service: PhantomData<Svc>,
    phantom_reader: PhantomData<Reader>,
    phantom_writer: PhantomData<Writer>,
}

impl<Svc, Reader, Writer> ProtoHttp1ServerMakeLayer<Svc, Reader, Writer>
where
    Svc: Service<HTTP1ServerEvent<Reader, Writer>, Response = Http1ServerResponseEvent> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    /// Create a new instance of the layer
    pub fn new(config: ProtoHttp1ServerConfig) -> Self {
        ProtoHttp1ServerMakeLayer {
            phantom_service: PhantomData,
            config,
            phantom_reader: PhantomData,
            phantom_writer: PhantomData,
        }
    }
}

impl<Svc, Reader, Writer> Layer<Svc> for ProtoHttp1ServerMakeLayer<Svc, Reader, Writer>
where
    Svc: Service<HTTP1ServerEvent<Reader, Writer>, Response = Http1ServerResponseEvent> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    type Service = ProtoHttp1ServerLayer<Svc, Reader, Writer>;

    fn layer(&self, inner: Svc) -> Self::Service {
        ProtoHttp1ServerLayer::new(self.config.clone(), inner)
    }
}
