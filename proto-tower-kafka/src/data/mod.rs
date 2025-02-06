mod header;
mod request;
mod response;

pub use header::ResponseHeaderComplete;
pub use header::ResponseHeaderIntermediary;
pub use request::KafkaRequest;
pub use response::KafkaResponse;

use std::fmt::Debug;

#[derive(Debug)]
pub enum KafkaProtocolError<E: Debug> {
    UnhandledImplementation(&'static str),
    InternalServiceError(E),
    InternalServiceClosed,
    Timeout,
}

#[macro_export]
macro_rules! encode_and_write_response {
    ($inner:ident, $buff_mut:ident, $writer:ident, $version:ident) => {{
        $inner
            .encode(&mut $buff_mut, $version)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response encode failure"))?;
        let sz = $buff_mut.len() as i32;
        let sz_bytes: [u8; 4] = sz.to_be_bytes();
        $writer
            .write_all(&sz_bytes)
            .await
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response size write failure"))?;
        $writer
            .write_all(&$buff_mut)
            .await
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response payload write failure"))?;
        Ok(())
    }};
}

#[macro_export]
macro_rules! encode_and_write_request {
    ($inner:ident, $writer:ident, $version:ident, $correlation:ident) => {{
        let mut buff_mut = BytesMut::new();
        $inner
            .encode(&mut buff_mut, $version)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response encode failure"))?;
        let sz = buff_mut.len() as i32;
        let sz_bytes: [u8; 4] = sz.to_be_bytes();
        $writer
            .write_all(&sz_bytes)
            .await
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response size write failure"))?;
        $writer
            .write_all(&buff_mut)
            .await
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response payload write failure"))?;
        Ok(())
    }};
}
