use crate::make_layer::ProtoHttp1MakeLayer;
use crate::{HTTP1Event, HTTP1Response, ProtoHttp1Config};
use http::header::{CONNECTION, UPGRADE};
use http::{HeaderMap, HeaderValue};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_handler() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(200),
        }))
        .service(TestService);
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
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(200),
        }))
        .service(TestService);
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
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(200),
        }))
        .service(TestService);
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
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(200),
        }))
        .service(TestService);
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
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(200),
        }))
        .service(TestService);
    client_writer
        .write_all(b"GET /upgrade/ HTTP/1.1\r\nHost: localhost\r\nUpgrade: plaintext-protocol\r\nConnection: Upgrade\r\n\r\nHello, World!")
        .await
        .unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let buffer = read_with_timeout(&mut client_reader, Duration::from_millis(1000)).await;
    assert_eq!(
        String::from_utf8(buffer).unwrap(),
        // TODO this bad, should not have 2 clrf
        "HTTP/1.1 101 Switching Protocols\r\n\r\nupgrade: plaintext-protocol\r\nconnection: Upgrade\r\n"
    );
    drop(client_writer);
    drop(client_reader);
    task.await.unwrap().unwrap();
}

async fn read_with_timeout<READ: AsyncReadExt + Unpin + Send + 'static>(reader: &mut READ, timeout: Duration) -> Vec<u8> {
    let mut data = Vec::new();
    let mut buffer = [0u8; 1024];
    let mut finished = AtomicBool::new(false);
    let mut tries = AtomicU32::new(0);
    const MAX_TRIES: u32 = 10;
    let remaining_timeout = timeout / MAX_TRIES;
    while !finished.load(Ordering::SeqCst) && tries.load(Ordering::SeqCst) < MAX_TRIES {
        tokio::select! {
            _ = tokio::time::sleep(remaining_timeout) => {
                let new_tries = tries.fetch_add(1, Ordering::SeqCst) + 1;
                eprintln!("Timeout ({:?} * {})", remaining_timeout, new_tries);
                if new_tries >= MAX_TRIES {
                    eprintln!("Timeout reached ({:?} * {})", remaining_timeout, new_tries);
                    finished.store(true, Ordering::SeqCst);
                }
            }
            n = reader.read(&mut buffer) => {
                match n {
                    Ok(n) => {
                        if n != 0 {
                            eprintln!("Test read {} bytes", n);
                            data.extend_from_slice(&buffer[..n]);
                            tries.store(0, Ordering::SeqCst);
                        } else {
                            tries.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                    Err(_) => {
                        eprintln!("Error reading");
                        finished.store(true, Ordering::SeqCst);
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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HTTP1Event<READER, WRITER>) -> Self::Future {
        Box::pin(async move {
            match req {
                HTTP1Event::Request(req) => {
                    if req.path.path() == "/" {
                        Ok(HTTP1Response {
                            status: http::StatusCode::OK,
                            headers: Default::default(),
                            body: vec![],
                        })
                    } else if req.path.path().starts_with("/path/") {
                        Ok(HTTP1Response {
                            status: http::StatusCode::OK,
                            headers: Default::default(),
                            body: format!("Path was {}", req.path.path()).into_bytes(),
                        })
                    } else if req.path.path().starts_with("/upgrade/") {
                        if !req.headers.contains_key("Upgrade") || !req.headers.contains_key("Connection") {
                            return Ok(HTTP1Response {
                                status: http::StatusCode::BAD_REQUEST,
                                headers: Default::default(),
                                body: "Upgrade and Connection headers are required".into(),
                            });
                        }
                        if req.headers.get("Upgrade").unwrap() != "plaintext-protocol" || req.headers.get("Connection").unwrap() != "Upgrade" {
                            return Ok(HTTP1Response {
                                status: http::StatusCode::BAD_REQUEST,
                                headers: Default::default(),
                                body: "Upgrade and Connection headers must be plaintext-protocol and Upgrade".into(),
                            });
                        }
                        let mut header_map = HeaderMap::new();
                        header_map.insert(UPGRADE, HeaderValue::from_static("plaintext-protocol"));
                        header_map.insert(CONNECTION, HeaderValue::from_static("Upgrade"));
                        Ok(HTTP1Response {
                            status: http::StatusCode::SWITCHING_PROTOCOLS,
                            headers: header_map,
                            body: vec![],
                        })
                    } else {
                        eprintln!("{:?}", req);
                        todo!()
                    }
                }
                HTTP1Event::ProtocolUpgrade(_req, _resp, (mut read, mut write)) => {
                    let mut buf = [0; 1024];
                    match read.read(&mut buf).await {
                        Ok(n) => match write.write_all(&buf[..n]).await {
                            Ok(n) => Err(()),
                            Err(n) => Err(()),
                        },
                        Err(_) => Err(()),
                    }
                }
            }
        })
    }
}
