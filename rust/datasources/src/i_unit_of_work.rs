use crate::result::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IUnitOfWork: Send {
    async fn begin(&mut self) -> Result<()>;
    async fn commit(&mut self) -> Result<()>;
    async fn abort(&mut self) -> Result<()>;
}
