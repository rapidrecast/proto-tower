use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::AsyncWriteExt;

#[async_trait]
pub trait WriteTo<Writer, E>
where
    Writer: AsyncWriteExt + Send + Unpin + 'static,
    E: Debug,
{
    async fn write_to(&self, writer: &mut Writer) -> Result<(), E>;
}
