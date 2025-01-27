use crate::data::{HTTP1Event, HTTTP1ResponseEvent};
use crate::server::layer::ProtoHttp1Layer;
use crate::server::ProtoHttp1Config;
use std::marker::PhantomData;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1MakeLayer<Svc, Reader, Writer>
where
    Svc: Service<HTTP1Event<Reader, Writer>, Response = HTTTP1ResponseEvent> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    config: ProtoHttp1Config,
    phantom_service: PhantomData<Svc>,
    phantom_reader: PhantomData<Reader>,
    phantom_writer: PhantomData<Writer>,
}

impl<Svc, Reader, Writer> ProtoHttp1MakeLayer<Svc, Reader, Writer>
where
    Svc: Service<HTTP1Event<Reader, Writer>, Response = HTTTP1ResponseEvent> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    /// Create a new instance of the layer
    pub fn new(config: ProtoHttp1Config) -> Self {
        ProtoHttp1MakeLayer {
            phantom_service: PhantomData,
            config,
            phantom_reader: PhantomData,
            phantom_writer: PhantomData,
        }
    }
}

impl<Svc, Reader, Writer> Layer<Svc> for ProtoHttp1MakeLayer<Svc, Reader, Writer>
where
    Svc: Service<HTTP1Event<Reader, Writer>, Response = HTTTP1ResponseEvent> + Send + Clone,
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    type Service = ProtoHttp1Layer<Svc, Reader, Writer>;

    fn layer(&self, inner: Svc) -> Self::Service {
        ProtoHttp1Layer::new(self.config.clone(), inner)
    }
}
