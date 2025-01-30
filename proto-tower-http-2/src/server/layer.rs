use crate::server::parser::{read_next_frame, Http2Frame, WriteOnto};
use crate::ProtoHttp2Config;
use proto_tower_util::{AsyncReadToBuf, ZeroReadBehaviour};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{Receiver, Sender};
use tower::Service;

#[derive(Debug)]
pub enum ProtoHttp2Error<Error: Debug> {
    InvalidPreface,
    Timeout,
    InnerServiceClosed,
    ServiceError(Error),
    OtherInternalError(&'static str),
}

/// A service to process HTTP/1.1 requests
///
/// This should not be constructed directly - it gets created by MakeService during invocation.
pub struct ProtoHttp2Layer<Svc>
where
    Svc: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = ()> + Send + Clone,
{
    config: ProtoHttp2Config,
    /// The inner service to process requests
    inner: Svc,
}

impl<Svc> ProtoHttp2Layer<Svc>
where
    Svc: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = ()> + Send + Clone,
{
    /// Create a new instance of the service
    pub fn new(config: ProtoHttp2Config, inner: Svc) -> Self {
        ProtoHttp2Layer { config, inner }
    }
}

impl<Reader, Writer, Svc, SvcError, SvcFut> Service<(Reader, Writer)> for ProtoHttp2Layer<Svc>
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    Svc: Service<(Receiver<Http2Frame>, Sender<Http2Frame>), Response = (), Error = SvcError, Future = SvcFut> + Send + Clone + 'static,
    SvcFut: Future<Output = Result<(Receiver<Http2Frame>, Sender<Http2Frame>), SvcError>> + Send + 'static,
    SvcError: Debug + Send + 'static,
{
    /// The response is handled by the protocol
    type Response = ();
    /// Errors would be failures in parsing the protocol - this should be handled by the protocol
    type Error = ProtoHttp2Error<SvcError>;
    /// The future is the protocol itself
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(ProtoHttp2Error::ServiceError)
    }

    /// Indefinitely process the protocol
    fn call(&mut self, (mut reader, mut writer): (Reader, Writer)) -> Self::Future {
        let mut service = self.inner.clone();
        let config = self.config.clone();
        Box::pin(async move {
            let async_read = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
            let mut preface = async_read.read_with_timeout(&mut reader, config.timeout, Some(28)).await;
            if preface != b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n" {
                return Err(ProtoHttp2Error::InvalidPreface);
            }
            let (layer_frame_sx, svc_frame_rx) = tokio::sync::mpsc::channel::<Http2Frame>(1);
            let (svc_frame_sx, mut layer_frame_rx) = tokio::sync::mpsc::channel::<Http2Frame>(1);
            let internal_task = tokio::spawn(service.call((svc_frame_rx, svc_frame_sx)));
            // Validate request
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(config.timeout) => {
                        return Err(ProtoHttp2Error::Timeout);
                    }
                    frame = layer_frame_rx.recv() => {
                        match frame {
                            Some(frame) => {
                                frame.write_onto(&mut writer).await.unwrap();
                            }
                            None => {
                                return Err(ProtoHttp2Error::InnerServiceClosed);
                            }
                        }
                    }
                    frame = read_next_frame(&mut reader, config.timeout) => {
                        match frame {
                            Ok(frame) => {
                                layer_frame_sx.send(frame).await.unwrap();
                            }
                            Err(e) => {
                                return Err(ProtoHttp2Error::OtherInternalError(e));
                            }
                        }
                    }
                }
                if internal_task.is_finished() {
                    return match internal_task.await {
                        Ok(Err(e)) => Err(ProtoHttp2Error::ServiceError(e)),
                        Ok(Ok(_)) | Err(_) => Err(ProtoHttp2Error::InnerServiceClosed),
                    };
                }
            }
        })
    }
}
