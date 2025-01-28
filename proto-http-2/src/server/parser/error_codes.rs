pub trait FrameErrorCode {
    fn as_frame_err(&self) -> Result<FrameError, ()>;
    fn as_u32(&self) -> u32;
}

#[derive(Debug)]
pub enum FrameError {
    NoError,
    ProtocolError,
    InternalError,
    FlowControlError,
    SettingsTimeout,
    StreamClosed,
    FrameSizeError,
    RefusedStream,
    Cancel,
    CompressionError,
    ConnectError,
    EnhanceYourCalm,
    InadequateSecurity,
    Http11Required,
}

impl FrameErrorCode for u32 {
    fn as_frame_err(&self) -> Result<FrameError, ()> {
        match self {
            0 => Ok(FrameError::NoError),
            1 => Ok(FrameError::ProtocolError),
            2 => Ok(FrameError::InternalError),
            3 => Ok(FrameError::FlowControlError),
            4 => Ok(FrameError::SettingsTimeout),
            5 => Ok(FrameError::StreamClosed),
            6 => Ok(FrameError::FrameSizeError),
            7 => Ok(FrameError::RefusedStream),
            8 => Ok(FrameError::Cancel),
            9 => Ok(FrameError::CompressionError),
            10 => Ok(FrameError::ConnectError),
            11 => Ok(FrameError::EnhanceYourCalm),
            12 => Ok(FrameError::InadequateSecurity),
            13 => Ok(FrameError::Http11Required),
            _ => Err(()),
        }
    }

    fn as_u32(&self) -> u32 {
        *self
    }
}
