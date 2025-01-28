use crate::server::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FrameWindowUpdate {
    pub reserved_and_window_size_increment: u32,
}

#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FrameWindowUpdate {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        todo!()
    }
}

pub fn read_window_update_frame(_flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    if msg_payload.len() != 4 {
        return Err("Expected 4 bytes for window update frame");
    }
    let reserved_and_window_size_increment = msg_payload.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    Ok(Http2InnerFrame::WindowUpdate(Http2FrameWindowUpdate {
        reserved_and_window_size_increment,
    }))
}
