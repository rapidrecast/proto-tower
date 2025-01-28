use tokio::io::AsyncWriteExt;

pub mod client;
pub mod data;
pub mod server;

#[derive(Clone, Debug)]
pub struct ProtoHttp2Config {
    pub max_header_size: usize,
    pub max_body_size: usize,
    pub timeout: std::time::Duration,
}
