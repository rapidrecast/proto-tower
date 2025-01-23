use crate::parser::parse_request;
use crate::{HTTP1Event, HTTP1Response, ProtoHttp1Config};
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
pub struct ProtoHttp1Layer<SERVICE, READER, WRITER>
where
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
    SERVICE: Service<HTTP1Event<READER, WRITER>, Response=HTTP1Response> + Send + Clone,
{
    config: ProtoHttp1Config,
    /// The inner service to process requests
    inner: SERVICE,
    reader_phantom: PhantomData<READER>,
    writer_phantom: PhantomData<WRITER>,
}

impl<SERVICE, READER, WRITER> ProtoHttp1Layer<SERVICE, READER, WRITER>
where
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
    SERVICE: Service<HTTP1Event<READER, WRITER>, Response=HTTP1Response> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: ProtoHttp1Config, inner: SERVICE) -> Self {
        ProtoHttp1Layer { config, inner, reader_phantom: PhantomData, writer_phantom: PhantomData }
    }
}

impl<READER, WRITER, SERVICE, ERROR, SVC_FUT> Service<(READER, WRITER)> for ProtoHttp1Layer<SERVICE, READER, WRITER>
where
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
    SERVICE: Service<HTTP1Event<READER, WRITER>, Response=HTTP1Response, Error=ERROR, Future=SVC_FUT> + Send + Clone + 'static,
    SVC_FUT: Future<Output=Result<HTTP1Response, ERROR>> + Send,
{
    /// The response is handled by the protocol
    type Response = ();
    /// Errors would be failures in parsing the protocol - this should be handled by the protocol
    type Error = ERROR;
    /// The future is the protocol itself
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    /// Indefinitely process the protocol
    fn call(&mut self, (mut reader, mut writer): (READER, WRITER)) -> Self::Future {
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
                        // no-op, timeout reached and loop won't rerun
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
                Ok(partial_resul) => {
                    match partial_resul {
                        Ok(req) => {
                            // Invoke handler
                            let res = service.call(HTTP1Event::Request(req)).await?;
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

