use crate::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use parser_helper::ParseHelper;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FramePushPromise {
    pub flags: Http2FramePushPromiseFlags,
    pub reserved_and_promised_stream_id: u32,
    pub header_block_fragment: Vec<u8>,
    pub padding: Vec<u8>,
}
#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FramePushPromise {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Http2FramePushPromiseFlags {
    flags: u8,
}

impl Http2FramePushPromiseFlags {
    pub fn from_u8(value: u8) -> Self {
        Http2FramePushPromiseFlags { flags: value }
    }

    pub fn as_u8(&self) -> u8 {
        self.flags
    }

    pub fn get_end_headers(&self) -> bool {
        self.flags & 0x4 == 0x4
    }

    pub fn get_padded(&self) -> bool {
        self.flags & 0x8 == 0x8
    }

    pub fn set_end_headers(&mut self, end_headers: bool) {
        if end_headers {
            self.flags |= 0x4;
        } else {
            self.flags &= !0x4;
        }
    }

    pub fn set_padded(&mut self, padded: bool) {
        if padded {
            self.flags |= 0x8;
        } else {
            self.flags &= !0x8;
        }
    }
}

pub fn read_push_promise_frame(_flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    let (pad_length, msg_payload) = msg_payload.take_exact_err(1, "Expected 1 byte padding length")?;
    let pad_length = pad_length[0] as usize;
    let (reserved_and_promised_stream_id, msg_payload) = msg_payload.take_exact_err(4, "Expected 4 bytes reserved and promised stream id")?;
    let reserved_and_promised_stream_id = reserved_and_promised_stream_id.iter().fold(0, |acc, &x| acc * 256 + x as u32);
    let message_length = msg_payload.len() - pad_length;
    let (header_block_fragment, msg_payload) = msg_payload.take_exact_err(message_length, "Expected header block fragment")?;
    let header_block_fragment = header_block_fragment.to_vec();
    let padding = msg_payload.to_vec();
    Ok(Http2InnerFrame::PushPromise(Http2FramePushPromise {
        flags: Http2FramePushPromiseFlags { flags: 0 },
        reserved_and_promised_stream_id,
        header_block_fragment,
        padding,
    }))
}
