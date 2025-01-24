use crate::parser::Http2Frame;
use parser_helper::ParseHelper;

#[derive(Debug)]
pub struct Http2FrameGoaway {
    pub reserved_and_last_stream_id: u32,
    pub error_code: u32,
    pub additional_debug_data: Vec<u8>,
}

pub fn read_goaway_frame(_flags: u8, msg_payload: &[u8]) -> Result<Http2Frame, &'static str> {
    if msg_payload.len() < 8 {
        return Err("Expected at least 8 bytes for goaway frame");
    }
    let (last_stream_id, msg_payload) = msg_payload.take_exact_err(4, "Expected 4 bytes for last stream id")?;
    let reserved_and_last_stream_id = last_stream_id.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let (error_code, msg_payload) = msg_payload.take_exact_err(4, "Expected 4 bytes for error code")?;
    let error_code = error_code.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let additional_debug_data = msg_payload.to_vec();
    Ok(Http2Frame::GoAway(Http2FrameGoaway {
        reserved_and_last_stream_id,
        error_code,
        additional_debug_data,
    }))
}
