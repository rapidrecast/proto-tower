use crate::parser::read_next_frame;
use crate::{Http2Request, Http2Response, ProtoHttp2Config};
use proto_tower::ZeroReadBehaviour;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::Service;

#[derive(Debug)]
pub enum ProtoHttp2Error<Error: Debug> {
    InvalidPreface,
    ServiceError(Error),
}

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
pub struct ProtoH2CLayer<Svc>
where
    Svc: Service<Http2Request, Response = Http2Response> + Send + Clone,
{
    config: ProtoHttp2Config,
    /// The inner service to process requests
    inner: Svc,
}

impl<Svc> ProtoH2CLayer<Svc>
where
    Svc: Service<Http2Request, Response = Http2Response> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: ProtoHttp2Config, inner: Svc) -> Self {
        ProtoH2CLayer { config, inner }
    }
}

impl<Reader, Writer, Svc, SvcError, SvcFut> Service<(Reader, Writer)> for ProtoH2CLayer<Svc>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<Http2Request, Response = Http2Response, Error = SvcError, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<Http2Response, SvcError>> + Send,
{
    /// The response is handled by the protocol
    type Response = ();
    /// Errors would be failures in parsing the protocol - this should be handled by the protocol
    type Error = ProtoHttp2Error<SvcError>;
    /// The future is the protocol itself
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    /// Indefinitely process the protocol
    fn call(&mut self, (mut reader, mut writer): (Reader, Writer)) -> Self::Future {
        let mut service = self.inner.clone();
        let config = self.config.clone();
        Box::pin(async move {
            let async_read = proto_tower::AsyncReadToBuf::new(ZeroReadBehaviour::TickAndYield);
            let mut preface = async_read.read_with_timeout(&mut reader, config.timeout, Some(28)).await;
            if preface != b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n" {
                return Err(ProtoHttp2Error::InvalidPreface);
            }
            // Validate request
            match read_next_frame(&mut reader).await {
                Ok(partial_resul) => {
                    match partial_resul {
                        Ok(req) => {
                            // Invoke handler
                            let res = dbg!(service.call(req).await?);
                            // Send response
                            res.write_onto(writer).await;
                            Ok(())
                        }
                        Err((req, errs)) => {
                            todo!()
                        }
                    }
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        })
    }
}
