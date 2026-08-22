use crate::middleware_aware::MiddlewareAware;
use crate::result::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Transaction: MiddlewareAware + Send + Sync {
    async fn commit(&self) -> Result<()>;
    async fn abort(&self) -> Result<()>;
}
