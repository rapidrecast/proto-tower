pub mod request;

use crate::data::request::HTTP1Request;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// A service downstream from HTTP1ServerLayer will receive these structs
pub enum HTTP1ServerEvent<READER, WRITER>
where
    READER: AsyncReadExt + Send + Unpin + 'static,
    WRITER: AsyncWriteExt + Send + Unpin + 'static,
{
    Request(HTTP1Request),
    /// A protocol upgrade including the original request and subsequent response
    ProtocolUpgrade(HTTP1Request, HTTP1Response, (READER, WRITER)),
}

/// When the client is called, it returns this value
#[derive(Debug, Eq, PartialEq)]
pub enum HTTP1ClientResponse<Reader, Writer>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    Response(HTTP1Response),
    /// If the response is a protocol upgrade, you will get this instead of a normal response
    ProtocolUpgrade(HTTP1Response, (Reader, Writer)),
}

pub enum Http1ServerResponseEvent {
    /// Use this for protocol upgrades
    NoResponseExpected,
    /// Use this for request responses
    Response(HTTP1Response),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ProtoHttp1LayerError<SvcError> {
    /// An error in the implementation of this layer
    #[allow(dead_code)]
    Implementation(String),
    /// The internal service returned a wrong response
    InternalServiceWrongResponse,
    /// An error in the internal service
    InternalServiceError(SvcError),
}

/// An HTTP/1.1 response
#[derive(Debug, Eq, PartialEq)]
pub struct HTTP1Response {
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
    pub body: Vec<u8>,
}

impl HTTP1Response {
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
