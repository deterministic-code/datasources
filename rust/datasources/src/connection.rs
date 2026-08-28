use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Connection: Send + Sync {
    async fn open(&self) -> Result<()>;
    async fn close(&self) -> Result<()>;
    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>>;
}
