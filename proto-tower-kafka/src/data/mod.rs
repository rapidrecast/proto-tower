mod header;
mod request;
mod response;

pub use header::ResponseHeaderComplete;
pub use header::ResponseHeaderIntermediary;
pub use request::KafkaRequest;
pub use response::KafkaResponse;
pub use response::ProtoInfo;

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
    ($proto_info:expr, $inner:ident, $writer:ident) => {{
        let mut buff_mut = BytesMut::new();
        // Produce the header
        let (correlation_id, version) = ($proto_info.correlation_id, $proto_info.api_version);
        let header = ResponseHeader::default().with_correlation_id(correlation_id);
        // TODO: header version?
        header
            .encode(&mut buff_mut, version)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response header encode failure"))?;
        // Produce the response
        $inner
            .encode(&mut buff_mut, version)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response encode failure"))?;
        let sz = buff_mut.len() as i32;
        let sz_bytes: [u8; 4] = sz.to_be_bytes();
        $writer
            .write_all(&sz_bytes)
            .await
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response size write failure"))?;
        // Write the entire response
        $writer
            .write_all(&buff_mut)
            .await
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response payload write failure"))?;
        Ok(())
    }};
}

#[macro_export]
macro_rules! encode_and_write_request {
    ($inner:ident, $writer:ident, $version:ident, $correlation:ident, $client_id:expr) => {{
        let mut buff_mut = BytesMut::new();

        // Special handling for ApiVersionsRequest
        let key = get_api_key($inner);
        let header = RequestHeader::default()
            .with_request_api_key(key)
            .with_request_api_version($version)
            .with_correlation_id(*$correlation)
            .with_client_id($client_id.map(|x| x.into()));

        header
            .encode(&mut buff_mut, $version)
            .map_err(|_| KafkaProtocolError::UnhandledImplementation("Response header encode failure"))?;

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
