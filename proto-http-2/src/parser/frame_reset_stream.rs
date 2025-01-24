use crate::parser::Http2Frame;

#[derive(Debug)]
pub struct Http2FrameResetStream {
    pub error_code: u32,
}

pub fn read_reset_stream_frame(_flags: u8, msg_payload: &[u8]) -> Result<Http2Frame, &'static str> {
    if msg_payload.len() != 4 {
        return Err("Expected 4 bytes for reset stream frame");
    }
    let error_code = msg_payload.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    Ok(Http2Frame::RstStream(Http2FrameResetStream { error_code }))
}
