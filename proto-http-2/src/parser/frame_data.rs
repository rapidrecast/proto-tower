use crate::parser::Http2InnerFrame;
use parser_helper::ParseHelper;

#[derive(Debug)]
pub struct Http2FrameData {
    pub flags: Http2FrameDataFlags,
    pub payload: Vec<u8>,
    pub padding: Vec<u8>,
}

#[derive(Debug)]
pub struct Http2FrameDataFlags {
    pub flags: u8,
}

impl Http2FrameDataFlags {
    pub fn from_u8(value: u8) -> Self {
        Http2FrameDataFlags { flags: value }
    }
    pub fn as_u8(&self) -> u8 {
        self.flags
    }
    pub fn flags_get_end_stream(&self) -> bool {
        self.flags & 0x1 == 0x1
    }
    pub fn flags_get_padded(&self) -> bool {
        self.flags & 0x8 == 0x8
    }
    pub fn flags_set_end_stream(&mut self, end_stream: bool) {
        if end_stream {
            self.flags |= 0x1;
        } else {
            self.flags &= !0x1;
        }
    }

    pub fn flags_set_padded(&mut self, padded: bool) {
        if padded {
            self.flags |= 0x8;
        } else {
            self.flags &= !0x8;
        }
    }
}

pub fn read_data_frame(flags: u8, msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    let (pad_length, msg_payload) = msg_payload.take_exact_err(1, "Expected 1 byte padding length")?;
    let pad_length = pad_length[0] as usize;
    let message_length = msg_payload.len() - pad_length;
    let (payload, msg_payload) = msg_payload.take_exact_err(message_length, "Expected payload")?;
    let padding = msg_payload;
    Ok(Http2InnerFrame::Data(Http2FrameData {
        flags: Http2FrameDataFlags::from_u8(flags),
        payload: payload.to_vec(),
        padding: padding.to_vec(),
    }))
}
