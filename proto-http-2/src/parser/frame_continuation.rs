use crate::parser::Http2Frame;

#[derive(Debug)]
pub struct Http2FrameContinuation {
    pub header_block_fragment: Vec<u8>,
}

pub fn read_continuation_frame(msg_payload: &[u8]) -> Result<Http2Frame, &'static str> {
    let header_block_fragment = msg_payload.to_vec();
    Ok(Http2Frame::Continuation(Http2FrameContinuation { header_block_fragment }))
}
