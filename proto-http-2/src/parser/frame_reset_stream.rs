use crate::parser::error_codes::{FrameError, FrameErrorCode};
use crate::parser::Http2InnerFrame;

#[derive(Debug)]
pub struct Http2FrameResetStream {
    pub error_code: FrameError,
}

pub fn read_reset_stream_frame(_flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    if msg_payload.len() != 4 {
        return Err("Expected 4 bytes for reset stream frame");
    }
    let error_code = msg_payload.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let error_code = error_code.as_frame_err().map_err(|_| "Invalid error code")?;
    Ok(Http2InnerFrame::RstStream(Http2FrameResetStream { error_code }))
}
