use crate::server::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FramePing {
    pub flags: u8,
    pub opaque_data: [u8; 8],
}
#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FramePing {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        todo!()
    }
}

pub fn read_ping_frame(flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    if msg_payload.len() != 8 {
        return Err("Expected 8 bytes for ping frame");
    }
    let mut opaque_data = [0; 8];
    for i in 0..8 {
        opaque_data[i] = msg_payload[i];
    }
    Ok(Http2InnerFrame::Ping(Http2FramePing { flags, opaque_data }))
}
