use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tower::Service;

/// A service you can conveniently use to validate layers that interact with AsyncReadExt and AsyncWriteExt
#[derive(Debug, Clone)]
pub struct TestIoService {
    pub read: Arc<Mutex<Vec<u8>>>,
    pub write: Arc<Vec<u8>>,
}

impl TestIoService {
    pub fn new(write: Vec<u8>) -> Self {
        TestIoService {
            read: Arc::new(Mutex::new(Vec::new())),
            write: Arc::new(write),
        }
    }
}

impl<Reader, Writer> Service<(Reader, Writer)> for TestIoService
where
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    type Response = ();
    type Error = ();
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, (mut read, mut write): (Reader, Writer)) -> Self::Future {
        let read_data = self.read.clone();
        let write_data = self.write.clone();
        Box::pin(async move {
            let mut buffer = vec![0; 1024];
            let sent_response = AtomicBool::new(false);
            while let Ok(sz) = read.read(&mut buffer).await {
                eprintln!("Read {} bytes", sz);
                if sz == 0 {
                    break;
                }
                let mut read_lock = read_data.lock().await;
                read_lock.extend_from_slice(&buffer[..sz]);
                if !sent_response.load(Ordering::SeqCst) {
                    let l = write_data.len();
                    eprintln!("Writing {} bytes", l);
                    write.write_all(&write_data).await.unwrap();
                    sent_response.store(true, Ordering::SeqCst);
                }
            }
            Ok(())
        })
    }
}
