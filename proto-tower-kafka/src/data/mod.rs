mod request;
mod response;

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
