//! Proto-tower-util is a collection of utilities that are used across the proto-tower project.

mod bytes_mut_ext;
mod chan;
pub mod debug;
mod debug_io_layer;
mod test_io_service;
mod timout_counter;
mod write_to;

pub use bytes_mut_ext::BytesMutHelper;
pub use chan::sx_rx_chans;
pub use debug_io_layer::{DebugIoLayer, DebugIoService};
pub use test_io_service::TestIoService;
pub use timout_counter::{CountOrDuration, NextTimeout, TimeoutCounter};
pub use write_to::WriteTo;

use std::cmp::min;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

pub enum ZeroReadBehaviour {
    TickAndYield,
    TickSleep,
    // TickMeasure,
}

/// A helper to read data within a timeout
/// This handles both the cases where the read(&mut buffer) may not return any data or constantly return len 0
pub struct AsyncReadToBuf<const S: usize> {
    zero_read_behaviour: ZeroReadBehaviour,
}

impl AsyncReadToBuf<1024> {
    pub const fn new_1024(zero_read_behaviour: ZeroReadBehaviour) -> AsyncReadToBuf<1024> {
        AsyncReadToBuf::<1024> { zero_read_behaviour }
    }
}

impl<const S: usize> AsyncReadToBuf<S> {
    pub const fn new(zero_read_behaviour: ZeroReadBehaviour) -> Self {
        AsyncReadToBuf::<S> { zero_read_behaviour }
    }

    pub async fn read_until<READ: AsyncReadExt + Unpin + Send + 'static>(&self, reader: &mut BufReader<READ>, byte: u8) -> Vec<u8> {
        let mut buf = Vec::with_capacity(S);
        reader.read_until(byte, &mut buf).await.unwrap();
        buf
    }
    pub async fn read_with_timeout_bytes<READ: AsyncReadExt + Unpin + Send + 'static>(
        &self,
        reader: &mut READ,
        _writer: &mut bytes::BytesMut,
        timeout: Duration,
        desired_size: Option<usize>,
    ) {
        // TODO(https://github.com/rapidrecast/proto-tower/issues/1): Use async read buffer
        let mut data = match desired_size {
            None => bytes::BytesMut::new(),
            Some(sz) => bytes::BytesMut::with_capacity(sz),
        };
        let mut buffer = [0u8; S];
        let finished = AtomicBool::new(false);
        let tries = AtomicU32::new(0);
        const MAX_TRIES: u32 = 10;
        let calculated_timeout = timeout / MAX_TRIES;
        while !finished.load(Ordering::SeqCst) && tries.load(Ordering::SeqCst) < MAX_TRIES {
            let buf_size = match desired_size {
                None => S,
                Some(sz) => min(S, sz - data.len()),
            };
            tokio::select! {
                _ = tokio::time::sleep(calculated_timeout) => {
                    let new_tries = tries.fetch_add(1, Ordering::SeqCst) + 1;
                    if new_tries >= MAX_TRIES {
                        finished.store(true, Ordering::SeqCst);
                    }
                }
                n = reader.read(&mut buffer[..buf_size]) => {
                    match n {
                        Ok(n) => {
                            if n != 0 {
                                data.extend_from_slice(&buffer[..n]);
                                tries.store(0, Ordering::SeqCst);
                            } else {
                                match self.zero_read_behaviour {
                                    ZeroReadBehaviour::TickAndYield => {
                                        tries.fetch_add(1, Ordering::SeqCst);
                                        tokio::task::yield_now().await;
                                    }
                                    ZeroReadBehaviour::TickSleep => {
                                        tries.fetch_add(1, Ordering::SeqCst);
                                        tokio::time::sleep(calculated_timeout).await;
                                    }
                                }

                            }
                        }
                        Err(_) => {
                            finished.store(true, Ordering::SeqCst);
                        }
                    }
                }
            }
        }
    }

    pub async fn read_with_timeout<READ: AsyncReadExt + Unpin + Send + 'static>(&self, reader: &mut READ, timeout: Duration, desired_size: Option<usize>) -> Vec<u8> {
        let mut buf = bytes::BytesMut::new();
        self.read_with_timeout_bytes(reader, &mut buf, timeout, desired_size).await;
        buf.to_vec()
    }
}

#[cfg(test)]
mod test {
    use crate::{AsyncReadToBuf, ZeroReadBehaviour};
    use std::io::Cursor;
    use std::time::Duration;

    #[tokio::test]
    async fn test_limited_read() {
        let data = b"hello world";
        let mut cursor = Cursor::new(data);
        let async_read = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
        let result = async_read.read_with_timeout(&mut cursor, Duration::from_secs(1), Some(5)).await;
        assert_eq!(result, b"hello");
    }

    #[tokio::test]
    async fn test_large_read() {
        let data = [0xff; 4096];
        let mut cursor = Cursor::new(data);
        let async_read = AsyncReadToBuf::new_1024(ZeroReadBehaviour::TickAndYield);
        let result = async_read.read_with_timeout(&mut cursor, Duration::from_secs(1), Some(2050)).await;
        assert_eq!(result, &[0xff; 2050]);
    }
}
