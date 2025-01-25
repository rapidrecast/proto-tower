use crate::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use parser_helper::ParseHelper;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FramePriority {
    pub exclusive_and_stream_dependency: u32,
    pub weight: u8,
}
#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FramePriority {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        todo!()
    }
}

pub fn read_priority_frame(flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    if msg_payload.len() != 5 {
        return Err("Expected 5 bytes for priority frame");
    }
    if flags != 0 {
        return Err("Priority frame must have flags set to 0");
    }
    let (exclusive_and_stream_dependency, msg_payload) = msg_payload.take_exact_err(4, "Expected 4 bytes exclusive and stream dependency")?;
    let exclusive_and_stream_dependency = exclusive_and_stream_dependency.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let (weight, _) = msg_payload.take_exact_err(1, "Expected 1 byte weight")?;
    Ok(Http2InnerFrame::Priority(Http2FramePriority {
        exclusive_and_stream_dependency,
        weight: weight[0],
    }))
}
