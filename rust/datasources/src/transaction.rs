use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Transaction: Send + Sync {
    fn isolation(&self) -> IsolationLevel;
    async fn commit(&self) -> Result<()>;
    async fn abort(&self) -> Result<()>;
}
