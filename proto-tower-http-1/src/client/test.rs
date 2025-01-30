use crate::client::make_layer::ProtoHttp1ClientMakeLayer;
use crate::client::ProtoHttp1ClientConfig;
use crate::data::request::HTTP1Request;
use crate::data::HTTP1ClientResponse;
use http::{HeaderMap, StatusCode};
use proto_tower_util::{AsyncReadToBuf, ZeroReadBehaviour};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Service, ServiceBuilder};

#[tokio::test]
async fn test_client() {
    let mut client = ServiceBuilder::new()
        .layer(ProtoHttp1ClientMakeLayer::new(ProtoHttp1ClientConfig {
            max_header_size: 0,
            max_body_size: 0,
            timeout: Duration::from_millis(200),
        }))
        .service(tower::service_fn(basic_server));

    let res = client
        .call(HTTP1Request {
            path: Default::default(),
            method: Default::default(),
            headers: Default::default(),
            body: vec![],
        })
        .await;
    let res = res.unwrap();

    let res = match res {
        HTTP1ClientResponse::Response(res) => res,
        HTTP1ClientResponse::ProtocolUpgrade(_, _) => panic!(),
    };
    let mut headers = HeaderMap::new();
    headers.insert("Content-Length", "37".parse().unwrap());
    assert_eq!(res.status, StatusCode::OK);
    assert_eq!(res.headers, headers);
    assert_eq!(String::from_utf8(res.body).unwrap(), "GET / HTTP/1.1\r\nContent-Length: 0\r\n\r\n");
}

async fn basic_server<Reader: AsyncReadExt + Send + Unpin + 'static, Writer: AsyncWriteExt + Send + Unpin + 'static>(
    (mut read, mut write): (Reader, Writer),
) -> Result<(), ()> {
    let reader = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let data = reader.read_with_timeout(&mut read, Duration::from_millis(100), None).await;
    write
        .write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", data.len()).as_bytes())
        .await
        .unwrap();
    write.write_all(&data).await.unwrap();
    Ok(())
}
