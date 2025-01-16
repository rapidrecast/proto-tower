use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::Service;

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
pub struct ProtoHttp1Layer<SERVICE>
where
    SERVICE: Service<HTTP1Request, Response = HTTP1Response> + Send + Clone,
{
    /// The inner service to process requests
    inner: SERVICE,
}

impl<SERVICE> ProtoHttp1Layer<SERVICE>
where
    SERVICE: Service<HTTP1Request, Response = HTTP1Response> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(inner: SERVICE) -> Self {
        ProtoHttp1Layer { inner }
    }
}

impl<READER, WRITER, SERVICE, ERROR, SVC_FUT> Service<(READER, WRITER)> for ProtoHttp1Layer<SERVICE>
where
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
    SERVICE: Service<HTTP1Request, Response = HTTP1Response, Error = ERROR, Future = SVC_FUT> + Send + Clone + 'static,
    SVC_FUT: Future<Output = Result<HTTP1Response, ERROR>> + Send,
{
    /// The response is handled by the protocol
    type Response = ();
    /// Errors would be failures in parsing the protocol - this should be handled by the protocol
    type Error = ERROR;
    /// The future is the protocol itself
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    /// Indefinitely process the protocol
    fn call(&mut self, (mut reader, mut writer): (READER, WRITER)) -> Self::Future {
        let mut service = self.inner.clone();
        Box::pin(async move {
            let mut buffer = Vec::with_capacity(1024);
            let mut temp_buf = [0u8; 1024];
            // Read all input
            while let Ok(n) = reader.read(&mut temp_buf).await {
                if n == 0 {
                    break;
                }
                buffer.extend_from_slice(&temp_buf[..n]);
            }
            // Validate request
            let req = HTTP1Request {};
            // Invoke handler
            let res = service.call(req).await?;
            // Send response
            writer.write_all("This is a response".as_bytes()).await.unwrap();
            Ok(())
        })
    }
}

pub struct HTTP1Request {}

pub struct HTTP1Response {}
