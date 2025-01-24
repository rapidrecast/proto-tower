use crate::parser::Http2Frame;
use parser_helper::ParseHelper;

#[derive(Debug)]
pub struct Http2FrameHeaders {
    pub flags: Http2FrameHeadersFlags,
    exclusive_and_stream_dependency: u32,
    pub weight: u8,
    pub header_block_fragment: Vec<u8>,
    pub padding: Vec<u8>,
}

#[derive(Debug)]
pub struct Http2FrameHeadersFlags {
    pub flags: u8,
}

impl Http2FrameHeadersFlags {
    pub fn from_u8(value: u8) -> Self {
        Http2FrameHeadersFlags { flags: value }
    }

    pub fn as_u8(&self) -> u8 {
        self.flags
    }

    pub fn get_end_stream(&self) -> bool {
        self.flags & 0x1 == 0x1
    }

    pub fn get_padded(&self) -> bool {
        self.flags & 0x8 == 0x8
    }

    pub fn get_priority(&self) -> bool {
        self.flags & 0x20 == 0x20
    }

    pub fn set_end_stream(&mut self, end_stream: bool) {
        if end_stream {
            self.flags |= 0x1;
        } else {
            self.flags &= !0x1;
        }
    }

    pub fn set_padded(&mut self, padded: bool) {
        if padded {
            self.flags |= 0x8;
        } else {
            self.flags &= !0x8;
        }
    }

    pub fn set_priority(&mut self, priority: bool) {
        if priority {
            self.flags |= 0x20;
        } else {
            self.flags &= !0x20;
        }
    }
}

pub fn read_header_frame(flags: u8, msg_payload: &[u8]) -> Result<Http2Frame, &'static str> {
    let (pad_length, msg_payload) = msg_payload.take_exact_err(1, "Expected 1 byte padding length")?;
    let pad_length = pad_length[0] as usize;
    let (exclusive_and_stream_dependency, msg_payload) = msg_payload.take_exact_err(4, "Expected 4 bytes for exclusive flag and stream dependency")?;
    let (weight, msg_payload) = msg_payload.take_exact_err(1, "Expected 1 byte for weight")?;
    let weight = weight[0];
    let message_length = msg_payload.len() - pad_length;
    let (header_block_fragment, msg_payload) = msg_payload.take_exact_err(message_length, "Expected header block fragment")?;
    let padding = msg_payload;
    Ok((Http2Frame::Headers(Http2FrameHeaders {
        flags: Http2FrameHeadersFlags::from_u8(flags),
        exclusive_and_stream_dependency: u32::from_be_bytes(exclusive_and_stream_dependency.try_into().unwrap()),
        weight,
        header_block_fragment: header_block_fragment.to_vec(),
        padding: padding.to_vec(),
    })))
}
