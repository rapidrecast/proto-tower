use crate::server::parser::error_codes::{FrameError, FrameErrorCode};
use crate::server::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FrameResetStream {
    pub error_code: FrameError,
}
#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FrameResetStream {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        todo!()
    }
}

pub fn read_reset_stream_frame(_flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    if msg_payload.len() != 4 {
        return Err("Expected 4 bytes for reset stream frame");
    }
    let error_code = msg_payload.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let error_code = error_code.as_frame_err().map_err(|_| "Invalid error code")?;
    Ok(Http2InnerFrame::RstStream(Http2FrameResetStream { error_code }))
}
