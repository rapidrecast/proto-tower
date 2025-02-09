use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{simplex, AsyncReadExt, AsyncWriteExt, ReadHalf, SimplexStream, WriteHalf};
use tokio::task::JoinHandle;
use tower::{Layer, Service};

const MAX_BUF_SIZE: usize = 1024;
const IDENTIFIER: &str = "[DEBUG_LAYER]";

#[derive(Clone)]
pub struct DebugIoService<InnerService>
where
    // We are explicit with the types, since we know the implementation we are providing downstream
    // However, the downstream service (such as another instance of this layer) can be generic
    InnerService: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = ()> + Clone + Send + 'static,
{
    inner: InnerService,
}

impl<InnerService, Reader, Writer> Service<(Reader, Writer)> for DebugIoService<InnerService>
where
    InnerService: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = ()> + Clone + Send + 'static,
    InnerService::Future: Future<Output = Result<InnerService::Response, InnerService::Error>> + Send + 'static,
    InnerService::Error: Send + 'static,
    // We need Unpin because otherwise we cannot access the methods of these traits
    Reader: AsyncReadExt + Send + Unpin + 'static,
    Writer: AsyncWriteExt + Send + Unpin + 'static,
{
    // Since all communication is done via the readers and writers, there isn't really a need for a return type
    type Response = ();
    type Error = InnerService::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map(|result| result.map_err(|err| err.into()))
    }

    fn call(&mut self, (mut input_reader, mut input_writer): (Reader, Writer)) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            // We create the read/write pairs that we will use to communicate with the downstream service
            let (read_svc, mut write_this) = simplex(MAX_BUF_SIZE);
            let (mut read_this, write_svc) = simplex(MAX_BUF_SIZE);

            // Now we spawn the downstream inner service because otherwise we would need to poll it to make it progress
            // Calling await on it directly would block the current task, preventing us from relaying messages
            // Because we have so many generics, my IDE isn't prompting with types, so I declared them explicitly here.
            let task: JoinHandle<Result<InnerService::Response, InnerService::Error>> = tokio::spawn(inner.call((read_svc, write_svc)));

            // Ideally everything below would be a loop, but we won't bother with that
            // We would need to handle more conditions than we would like for the purpose of the example

            // Read from the layer input
            let mut input_read_buffer = [0u8; 1024];
            let mut output_read_buffer = [0u8; 1024];

            loop {
                tokio::select! {
                    result_sz = input_reader.read(&mut input_read_buffer) => {
                        match result_sz {
                            Ok(0) | Err(_) => {
                                eprintln!("{} Failed to read from input reader", IDENTIFIER);
                                break;
                            }
                            Ok(sz) => {
                                let have_read = &input_read_buffer[..sz];
                                eprintln!("{} Read {} bytes: {}", IDENTIFIER, sz, escape_bytes_hex(have_read));
                                if let Err(e) = write_this.write_all(have_read).await {
                                    eprintln!("{} Failed to write to downstream service: {:?}", IDENTIFIER, e);
                                    break;
                                }
                            }
                        }
                    }
                    result_sz = read_this.read(&mut output_read_buffer) => {
                        match result_sz {
                            Ok(0) | Err(_) => {
                                eprintln!("{} Failed to read from downstream reader", IDENTIFIER);
                                break;
                            }
                            Ok(sz) => {
                                let have_read = &output_read_buffer[..sz];
                                eprintln!("{} Read {} bytes: {}", IDENTIFIER, sz, escape_bytes_hex(have_read));
                                if let Err(e) = input_writer.write_all(have_read).await {
                                    eprintln!("{} Failed to write to upstream service: {:?}", IDENTIFIER, e);
                                    break;
                                }
                            }
                        }
                    }
                }
                if task.is_finished() {
                    break;
                }
            }
            drop(input_reader);
            drop(input_writer);
            drop(read_this);
            drop(write_this);

            // Let's politely wait for the task to complete in case it has errored
            task.await.unwrap()
        })
    }
}

/// I/O Pattern Layer takes a (read, write) (as it would for servers) and will also send down
/// a (read, write) pair (as you would do for clients)
#[derive(Default)]
pub struct DebugIoLayer {}

impl<InnerService> Layer<InnerService> for DebugIoLayer
where
    InnerService: Service<(ReadHalf<SimplexStream>, WriteHalf<SimplexStream>), Response = ()> + Clone + Send + 'static,
    InnerService::Future: Future<Output = Result<InnerService::Response, InnerService::Error>> + Send + 'static,
    InnerService::Error: Send + 'static,
{
    type Service = DebugIoService<InnerService>;

    fn layer(&self, inner: InnerService) -> Self::Service {
        DebugIoService { inner }
    }
}

fn escape_bytes_hex(input: &[u8]) -> String {
    input
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' {
                (b as char).to_string() // Keep printable ASCII
            } else {
                format!("\\x{:02x}", b) // Convert others to hex
            }
        })
        .collect()
}
