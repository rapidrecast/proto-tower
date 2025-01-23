use crate::make_layer::ProtoHttp1MakeLayer;
use crate::{HTTP1Event, HTTP1Response, ProtoHttp1Config};
use http::header::UPGRADE;
use http::{HeaderMap, HeaderName, HeaderValue};
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
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
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
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
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
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
        max_header_size: 0,
        max_body_size: 0,
        timeout: Duration::from_millis(200),
    })).service(TestService);
    client_writer.write_all(b"GET /header HTTP/1.1\r\nHost: localhost\r\n\r\n").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let buffer = read_with_timeout(&mut client_reader, Duration::from_millis(200)).await;
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\n{\"Host\": \"localhost\"}");
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn test_body() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
        max_header_size: 0,
        max_body_size: 0,
        timeout: Duration::from_millis(200),
    })).service(TestService);
    client_writer.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\nHello, World!").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let buffer = read_with_timeout(&mut client_reader, Duration::from_millis(200)).await;
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\n");
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn test_multipart() {
    todo!()
}

#[tokio::test]
async fn test_protocol_upgrade() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new().layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
        max_header_size: 0,
        max_body_size: 0,
        timeout: Duration::from_millis(200),
    })).service(TestService);
    client_writer.write_all(b"GET /upgrade/ HTTP/1.1\r\nHost: localhost\r\nUpgrade: plaintext-protocol\r\nConnection: Upgrade\r\n\r\nHello, World!").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let buffer = read_with_timeout(&mut client_reader, Duration::from_millis(200)).await;
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\n");
    task.await.unwrap().unwrap();
}

async fn read_with_timeout<READ: AsyncReadExt + Unpin + Send + 'static>(reader: &mut READ, timeout: Duration) -> Vec<u8> {
    let start = std::time::Instant::now();
    let mut data = Vec::new();
    let mut buffer = [0u8; 1024];
    let mut finised = false;
    while !finised {
        tokio::select! {
            _ = tokio::time::sleep(std::time::Instant::now() - (start + timeout)) => {
                eprintln!("Timeout reached");
                finised = true;
            }
            n = reader.read(&mut buffer) => {
                match n {
                    Ok(n) => {
                        eprintln!("Read {} bytes", n);
                        if n != 0 {
                            data.extend_from_slice(&buffer[..n]);
                        }
                    }
                    Err(_) => {
                        eprintln!("Error reading");
                        finised = true;
                    }
                }
            }
        }
    }
    data
}

#[derive(Clone)]
pub struct TestService;

impl<READER, WRITER> Service<HTTP1Event<READER, WRITER>> for TestService
where
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
{
    type Response = HTTP1Response;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HTTP1Event<READER, WRITER>) -> Self::Future {
        Box::pin(async move {
            match req {
                HTTP1Event::Request(req) => {
                    if req.path.path() == "/" {
                        Ok(HTTP1Response { status: http::StatusCode::OK, headers: Default::default(), body: vec![] })
                    } else if req.path.path().starts_with("/path/") {
                        Ok(HTTP1Response { status: http::StatusCode::OK, headers: Default::default(), body: format!("Path was {}", req.path.path()).into_bytes() })
                    } else if req.path.path().starts_with("/upgrade/") {
                        assert_eq!(req.headers.get("Upgrade").unwrap(), "plaintext-protocol");
                        assert_eq!(req.headers.get("Connection").unwrap(), "Upgrade");
                        let mut header_map = HeaderMap::new();
                        header_map.insert(UPGRADE, HeaderValue::from_static("plaintext-protocol"));
                        header_map.insert(HeaderName::from_static("Connection"), HeaderValue::from_static("Upgrade"));
                        Ok(HTTP1Response { status: http::StatusCode::SWITCHING_PROTOCOLS, headers: header_map, body: vec![] })
                    } else {
                        eprintln!("{:?}", req);
                        todo!()
                    }
                }
                HTTP1Event::ProtocolUpgrade(_req, _resp, (_read, _write)) => {
                    todo!()
                }
            }
        })
    }
}
