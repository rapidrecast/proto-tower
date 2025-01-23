use crate::layer::ProtoHttp1Layer;
use crate::{HTTP1Event, HTTP1Response, ProtoHttp1Config};
use std::marker::PhantomData;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Layer, Service};

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1MakeLayer<SERVICE, READER, WRITER>
where
    SERVICE: Service<HTTP1Event<READER, WRITER>, Response=HTTP1Response> + Send + Clone,
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
{
    config: ProtoHttp1Config,
    phantom_service: PhantomData<SERVICE>,
    phantom_reader: PhantomData<READER>,
    phantom_writer: PhantomData<WRITER>,
}

impl<SERVICE, READER, WRITER> ProtoHttp1MakeLayer<SERVICE, READER, WRITER>
where
    SERVICE: Service<HTTP1Event<READER, WRITER>, Response=HTTP1Response> + Send + Clone,
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
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

impl<SERVICE, READER, WRITER> Layer<SERVICE> for ProtoHttp1MakeLayer<SERVICE, READER, WRITER>
where
    SERVICE: Service<HTTP1Event<READER, WRITER>, Response=HTTP1Response> + Send + Clone,
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
{
    type Service = ProtoHttp1Layer<SERVICE, READER, WRITER>;

    fn layer(&self, inner: SERVICE) -> Self::Service {
        ProtoHttp1Layer::new(self.config.clone(), inner)
    }
}
