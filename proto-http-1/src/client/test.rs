use crate::client::make_layer::ProtoHttp1ClientMakeLayer;
use crate::client::ProtoHttp1ClientConfig;
use crate::data::{HTTP1ClientResponse, HTTP1Request, HTTTP1Response};
use http::StatusCode;
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
    assert_eq!(
        res,
        HTTTP1Response {
            status: StatusCode::OK,
            headers: Default::default(),
            body: vec![],
        }
    );
}

async fn basic_server<Reader: AsyncReadExt + Send + Unpin + 'static, Writer: AsyncWriteExt + Send + Unpin + 'static>(
    (mut read, mut write): (Reader, Writer),
) -> Result<(), ()> {
    let reader = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let data = reader.read_with_timeout(&mut read, Duration::from_millis(100), None).await;
    assert_eq!(data, vec![]);
    write.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n").await.unwrap();
    Ok(())
}
