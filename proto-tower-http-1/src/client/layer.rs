use crate::client::parser::parse_http1_response;
use crate::client::ProtoHttp1ClientConfig;
use crate::data::request::HTTP1Request;
use crate::data::HTTP1ClientResponse;
use proto_tower_util::{AsyncReadToBuf, WriteTo, ZeroReadBehaviour};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{ReadHalf, SimplexStream, WriteHalf};
use tower::Service;

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
pub struct ProtoHttp1ClientLayer<Svc>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = ()> + Send + Clone,
{
    config: ProtoHttp1ClientConfig,
    /// The inner service to process requests
    inner: Svc,
}

impl<Svc> ProtoHttp1ClientLayer<Svc>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: ProtoHttp1ClientConfig, inner: Svc) -> Self {
        ProtoHttp1ClientLayer { config, inner }
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

impl<Svc, SvcError, SvcFut> Service<HTTP1Request> for ProtoHttp1ClientLayer<Svc>
where
    Svc: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = (), Error = SvcError, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<(), SvcError>> + Send,
{
    /// The response is handled by the protocol
    type Response = HTTP1ClientResponse<ReadHalf<SimplexStream>, WriteHalf<SimplexStream>>;
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
        Box::pin(async move {
            let (svc_read, mut write) = tokio::io::simplex(1024);
            let (mut read, svc_write) = tokio::io::simplex(1024);
            request.write_to(&mut write).await.unwrap();
            service.call((svc_read, svc_write)).await.map_err(|e| ProtoHttp1LayerError::InternalServiceError(e))?;
            let read_buf = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
            let buf = read_buf.read_with_timeout(&mut read, config.timeout, None).await;
            let resp = parse_http1_response(&buf).unwrap();
            Ok(HTTP1ClientResponse::Response(resp))
        })
    }
}
