use crate::server::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FrameUnknown {
    pub frame_type: u8,
    pub flags: u8,
    pub payload: Vec<u8>,
}
#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FrameUnknown {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        todo!()
    }
}

pub fn read_unknown_frame(frame_type: u8, flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    Ok(Http2InnerFrame::Unknown(Http2FrameUnknown {
        frame_type,
        flags,
        payload: msg_payload.to_vec(),
    }))
}
