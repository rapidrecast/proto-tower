use crate::server::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FrameContinuation {
    pub header_block_fragment: Vec<u8>,
}
#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FrameContinuation {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        writer.write_all(&self.header_block_fragment).await.map_err(|_| ())
    }
}

pub fn read_continuation_frame(msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    let header_block_fragment = msg_payload.to_vec();
    Ok(Http2InnerFrame::Continuation(Http2FrameContinuation { header_block_fragment }))
}
