use crate::client::ProtoHttp1ClientConfig;
use crate::data::{HTTP1ClientResponse, HTTP1Request};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::Service;

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
pub struct ProtoHttp1ClientLayer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Reader, Writer), Response = ()> + Send + Clone,
{
    config: ProtoHttp1ClientConfig,
    /// The inner service to process requests
    inner: Svc,
    reader_phantom: PhantomData<Reader>,
    writer_phantom: PhantomData<Writer>,
}

impl<Svc, Reader, Writer> ProtoHttp1ClientLayer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Reader, Writer), Response = ()> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: ProtoHttp1ClientConfig, inner: Svc) -> Self {
        ProtoHttp1ClientLayer {
            config,
            inner,
            reader_phantom: PhantomData,
            writer_phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum ProtoHttp1LayerError<SvcError> {
    /// An error in the implementation of this layer
    #[allow(dead_code)]
    Implementation(String),
    /// The internal service returned a wrong response
    InternalServiceWrongResponse,
    /// An error in the internal service
    InternalServiceError(SvcError),
}

impl<Reader, Writer, Svc, SvcError, SvcFut> Service<HTTP1Request> for ProtoHttp1ClientLayer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Reader, Writer), Response = (), Error = SvcError, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<(), SvcError>> + Send,
{
    /// The response is handled by the protocol
    type Response = HTTP1ClientResponse<Reader, Writer>;
    /// Errors would be failures in parsing the protocol - this should be handled by the protocol
    type Error = ProtoHttp1LayerError<SvcError>;
    /// The future is the protocol itself
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| ProtoHttp1LayerError::InternalServiceError(e))
    }

    /// Indefinitely process the protocol
    fn call(&mut self, request: HTTP1Request) -> Self::Future {
        let mut service = self.inner.clone();
        let config = self.config.clone();
        Box::pin(async move { todo!() })
    }
}
