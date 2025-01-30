use crate::data::{KafkaProtocolError, KafkaRequest, KafkaResponse};
use crate::server::KafkaProtoServerConfig;
use proto_tower_util::{AsyncReadToBuf, ZeroReadBehaviour};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{Receiver, Sender};
use tower::Service;

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
pub struct ProtoHttp1ServerLayer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>), Response = ()> + Send + Clone,
{
    config: KafkaProtoServerConfig,
    /// The inner service to process requests
    inner: Svc,
    reader_phantom: PhantomData<Reader>,
    writer_phantom: PhantomData<Writer>,
}

impl<Svc, Reader, Writer> ProtoHttp1ServerLayer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: KafkaProtoServerConfig, inner: Svc) -> Self {
        ProtoHttp1ServerLayer {
            config,
            inner,
            reader_phantom: PhantomData,
            writer_phantom: PhantomData,
        }
    }
}

impl<Reader, Writer, Svc, SvcError, SvcFut> Service<(Reader, Writer)> for ProtoHttp1ServerLayer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>), Response = (), Error = SvcError, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<(), SvcError>> + Send,
    SvcError: std::fmt::Debug + Send + 'static,
{
    /// The response is handled by the protocol
    type Response = ();
    /// Errors would be failures in parsing the protocol - this should be handled by the protocol
    type Error = KafkaProtocolError<SvcError>;
    /// The future is the protocol itself
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| KafkaProtocolError::InternalServiceError(e))
    }

    /// Indefinitely process the protocol
    fn call(&mut self, (mut reader, mut writer): (Reader, Writer)) -> Self::Future {
        let mut service = self.inner.clone();
        let config = self.config.clone();
        Box::pin(async move {
            let async_reader = AsyncReadToBuf::<1024>::new(ZeroReadBehaviour::TickAndYield);
            let buffer = async_reader.read_with_timeout(&mut reader, config.timeout, None).await;
            let (svc_write, write) = tokio::sync::mpsc::channel::<KafkaResponse>(1024);
            let (read, svc_read) = tokio::sync::mpsc::channel::<KafkaRequest>(1024);
            let svc_fut = service.call((svc_read, svc_write)).await.map_err(|e| KafkaProtocolError::InternalServiceError(e))?;
            Ok(())
        })
    }
}
