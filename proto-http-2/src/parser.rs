use proto_tower::{AsyncReadToBuf, ZeroReadBehaviour};
use std::time::Duration;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub enum Http2Frame {}

const MAX_FRAME_SIZE: usize = 16_384;

/// Read the next frame from the client
pub(crate) async fn read_next_frame<Reader: AsyncReadExt + Send + Unpin + 'static>(reader: &mut Reader, timeout: Duration) -> Result<Http2Frame, &'static str> {
    let async_reader = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let header = async_reader.read_with_timeout(reader, timeout, Some(9)).await;
    // 24 bits
    let length = header[0..3].iter().fold(0, |acc, &x| acc * 256 + x as u32);
    // 8 bits
    let frame_type = header[3];
    // 8 bits
    let flags = header[4];
    // 31 bits (1 bit reserved)
    let stream_id = header[5..9].iter().fold(0, |acc, &x| acc * 256 + x as u32);
    // we probably don't need the reserved bit
    //    R: A reserved 1-bit field.  The semantics of this bit are undefined,
    //       and the bit MUST remain unset (0x0) when sending and MUST be
    //       ignored when receiving.
    let _reserved = ((stream_id & 0x80000000) >> 31) as u8;
    let stream_id = stream_id & 0x7FFFFFFF;
    if length > MAX_FRAME_SIZE as u32 {
        return Err("Frame too large");
    }
    let payload = async_reader.read_with_timeout(reader, timeout, Some(length as usize)).await;
    Err("Not implemented")
}
