mod error_codes;
mod frame_continuation;
mod frame_data;
mod frame_goaway;
mod frame_header;
mod frame_ping;
mod frame_priority;
mod frame_push_promise;
mod frame_reset_stream;
mod frame_settings;
mod frame_unknown;
mod frame_window_update;

use crate::server::parser::frame_continuation::Http2FrameContinuation;
use crate::server::parser::frame_data::Http2FrameData;
use crate::server::parser::frame_goaway::Http2FrameGoaway;
use crate::server::parser::frame_header::Http2FrameHeaders;
use crate::server::parser::frame_ping::Http2FramePing;
use crate::server::parser::frame_priority::Http2FramePriority;
use crate::server::parser::frame_push_promise::Http2FramePushPromise;
use crate::server::parser::frame_reset_stream::Http2FrameResetStream;
use crate::server::parser::frame_settings::Http2FrameSettings;
use crate::server::parser::frame_unknown::Http2FrameUnknown;
use crate::server::parser::frame_window_update::Http2FrameWindowUpdate;
use async_trait::async_trait;
use parser_helper::ParseHelper;
use proto_tower_util::{AsyncReadToBuf, ZeroReadBehaviour};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct Http2Frame {
    pub stream_id: u32,
    pub inner_frame: Http2InnerFrame,
}

#[derive(Debug)]
pub enum Http2InnerFrame {
    Data(Http2FrameData),
    Headers(Http2FrameHeaders),
    Priority(Http2FramePriority),
    RstStream(Http2FrameResetStream),
    Settings(Http2FrameSettings),
    PushPromise(Http2FramePushPromise),
    Ping(Http2FramePing),
    GoAway(Http2FrameGoaway),
    WindowUpdate(Http2FrameWindowUpdate),
    Continuation(Http2FrameContinuation),
    Unknown(Http2FrameUnknown),
}

impl Http2InnerFrame {
    pub fn len(&self) -> usize {
        match self {
            Http2InnerFrame::Data(frame) => frame.payload.len() + frame.padding.len() + 1,
            Http2InnerFrame::Headers(frame) => frame.header_block_fragment.len() + frame.padding.len() + 6,
            Http2InnerFrame::Priority(_) => 5,
            Http2InnerFrame::RstStream(_) => 4,
            Http2InnerFrame::Settings(frame) => 6 * frame.settings.len(),
            Http2InnerFrame::PushPromise(frame) => 5 + frame.header_block_fragment.len() + frame.padding.len(),
            Http2InnerFrame::Ping(_) => 8,
            Http2InnerFrame::GoAway(frame) => 8 + frame.additional_debug_data.len(),
            Http2InnerFrame::WindowUpdate(_) => 4,
            Http2InnerFrame::Continuation(frame) => frame.header_block_fragment.len(),
            Http2InnerFrame::Unknown(frame) => frame.payload.len(),
        }
    }
}

