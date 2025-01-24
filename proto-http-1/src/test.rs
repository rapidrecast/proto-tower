use crate::make_layer::ProtoHttp1MakeLayer;
use crate::{HTTP1Event, HTTTP1Response, HTTTP1ResponseEvent, ProtoHttp1Config};
use http::header::{CONNECTION, UPGRADE};
use http::{HeaderMap, HeaderValue};
use proto_tower::ZeroReadBehaviour;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_handler() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(100),
        }))
        .service(TestService);
    client_writer.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let reader = proto_tower::AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let buffer = reader.read_with_timeout(&mut client_reader, Duration::from_millis(200), None).await;
    let (task, result) = result_if_task_finished(task).await;
    if let Some(res) = result {
        res.unwrap();
    }
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\n");
    if let Some(task) = task {
        task.await.unwrap().unwrap();
    }
}

#[tokio::test]
async fn test_path() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(100),
        }))
        .service(TestService);
    client_writer.write_all(b"GET /path/abc HTTP/1.1\r\n\r\n").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let reader = proto_tower::AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let buffer = reader.read_with_timeout(&mut client_reader, Duration::from_millis(200), None).await;
    let (task, result) = result_if_task_finished(task).await;
    if let Some(res) = result {
        res.unwrap();
    }
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\nPath was /path/abc");
    if let Some(task) = task {
        task.await.unwrap().unwrap();
    }
}

#[tokio::test]
async fn test_headers() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(100),
        }))
        .service(TestService);
    client_writer.write_all(b"GET /header HTTP/1.1\r\nHost: localhost\r\n\r\n").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let reader = proto_tower::AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let buffer = reader.read_with_timeout(&mut client_reader, Duration::from_millis(200), None).await;
    let (task, result) = result_if_task_finished(task).await;
    if let Some(res) = result {
        res.unwrap();
    }
    assert_eq!(
        String::from_utf8(buffer).unwrap(),
        "HTTP/1.1 200 OK\r\nhost: localhost\r\n\r\nReceived headers are returned"
    );
    if let Some(task) = task {
        task.await.unwrap().unwrap();
    }
}

#[tokio::test]
async fn test_body() {
    let (mut client_reader, server_writer) = tokio::io::duplex(1024);
    let (server_reader, mut client_writer) = tokio::io::duplex(1024);
    let mut service = ServiceBuilder::new()
        .layer(ProtoHttp1MakeLayer::new(ProtoHttp1Config {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(100),
        }))
        .service(TestService);
    client_writer.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\nHello, World!").await.unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let reader = proto_tower::AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let buffer = reader.read_with_timeout(&mut client_reader, Duration::from_millis(200), None).await;
    let (task, result) = result_if_task_finished(task).await;
    if let Some(res) = result {
        res.unwrap();
    }
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1 200 OK\r\n\r\n");
    if let Some(task) = task {
        task.await.unwrap().unwrap();
    }
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
            timeout: Duration::from_millis(100),
        }))
        .service(TestService);
    client_writer
        .write_all(b"GET /upgrade/ HTTP/1.1\r\nHost: localhost\r\nUpgrade: plaintext-protocol\r\nConnection: Upgrade\r\n\r\nHello, World!")
        .await
        .unwrap();
    let task = tokio::spawn(service.call((server_reader, server_writer)));
    let reader = proto_tower::AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let buffer = reader.read_with_timeout(&mut client_reader, Duration::from_millis(200), None).await;
    let (task, result) = result_if_task_finished(task).await;
    if let Some(res) = result {
        res.unwrap();
    }
    let task = task.unwrap();
    assert_eq!(
        String::from_utf8(buffer).unwrap(),
        "HTTP/1.1 101 Switching Protocols\r\nupgrade: plaintext-protocol\r\nconnection: Upgrade\r\n\r\n"
    );
    client_writer.write_all(b"Hello, World!").await.unwrap();
    let reader = proto_tower::AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let buffer = reader.read_with_timeout(&mut client_reader, Duration::from_millis(200), None).await;
    assert_eq!(String::from_utf8(buffer).unwrap(), "Hello, World!");
    drop(client_writer);
    drop(client_reader);
    task.await.unwrap().unwrap();
}

async fn result_if_task_finished<T, E>(task: JoinHandle<Result<T, E>>) -> (Option<JoinHandle<Result<T, E>>>, Option<Result<T, E>>) {
    if task.is_finished() {
        let res = task.await.unwrap();
        (None, Some(res))
    } else {
        (Some(task), None)
    }
}

#[derive(Clone)]
pub struct TestService;

impl<READER, WRITER> Service<HTTP1Event<READER, WRITER>> for TestService
where
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
{
    type Response = HTTTP1ResponseEvent;
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
                        Ok(HTTTP1ResponseEvent::Response(HTTTP1Response {
                            status: http::StatusCode::OK,
                            headers: Default::default(),
                            body: vec![],
                        }))
                    } else if req.path.path().starts_with("/path") {
                        Ok(HTTTP1ResponseEvent::Response(HTTTP1Response {
                            status: http::StatusCode::OK,
                            headers: Default::default(),
                            body: format!("Path was {}", req.path.path()).into_bytes(),
                        }))
                    } else if req.path.path().starts_with("/header") {
                        Ok(HTTTP1ResponseEvent::Response(HTTTP1Response {
                            status: http::StatusCode::OK,
                            headers: req.headers,
                            body: Vec::from("Received headers are returned"),
                        }))
                    } else if req.path.path().starts_with("/upgrade") {
                        if !req.headers.contains_key("Upgrade") || !req.headers.contains_key("Connection") {
                            return Ok(HTTTP1ResponseEvent::Response(HTTTP1Response {
                                status: http::StatusCode::BAD_REQUEST,
                                headers: Default::default(),
                                body: "Upgrade and Connection headers are required".into(),
                            }));
                        }
                        if req.headers.get("Upgrade").unwrap() != "plaintext-protocol" || req.headers.get("Connection").unwrap() != "Upgrade" {
                            return Ok(HTTTP1ResponseEvent::Response(HTTTP1Response {
                                status: http::StatusCode::BAD_REQUEST,
                                headers: Default::default(),
                                body: "Upgrade and Connection headers must be plaintext-protocol and Upgrade".into(),
                            }));
                        }
                        let mut header_map = HeaderMap::new();
                        header_map.insert(UPGRADE, HeaderValue::from_static("plaintext-protocol"));
                        header_map.insert(CONNECTION, HeaderValue::from_static("Upgrade"));
                        Ok(HTTTP1ResponseEvent::Response(HTTTP1Response {
                            status: http::StatusCode::SWITCHING_PROTOCOLS,
                            headers: header_map,
                            body: vec![],
                        }))
                    } else {
                        Ok(HTTTP1ResponseEvent::Response(HTTTP1Response {
                            status: http::StatusCode::NOT_FOUND,
                            headers: Default::default(),
                            body: vec![],
                        }))
                    }
                }
                HTTP1Event::ProtocolUpgrade(_req, _resp, (mut read, mut write)) => {
                    let mut buf = [0; 1024];
                    match read.read(&mut buf).await {
                        Ok(n) => match write.write_all(&buf[..n]).await {
                            Ok(_n) => Ok(HTTTP1ResponseEvent::NoResponseExpected),
                            Err(_error) => Ok(HTTTP1ResponseEvent::NoResponseExpected),
                        },
                        Err(_) => Ok(HTTTP1ResponseEvent::NoResponseExpected),
                    }
                }
            }
        })
    }
}
