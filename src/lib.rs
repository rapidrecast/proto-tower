//! This should not be included directly by users of this library
//! It includes shared helper code

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Duration;
use tokio::io::AsyncReadExt;

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

    pub async fn read_with_timeout<READ: AsyncReadExt + Unpin + Send + 'static>(&self, reader: &mut READ, timeout: Duration) -> Vec<u8> {
        let mut data = Vec::new();
        let mut buffer = [0u8; S];
        let finished = AtomicBool::new(false);
        let tries = AtomicU32::new(0);
        const MAX_TRIES: u32 = 10;
        let calculated_timeout = timeout / MAX_TRIES;
        while !finished.load(Ordering::SeqCst) && tries.load(Ordering::SeqCst) < MAX_TRIES {
            tokio::select! {
                _ = tokio::time::sleep(calculated_timeout) => {
                    let new_tries = tries.fetch_add(1, Ordering::SeqCst) + 1;
                    if new_tries >= MAX_TRIES {
                        finished.store(true, Ordering::SeqCst);
                    }
                }
                n = reader.read(&mut buffer) => {
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
        data
    }
}
