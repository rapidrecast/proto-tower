use crate::parser::Http2Frame;
use parser_helper::ParseHelper;

#[derive(Debug)]
pub struct Http2FramePriority {
    pub exclusive_and_stream_dependency: u32,
    pub weight: u8,
}

pub fn read_priority_frame(flags: u8, msg_payload: &[u8]) -> Result<Http2Frame, &'static str> {
    if msg_payload.len() != 5 {
        return Err("Expected 5 bytes for priority frame");
    }
    if flags != 0 {
        return Err("Priority frame must have flags set to 0");
    }
    let (exclusive_and_stream_dependency, msg_payload) = msg_payload.take_exact_err(4, "Expected 4 bytes exclusive and stream dependency")?;
    let exclusive_and_stream_dependency = exclusive_and_stream_dependency.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let (weight, _) = msg_payload.take_exact_err(1, "Expected 1 byte weight")?;
    Ok(Http2Frame::Priority(Http2FramePriority {
        exclusive_and_stream_dependency,
        weight: weight[0],
    }))
}
