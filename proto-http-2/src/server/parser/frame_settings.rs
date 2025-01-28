use crate::server::parser::{Http2InnerFrame, WriteOnto};
use async_trait::async_trait;
use parser_helper::ParseHelper;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Http2FrameSettings {
    pub flags: Http2FrameSettingsFlags,
    pub settings: Vec<Http2SettingsEntry>,
}
#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2FrameSettings {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Http2SettingsEntry {
    pub identifier: SettingsIdentifier,
    pub value: u32,
}

#[derive(Debug)]
pub enum SettingsIdentifier {
    HeaderTableSize,
    EnablePush,
    MaxConcurrentStreams,
    InitialWindowSize,
    MaxFrameSize,
    MaxHeaderListSize,
}

impl SettingsIdentifier {
    fn from_u16(value: u16) -> Result<Self, ()> {
        match value {
            0x1 => Ok(SettingsIdentifier::HeaderTableSize),
            0x2 => Ok(SettingsIdentifier::EnablePush),
            0x3 => Ok(SettingsIdentifier::MaxConcurrentStreams),
            0x4 => Ok(SettingsIdentifier::InitialWindowSize),
            0x5 => Ok(SettingsIdentifier::MaxFrameSize),
            0x6 => Ok(SettingsIdentifier::MaxHeaderListSize),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Http2FrameSettingsFlags {
    pub flags: u8,
}

impl Http2FrameSettingsFlags {
    pub fn from_u8(value: u8) -> Self {
        Http2FrameSettingsFlags { flags: value }
    }

    pub fn as_u8(&self) -> u8 {
        self.flags
    }

    pub fn get_ack(&self) -> bool {
        self.flags & 0x1 == 0x1
    }

    pub fn set_ack(&mut self, ack: bool) {
        if ack {
            self.flags |= 0x1;
        } else {
            self.flags &= !0x1;
        }
    }
}

pub fn read_settings_frame(flags: u8, mut msg_payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
    let flags = Http2FrameSettingsFlags::from_u8(flags);
    if flags.get_ack() && msg_payload.len() > 0 {
        return Err("Settings frame with ACK flag must have empty payload");
    }
    if msg_payload.len() % 6 != 0 {
        return Err("Settings frame payload must be multiple of 6 bytes");
    }
    let mut settings = Vec::with_capacity(msg_payload.len() / 6);
    // TODO this can be improved with rayon or simd - read 6 bytes then transform block
    while msg_payload.len() > 0 {
        let (identifier, new_payload) = msg_payload.take_exact_err(2, "Expected 2 bytes for identifier")?;
        let identifier = identifier.iter().fold(0, |acc, &x| acc * 256 + x as u16);
        let identifier = SettingsIdentifier::from_u16(identifier).map_err(|_| "Invalid identifier")?;
        let (value, new_payload) = new_payload.take_exact_err(4, "Expected 4 bytes for value")?;
        let value = value.iter().fold(0, |acc, &x| acc * 256 + x as u32);
        settings.push(Http2SettingsEntry { identifier, value });
        msg_payload = new_payload;
    }
    Ok(Http2InnerFrame::Settings(Http2FrameSettings { flags, settings }))
}
