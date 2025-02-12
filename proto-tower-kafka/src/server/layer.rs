use crate::data::inner_response::{TrackedKafkaRequest, TrackedKafkaResponse};
use crate::data::KafkaProtocolError;
use crate::server::parser::parse_kafka_request;
use crate::server::KafkaProtoServerConfig;
use bytes::{Buf, BytesMut};
use kafka_protocol::protocol::buf::ByteBuf;
use proto_tower_util::debug::debug_hex;
use proto_tower_util::{CountOrDuration, TimeoutCounter, WriteTo};
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{Receiver, Sender};
use tower::Service;

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
#[derive(Debug, Clone)]
pub struct ProtoKafkaServerService<Svc>
where
    Svc: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = ()> + Send + Clone,
{
    config: KafkaProtoServerConfig,
    /// The inner service to process requests
    inner: Svc,
}

impl<Svc> ProtoKafkaServerService<Svc>
where
    Svc: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: KafkaProtoServerConfig, inner: Svc) -> Self {
        ProtoKafkaServerService { config, inner }
    }
}

impl<Reader, Writer, Svc, SvcError, SvcFut> Service<(Reader, Writer)> for ProtoKafkaServerService<Svc>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Receiver<TrackedKafkaRequest>, Sender<TrackedKafkaResponse>), Response = (), Error = SvcError, Future = SvcFut> + Send + Clone + 'static,
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
    fn call(&mut self, (mut input_reader, mut input_writer): (Reader, Writer)) -> Self::Future {
        let mut service = self.inner.clone();
        let config = self.config.clone();
        Box::pin(async move {
            // Start the downstream call
            let (svc_sx, mut downstream_rx) = tokio::sync::mpsc::channel::<TrackedKafkaResponse>(1024);
            let (downstream_sx, svc_rx) = tokio::sync::mpsc::channel::<TrackedKafkaRequest>(1024);
            let svc_fut = tokio::spawn(service.call((svc_rx, svc_sx)));

            // Track correlation_id to api_version
            let mut tracked_requests = BTreeMap::<i32, i16>::new();
            let mut inbound_read_buffer = BytesMut::new();
            let mut inbound_read_temp_buffer = [0u8; 1024];
            let inbound_timeout = TimeoutCounter::new(CountOrDuration::Count(10), CountOrDuration::Duration(config.timeout));
            loop {
                tokio::select! {
                    // Inner service sending responses
                    r = downstream_rx.recv() => {
                        match r {
                            None => {
                                return Err(KafkaProtocolError::InternalServiceClosed);
                            }
                            Some(resp) => {
                                eprintln!("Tracked_requests: {:?}", tracked_requests);
                                let api_version = tracked_requests.remove(&resp.correlation_id).ok_or(KafkaProtocolError::UnhandledImplementation("Received message with unmatched correlation_id"))?;
                                eprintln!("Sending response: {:?}", resp);
                                let resp = resp.into_inner(api_version);
                                resp.write_to(&mut input_writer).await?;
                            }
                        }
                    }
                    // Protocol sending requests
                    sz = input_reader.read(&mut inbound_read_temp_buffer) => {
                        match sz {
                            Ok(0) => {
                                // noop, but might indicate termination
                                eprintln!("Zero read on kafka server layer");
                                return Err(KafkaProtocolError::InternalServiceClosed);
                            }
                            Ok(sz) => {
                                inbound_timeout.reset();
                                inbound_read_buffer.extend_from_slice(&inbound_read_temp_buffer[..sz]);
                                if let Some(mut mut_buf) = check_valid_packet(&mut inbound_read_buffer) {
                                    let res = parse_kafka_request(&mut mut_buf, &mut tracked_requests);
                                    match res {
                                        Ready(resp) => {
                                            match resp {
                                                Ok((header, resp)) => {
                                                    let resp = TrackedKafkaRequest{correlation_id: header.correlation_id,request: resp};
                                                    // TODO we should retain protocol info from the header
                                                    if let Err(_) = downstream_sx.send(resp.clone()).await {
                                                        return Err(KafkaProtocolError::InternalServiceClosed);
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("Buffer:\n{}", debug_hex(&mut_buf));
                                                    eprintln!("Error parsing request: {:?}", e);
                                                    // No-op, not enough data. Assuming parsing is valid.
                                                }
                                            }
                                        }
                                        Pending => {
                                                eprintln!("Pending");
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error reading: {:?}", e);
                                return Err(KafkaProtocolError::InternalServiceClosed);
                            }
                        }
                    }
                    e = inbound_timeout.next_timeout() => {
                        svc_fut.abort();
                        e.map_err(|_| KafkaProtocolError::Timeout)?;
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

fn check_valid_packet(buff: &mut BytesMut) -> Option<BytesMut> {
    let sz = Buf::try_get_i32(&mut buff.peek_bytes(0..4)).ok()?;
    let sz = sz as usize;
    if buff.len() < sz + 4 {
        return None;
    }
    let mut mut_buf = BytesMut::new();
    mut_buf.extend(buff.try_get_bytes(sz + 4).ok()?);
    Some(mut_buf)
}
