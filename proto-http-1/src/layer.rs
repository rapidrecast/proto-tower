use crate::parser::parse_request;
use crate::{HTTP1Event, HTTTP1ResponseEvent, ProtoHttp1Config};
use http::header::{CONNECTION, UPGRADE};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::Service;

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
pub struct ProtoHttp1Layer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<HTTP1Event<Reader, Writer>, Response = HTTTP1ResponseEvent> + Send + Clone,
{
    config: ProtoHttp1Config,
    /// The inner service to process requests
    inner: Svc,
    reader_phantom: PhantomData<Reader>,
    writer_phantom: PhantomData<Writer>,
}

impl<Svc, Reader, Writer> ProtoHttp1Layer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<HTTP1Event<Reader, Writer>, Response = HTTTP1ResponseEvent> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: ProtoHttp1Config, inner: Svc) -> Self {
        ProtoHttp1Layer {
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

impl<Reader, Writer, Svc, SvcError, SvcFut> Service<(Reader, Writer)> for ProtoHttp1Layer<Svc, Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<HTTP1Event<Reader, Writer>, Response = HTTTP1ResponseEvent, Error = SvcError, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<HTTTP1ResponseEvent, SvcError>> + Send,
{
    /// The response is handled by the protocol
    type Response = ();
    /// Errors would be failures in parsing the protocol - this should be handled by the protocol
    type Error = ProtoHttp1LayerError<SvcError>;
    /// The future is the protocol itself
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| ProtoHttp1LayerError::InternalServiceError(e))
    }

    /// Indefinitely process the protocol
    fn call(&mut self, (mut reader, mut writer): (Reader, Writer)) -> Self::Future {
        let mut service = self.inner.clone();
        let config = self.config.clone();
        Box::pin(async move {
            let mut last_time = std::time::Instant::now();
            let mut buffer = Vec::with_capacity(1024);
            let mut temp_buf = [0u8; 1024];
            // Read all input
            while last_time + config.timeout > std::time::Instant::now() {
                tokio::select! {
                    _ = tokio::time::sleep(config.timeout) => {
                    }
                    n = reader.read(&mut temp_buf) => {
                        match n {
                            Ok(n) => {
                                if n != 0 {
                                    last_time = std::time::Instant::now();
                                    buffer.extend_from_slice(&temp_buf[..n]);
                                }
                            }
                            Err(_) => {
                                last_time = std::time::Instant::now()-config.timeout-Duration::from_secs(1);
                            }
                        }
                    }
                }
            }
            // Validate request
            match parse_request(&buffer) {
                Ok(partial_result) => {
                    match partial_result {
                        Ok(req) => {
                            // Invoke handler
                            let res = service
                                .call(HTTP1Event::Request(req.clone()))
                                .await
                                .map_err(|e| ProtoHttp1LayerError::InternalServiceError(e))?;
                            let res = match res {
                                HTTTP1ResponseEvent::Response(res) => res,
                                // TODO we need a better error to indicate wrong response from  service
                                _ => return Err(ProtoHttp1LayerError::InternalServiceWrongResponse),
                            };
                            // Send response
                            res.write_onto(&mut writer).await;
                            if res.headers.contains_key(&UPGRADE) && res.headers.contains_key(&CONNECTION) && res.headers.get(CONNECTION).unwrap() == "Upgrade" {
                                // Upgrade protocol
                                service
                                    .call(HTTP1Event::ProtocolUpgrade(req, res, (reader, writer)))
                                    .await
                                    .map_err(|e| ProtoHttp1LayerError::InternalServiceError(e))?;
                            }
                            Ok(())
                        }
                        Err((_req, _errs)) => Err(ProtoHttp1LayerError::Implementation("Partial error is not implemented yet".to_string())),
                    }
                }
                Err(e) => Err(ProtoHttp1LayerError::Implementation(format!("Error parsing request: {}", e))),
            }
        })
    }
}
