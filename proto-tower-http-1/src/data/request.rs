use async_trait::async_trait;
use http::Uri;
use tokio::io::AsyncWriteExt;

/// An HTTP/1.1 request
#[derive(Debug, Clone)]
pub struct HTTP1Request {
    pub path: Uri,
    pub method: http::Method,
    pub headers: http::HeaderMap,
    pub body: Vec<u8>,
}

#[async_trait]
impl<Writer> proto_tower_util::WriteTo<Writer, ()> for HTTP1Request
where
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    async fn write_to(&self, writer: &mut Writer) -> Result<(), ()> {
        // Request line
        writer.write_all(self.method.as_str().as_bytes()).await.map_err(|_| ())?;
        writer.write_all(" ".as_bytes()).await.map_err(|_| ())?;
        writer.write_all(self.path.path().as_bytes()).await.map_err(|_| ())?;
        writer.write_all(" HTTP/1.1\r\n".as_bytes()).await.map_err(|_| ())?;
        // Headers
        for (header_name, header_value) in &self.headers {
            writer.write_all(header_name.as_str().as_bytes()).await.map_err(|_| ())?;
            writer.write_all(": ".as_bytes()).await.map_err(|_| ())?;
            writer.write_all(header_value.as_bytes()).await.map_err(|_| ())?;
            writer.write_all("\r\n".as_bytes()).await.map_err(|_| ())?;
        }
        writer
            .write_all(format!("Content-Length: {}\r\n", self.body.len()).as_bytes())
            .await
            .map_err(|_| ())?;
        writer.write_all("\r\n".as_bytes()).await.map_err(|_| ())?;
        // Body
        if !self.body.is_empty() {
            writer.write_all(&self.body).await.map_err(|_| ())?;
        }
        Ok(())
    }
}
