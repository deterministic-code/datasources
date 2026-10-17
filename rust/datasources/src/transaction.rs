use crate::connection::Connection;
use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Transaction: Send + Sync {
    fn connection(&self) -> Arc<dyn Connection>;
    fn isolation(&self) -> IsolationLevel;
    async fn commit(&self) -> Result<()>;
    async fn abort(&self) -> Result<()>;
}
