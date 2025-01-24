use crate::parser::Http2InnerFrame;

#[derive(Debug)]
pub struct Http2FrameUnknown {
    pub frame_type: u8,
    pub flags: u8,
    pub payload: Vec<u8>,
}

pub fn read_unknown_frame(frame_type: u8, flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    Ok(Http2InnerFrame::Unknown(Http2FrameUnknown {
        frame_type,
        flags,
        payload: msg_payload.to_vec(),
    }))
}
