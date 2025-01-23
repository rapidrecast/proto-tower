use crate::layer::ProtoHttp1Layer;
use crate::{HTTP1Request, HTTP1Response, ProtoHttp1Config};
use std::collections::BTreeMap;
use std::future::Future;
use std::marker::PhantomData;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::{Layer, Service};

pub type ProtoUpgradeMap<READ: AsyncReadExt + Send + Unpin + 'static, WRITE: AsyncWriteExt + Send + Unpin + 'static> = BTreeMap<&'static str, Box<dyn Service<(READ, WRITE), Response=(), Error=(), Future=Box<dyn Future<Output=Result<(), ()>> + Send>> + Send>>;

/// This is the initializer for the layer.
/// Invocations to the service will take the sender and receiver of the connection and process
/// the full lifetime.
pub struct ProtoHttp1MakeLayer<SERVICE, READ, WRITE>
where
    SERVICE: Service<HTTP1Request, Response=HTTP1Response> + Send + Clone,
    READ: AsyncReadExt + Send + Unpin + 'static,
    WRITE: AsyncWriteExt + Send + Unpin + 'static,
{
    config: ProtoHttp1Config,
    protocol_upgrades: ProtoUpgradeMap<READ, WRITE>,
    phantom_service: PhantomData<SERVICE>,
    phantom_read: PhantomData<READ>,
    phantom_write: PhantomData<WRITE>,
}

impl<SERVICE, READ, WRITE> ProtoHttp1MakeLayer<SERVICE, READ, WRITE>
where
    SERVICE: Service<HTTP1Request, Response=HTTP1Response> + Send + Clone,
    READ: AsyncReadExt + Send + Unpin + 'static,
    WRITE: AsyncWriteExt + Send + Unpin + 'static,
{
    /// Create a new instance of the layer
    pub fn new(config: ProtoHttp1Config, protocol_upgrades: Option<ProtoUpgradeMap<READ, WRITE>>) -> Self {
        ProtoHttp1MakeLayer {
            phantom_service: PhantomData,
            phantom_read: PhantomData,
            config,
            protocol_upgrades: protocol_upgrades.unwrap_or(BTreeMap::new()),
            phantom_write: PhantomData,
        }
    }
}

impl<SERVICE, READ, WRITE> Layer<SERVICE> for ProtoHttp1MakeLayer<SERVICE, READ, WRITE>
where
    SERVICE: Service<HTTP1Request, Response=HTTP1Response> + Send + Clone,
    READ: AsyncReadExt + Send + Unpin + 'static,
    WRITE: AsyncWriteExt + Send + Unpin + 'static,
{
    type Service = ProtoHttp1Layer<SERVICE, READ, WRITE>;

    fn layer(&self, inner: SERVICE) -> Self::Service {
        ProtoHttp1Layer::new(self.config.clone(), inner)
    }
}
