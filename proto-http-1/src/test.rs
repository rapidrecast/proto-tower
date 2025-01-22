use crate::layer::{HTTP1Request, HTTP1Response, ProtoHttp1Config};
use crate::make_layer::ProtoHttp1MakeLayer;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_handler() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config{
        max_header_size: 0,
        max_body_size: 0,
        timeout: Duration::from_millis(200),
    })).service(TestService);
    client_writer.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let mut buffer = Vec::with_capacity(1024);
    for _ in 0..100 {
        let mut temp_buf = [0u8; 1024];
        let n = client_reader.read(&mut temp_buf).await.unwrap();
        buffer.extend_from_slice(&temp_buf[..n]);
        if n > 0 {
            break;
        }
    }
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\n");
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn test_path() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config{
        max_header_size: 0,
        max_body_size: 0,
        timeout: Duration::from_millis(200),
    })).service(TestService);
    client_writer.write_all(b"GET /path/abc HTTP/1.1\r\n\r\n").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let mut buffer = Vec::with_capacity(1024);
    for _ in 0..100 {
        let mut temp_buf = [0u8; 1024];
        let n = client_reader.read(&mut temp_buf).await.unwrap();
        buffer.extend_from_slice(&temp_buf[..n]);
        if n > 0 {
            break;
        }
    }
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\nPath was /path/abc");
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn test_headers() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config{
        max_header_size: 0,
        max_body_size: 0,
        timeout: Duration::from_millis(200),
    })).service(TestService);
    client_writer.write_all(b"GET /header HTTP/1.1\r\nHost: localhost\r\n\r\n").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let mut buffer = Vec::with_capacity(1024);
    for _ in 0..100 {
        let mut temp_buf = [0u8; 1024];
        let n = client_reader.read(&mut temp_buf).await.unwrap();
        buffer.extend_from_slice(&temp_buf[..n]);
        if n > 0 {
            break;
        }
    }
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\n{\"Host\": \"localhost\"}");
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn test_body() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config{
        max_header_size: 0,
        max_body_size: 0,
        timeout: Duration::from_millis(200),
    })).service(TestService);
    client_writer.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\nHello, World!").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let mut buffer = Vec::with_capacity(1024);
    for _ in 0..100 {
        let mut temp_buf = [0u8; 1024];
        let n = client_reader.read(&mut temp_buf).await.unwrap();
        buffer.extend_from_slice(&temp_buf[..n]);
        if n > 0 {
            break;
        }
    }
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\n");
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn test_multipart() {
    todo!()
}

#[derive(Clone)]
pub struct TestService;

impl Service<HTTP1Request> for TestService {
    type Response = HTTP1Response;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HTTP1Request) -> Self::Future {
        Box::pin(async move {
            if req.path.path() == "/" {
                Ok(HTTP1Response { status: http::StatusCode::OK, headers: Default::default(), body: vec![] })
            } else if req.path.path().starts_with("/path/") {
                Ok(HTTP1Response { status: http::StatusCode::OK, headers: Default::default(), body: format!("Path was {}", req.path.path()).into_bytes() })
            } else {
                eprintln!("{:?}", req);
                todo!()
            }
        })
    }
}
