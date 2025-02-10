use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicU64;
use std::sync::RwLock as StdRwLock;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub struct TimeoutCounter {
    pub tick_rate: Duration,
    pub total_count: u64,
    pub current_count: AtomicU64,
    pub start_time: StdRwLock<Instant>,
}

pub enum CountOrDuration {
    Count(u64),
    Duration(Duration),
}

impl TimeoutCounter {
    /// TODO this can actually be a fixed timeout with default of 1 increments (assuming the future gets cancelled)
    /// Then the second part would always be duration and the tick increment would be num or duration
    pub fn new(interval: CountOrDuration, timeout: CountOrDuration) -> Self {
        match (interval, timeout) {
            (CountOrDuration::Count(count), CountOrDuration::Duration(timeout)) => {
                let tick_rate = Duration::from_millis((timeout.as_millis() / count as u128) as u64);
                TimeoutCounter {
                    tick_rate,
                    total_count: count,
                    current_count: AtomicU64::new(0),
                    start_time: StdRwLock::new(Instant::now()),
                }
            }
            (CountOrDuration::Duration(tick_rate), CountOrDuration::Count(count)) => TimeoutCounter {
                tick_rate,
                total_count: count,
                current_count: AtomicU64::new(0),
                start_time: StdRwLock::new(Instant::now()),
            },
            (_, _) => panic!("Invalid combination of interval and timeout"),
        }
    }
    pub fn next_timeout(&self) -> NextTimeout {
        // adjust count and timeout for current time
        let start_time = self.start_time.read().unwrap();
        let now = Instant::now();
        let elapsed_time = now.checked_duration_since(*start_time).expect("Time went backwards");
        // find tick
        let next_count = (elapsed_time.as_millis() / self.tick_rate.as_millis() + 1) as u64;
        self.current_count.store(next_count, std::sync::atomic::Ordering::Relaxed);
        NextTimeout::new(now + self.tick_rate, next_count > self.total_count)
    }

    pub fn reset(&self) {
        {
            let mut lock = self.start_time.write().unwrap();
            *lock = Instant::now();
        }
        self.current_count.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct NextTimeout {
    when: Instant,
    fail: bool,
}

impl NextTimeout {
    pub fn new(when: Instant, fail: bool) -> Self {
        NextTimeout { when, fail }
    }
}

impl Future for NextTimeout {
    type Output = Result<(), ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.fail {
            return Poll::Ready(Err(()));
        }
        if Instant::now() >= self.when {
            Poll::Ready(Ok(()))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_counter() {
        let start = Instant::now();
        let timeout_counter = TimeoutCounter::new(CountOrDuration::Duration(Duration::from_millis(100)), CountOrDuration::Count(3));
        timeout_counter.next_timeout().await.unwrap();
        timeout_counter.next_timeout().await.unwrap();
        timeout_counter.reset();
        timeout_counter.next_timeout().await.unwrap();
        timeout_counter.next_timeout().await.unwrap();
        timeout_counter.next_timeout().await.unwrap();
        assert!(timeout_counter.next_timeout().await.is_err());
        let duration = start.elapsed();
        assert!(duration.as_millis() >= 500, "{:?}", duration);
    }
}
