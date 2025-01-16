use crate::layer::{HTTP1Request, HTTP1Response};
use crate::make_layer::ProtoHttp1MakeLayer;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_handler() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new())
        .service(TestService);
    client_writer
        .write_all(b"GET / HTTP/1.1\r\n\r\n")
        .await
        .unwrap();
    service.call((server_reader, server_writer)).await.unwrap();
    let mut buffer = Vec::with_capacity(1024);
    client_reader.read(&mut buffer).await.unwrap();
    assert_eq!(buffer, b"HTTP/1.1 200 OK\r\n\r\n");
}

#[derive(Clone)]
pub struct TestService;

impl Service<HTTP1Request> for TestService {
    type Response = HTTP1Response;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HTTP1Request) -> Self::Future {
        Box::pin(async move { Ok(HTTP1Response {}) })
    }
}
