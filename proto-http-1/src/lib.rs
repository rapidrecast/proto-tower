use http::Uri;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod layer;
mod make_layer;
#[cfg(test)]
mod test;
mod parser;

#[derive(Clone, Debug)]
pub struct ProtoHttp1Config {
    pub max_header_size: usize,
    pub max_body_size: usize,
    pub timeout: std::time::Duration,
}

/// This is what the downstream service will receive
pub enum HTTP1Event<READER, WRITER>
where
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
{
    Request(HTTP1Request),
    ProtocolUpgrade((READER, WRITER)),
}

/// An HTTP/1.1 request
#[derive(Debug)]
pub struct HTTP1Request {
    pub path: Uri,
    pub method: http::Method,
    pub headers: http::HeaderMap,
    pub body: Vec<u8>,
}

/// An HTTP/1.1 response
#[derive(Debug)]
pub struct HTTP1Response {
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
    pub body: Vec<u8>,
}

impl HTTP1Response {
    pub async fn write_onto<WRITER: AsyncWriteExt + Send + Unpin + 'static>(&self, mut writer: WRITER) {
        // RESPONSE
        const VERSION: &[u8] = "HTTP/1.1".as_bytes();
        writer.write_all(VERSION).await.unwrap();
        writer.write_all(" ".as_bytes()).await.unwrap();
        writer.write_all(self.status.as_str().as_bytes()).await.unwrap();
        writer.write_all(" ".as_bytes()).await.unwrap();
        writer.write_all(self.status.canonical_reason().unwrap_or("UNKNOWN REASON").as_bytes()).await.unwrap();
        writer.write_all("\r\n".as_bytes()).await.unwrap();

        // HEADERS
        writer.write_all("\r\n".as_bytes()).await.unwrap();
        for (k, v) in self.headers.iter() {
            writer.write_all(k.as_str().as_bytes()).await.unwrap();
            writer.write_all(": ".as_bytes()).await.unwrap();
            writer.write_all(v.as_bytes()).await.unwrap();
            writer.write_all("\r\n".as_bytes()).await.unwrap();
        }

        // BODY
        writer.write_all(&self.body).await.unwrap();
    }
}
