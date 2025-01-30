use crate::data::{KafkaProtocolError, KafkaRequest, KafkaResponse};
use crate::server::parser::parse_kafka_request;
use crate::server::KafkaProtoServerConfig;
use bytes::BytesMut;
use proto_tower_util::{AsyncReadToBuf, WriteTo, ZeroReadBehaviour};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::mpsc::{Receiver, Sender};
use tower::Service;

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
pub struct ProtoKafkaServerLayer<Svc>
where
    Svc: Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>), Response = ()> + Send + Clone,
{
    config: KafkaProtoServerConfig,
    /// The inner service to process requests
    inner: Svc,
}

impl<Svc> ProtoKafkaServerLayer<Svc>
where
    Svc: Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: KafkaProtoServerConfig, inner: Svc) -> Self {
        ProtoKafkaServerLayer { config, inner }
    }
}

impl<Reader, Writer, Svc, SvcError, SvcFut> Service<(Reader, Writer)> for ProtoKafkaServerLayer<Svc>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Receiver<KafkaRequest>, Sender<KafkaResponse>), Response = (), Error = SvcError, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<(), SvcError>> + Send + 'static,
    SvcError: std::fmt::Debug + Send + 'static,
{
    /// The response is handled by the protocol
    type Response = ();
    /// Errors would be failures in parsing the protocol - this should be handled by the protocol
    type Error = KafkaProtocolError<SvcError>;
    /// The future is the protocol itself
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| KafkaProtocolError::InternalServiceError(e))
    }

    /// Indefinitely process the protocol
    fn call(&mut self, (reader, writer): (Reader, Writer)) -> Self::Future {
        let mut service = self.inner.clone();
        let config = self.config.clone();
        Box::pin(async move {
            // Buffer readers and writers
            let mut reader = BufReader::new(reader);
            let mut writer = BufWriter::new(writer);

            // Start the downstream call
            let (svc_write, mut read) = tokio::sync::mpsc::channel::<KafkaResponse>(1024);
            let (mut write, svc_read) = tokio::sync::mpsc::channel::<KafkaRequest>(1024);
            let svc_fut = tokio::spawn(service.call((svc_read, svc_write)));

            let mut data = Vec::with_capacity(1024);
            let read_buf = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
            loop {
                tokio::select! {
                    // Inner service sending responses
                    r = read.recv() => {
                        match r {
                            None => {
                                return Err(KafkaProtocolError::InternalServiceClosed);
                            }
                            Some(resp) => {
                                eprintln!("Sending response: {:?}", resp);
                                resp.write_to(&mut writer).await?;
                            }
                        }
                    }
                    // Protocol sending requests
                    r = read_buf.read_with_timeout(&mut reader, config.timeout, None) => {
                        if r.is_empty() {
                            return Err(KafkaProtocolError::Timeout);
                        }
                        data.extend_from_slice(&r);
                        let mut mut_buf = BytesMut::new();
                        mut_buf.extend_from_slice(&data);
                        let res = parse_kafka_request(&mut mut_buf);
                        match res {
                            Ok(resp) => {
                                let sz = data.len()-mut_buf.len();
                                data.drain(..sz);
                                if let Err(e) = write.send(resp).await {
                                    return Err(KafkaProtocolError::InternalServiceClosed);
                                }
                            }
                            Err(e) => {
                                eprintln!("Error parsing request: {:?}", e);
                                // No-op, not enough data. Assuming parsing is valid.
                            }
                        }
                    }
                }
                if svc_fut.is_finished() {
                    // Service finished
                    let res = svc_fut.await;
                    return match res {
                        Ok(Err(e)) => Err(KafkaProtocolError::InternalServiceError(e)),
                        Ok(Ok(_)) | Err(_) => Err(KafkaProtocolError::InternalServiceClosed),
                    };
                }
            }
        })
    }
}