#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2InnerFrame {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        match self {
            Http2InnerFrame::Data(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::Headers(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::Priority(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::RstStream(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::Settings(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::PushPromise(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::Ping(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::GoAway(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::WindowUpdate(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::Continuation(frame) => frame.write_onto(writer).await,
            Http2InnerFrame::Unknown(frame) => frame.write_onto(writer).await,
        }
    }
}

pub trait Http2TypeToFrame {
    fn to_frame(&self, flags: u8, payload: &[u8]) -> Result<Http2InnerFrame, &'static str>;
}

#[derive(Debug)]
pub enum Http2FrameType {
    Data,
    Headers,
    Priority,
    RstStream,
    Settings,
    PushPromise,
    Ping,
    GoAway,
    WindowUpdate,
    Continuation,
    Unknown(u8),
}

impl Http2TypeToFrame for Http2FrameType {
    fn to_frame(&self, flags: u8, payload: &[u8]) -> Result<Http2InnerFrame, &'static str> {
        match self {
            Http2FrameType::Data => frame_data::read_data_frame(flags, payload),
            Http2FrameType::Headers => frame_header::read_header_frame(flags, payload),
            Http2FrameType::Priority => frame_priority::read_priority_frame(flags, payload),
            Http2FrameType::RstStream => frame_reset_stream::read_reset_stream_frame(flags, payload),
            Http2FrameType::Settings => frame_settings::read_settings_frame(flags, payload),
            Http2FrameType::PushPromise => frame_push_promise::read_push_promise_frame(flags, payload),
            Http2FrameType::Ping => frame_ping::read_ping_frame(flags, payload),
            Http2FrameType::GoAway => frame_goaway::read_goaway_frame(flags, payload),
            Http2FrameType::WindowUpdate => frame_window_update::read_window_update_frame(flags, payload),
            Http2FrameType::Continuation => frame_continuation::read_continuation_frame(payload),
            Http2FrameType::Unknown(frame_type) => frame_unknown::read_unknown_frame(*frame_type, flags, payload),
        }
    }
}

impl Http2FrameType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Http2FrameType::Data,
            1 => Http2FrameType::Headers,
            2 => Http2FrameType::Priority,
            3 => Http2FrameType::RstStream,
            4 => Http2FrameType::Settings,
            5 => Http2FrameType::PushPromise,
            6 => Http2FrameType::Ping,
            7 => Http2FrameType::GoAway,
            8 => Http2FrameType::WindowUpdate,
            9 => Http2FrameType::Continuation,
            _ => Http2FrameType::Unknown(value),
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            Http2FrameType::Data => 0,
            Http2FrameType::Headers => 1,
            Http2FrameType::Priority => 2,
            Http2FrameType::RstStream => 3,
            Http2FrameType::Settings => 4,
            Http2FrameType::PushPromise => 5,
            Http2FrameType::Ping => 6,
            Http2FrameType::GoAway => 7,
            Http2FrameType::WindowUpdate => 8,
            Http2FrameType::Continuation => 9,
            Http2FrameType::Unknown(value) => *value,
        }
    }
}

const MAX_FRAME_SIZE: usize = 16_384;

/// Read the next frame from the client
pub(crate) async fn read_next_frame<Reader: AsyncReadExt + Send + Unpin + 'static>(reader: &mut Reader, timeout: Duration) -> Result<Http2Frame, &'static str> {
    let async_reader = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
    let header = async_reader.read_with_timeout(reader, timeout, Some(9)).await;
    // 24 bits
    let length = header[0..3].iter().fold(0, |acc, &x| acc * 256 + x as u32);
    // 8 bits
    let frame_type = Http2FrameType::from_u8(header[3]);
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
    let inner_frame = frame_type.to_frame(flags, &payload)?;
    Ok(Http2Frame { stream_id, inner_frame })
}

#[async_trait]
pub trait WriteOnto<Writer>
where
    Writer: AsyncWriteExt + Send + Unpin,
{
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()>;
}

#[async_trait]
impl<Writer: AsyncWriteExt + Send + Unpin + 'static> WriteOnto<Writer> for Http2Frame {
    async fn write_onto(&self, writer: &mut Writer) -> Result<(), ()> {
        let length = self.inner_frame.len() as u32;
        writer.write_all(&length.to_be_bytes()).await.map_err(|_| ())?;
        let (flags, frame_type) = match &self.inner_frame {
            Http2InnerFrame::Data(d) => (d.flags.as_u8(), Http2FrameType::Data),
            Http2InnerFrame::Headers(d) => (d.flags.as_u8(), Http2FrameType::Headers),
            Http2InnerFrame::Priority(_) => (0, Http2FrameType::Priority),
            Http2InnerFrame::RstStream(_) => (0, Http2FrameType::RstStream),
            Http2InnerFrame::Settings(d) => (d.flags.as_u8(), Http2FrameType::Settings),
            Http2InnerFrame::PushPromise(d) => (d.flags.as_u8(), Http2FrameType::PushPromise),
            Http2InnerFrame::Ping(d) => (d.flags, Http2FrameType::Ping),
            Http2InnerFrame::GoAway(_) => (0, Http2FrameType::GoAway),
            Http2InnerFrame::WindowUpdate(_) => (0, Http2FrameType::WindowUpdate),
            Http2InnerFrame::Continuation(_) => (0, Http2FrameType::Continuation),
            Http2InnerFrame::Unknown(d) => (0, Http2FrameType::Unknown(d.frame_type)),
        };
        writer.write_all(&frame_type.into_u8().to_be_bytes()).await.map_err(|_| ())?;
        writer.write_all(&flags.to_be_bytes()).await.map_err(|_| ())?;
        self.inner_frame.write_onto(writer).await
    }
}
