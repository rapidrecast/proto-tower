use http::Uri;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod layer;
mod make_layer;
mod parser;
#[cfg(test)]
mod test;

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
    /// A protocol upgrade including the original request and subsequent response
    ProtocolUpgrade(HTTP1Request, HTTTP1Response, (READER, WRITER)),
}

/// An HTTP/1.1 request
#[derive(Debug, Clone)]
pub struct HTTP1Request {
    pub path: Uri,
    pub method: http::Method,
    pub headers: http::HeaderMap,
    pub body: Vec<u8>,
}

pub enum HTTTP1ResponseEvent {
    /// Use this for protocol upgrades
    NoResponseExpected,
    /// Use this for request responses
    Response(HTTTP1Response),
}

/// An HTTP/1.1 response
#[derive(Debug)]
pub struct HTTTP1Response {
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
    pub body: Vec<u8>,
}

impl HTTTP1Response {
    pub async fn write_onto<WRITER: AsyncWriteExt + Send + Unpin + 'static>(&self, writer: &mut WRITER) {
        // RESPONSE
        const VERSION: &[u8] = "HTTP/1.1".as_bytes();
        writer.write_all(VERSION).await.unwrap();
        writer.write_all(" ".as_bytes()).await.unwrap();
        writer.write_all(self.status.as_str().as_bytes()).await.unwrap();
        writer.write_all(" ".as_bytes()).await.unwrap();
        writer.write_all(self.status.canonical_reason().unwrap_or("UNKNOWN REASON").as_bytes()).await.unwrap();
        writer.write_all("\r\n".as_bytes()).await.unwrap();

        // HEADERS
        for (k, v) in self.headers.iter() {
            writer.write_all(k.as_str().as_bytes()).await.unwrap();
            writer.write_all(": ".as_bytes()).await.unwrap();
            writer.write_all(v.as_bytes()).await.unwrap();
            writer.write_all("\r\n".as_bytes()).await.unwrap();
        }
        writer.write_all("\r\n".as_bytes()).await.unwrap();

        // BODY
        writer.write_all(&self.body).await.unwrap();
    }
}
