use crate::server::parser::error_codes::{FrameError, FrameErrorCode};
use crate::server::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use parser_helper::ParseHelper;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FrameGoaway {
    pub reserved_and_last_stream_id: u32,
    pub error_code: FrameError,
    pub additional_debug_data: Vec<u8>,
}
#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FrameGoaway {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        todo!()
    }
}

pub fn read_goaway_frame(_flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    if msg_payload.len() < 8 {
        return Err("Expected at least 8 bytes for goaway frame");
    }
    let (last_stream_id, msg_payload) = msg_payload.take_exact_err(4, "Expected 4 bytes for last stream id")?;
    let reserved_and_last_stream_id = last_stream_id.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let (error_code, msg_payload) = msg_payload.take_exact_err(4, "Expected 4 bytes for error code")?;
    let error_code = error_code.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let error_code = error_code.as_frame_err().map_err(|_| "Invalid error code")?;
    let additional_debug_data = msg_payload.to_vec();
    Ok(Http2InnerFrame::GoAway(Http2FrameGoaway {
        reserved_and_last_stream_id,
        error_code,
        additional_debug_data,
    }))
}
