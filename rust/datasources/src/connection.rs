use crate::command::Command;
use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Connection: Send + Sync {
    fn provider(&self) -> &str;
    fn is_open(&self) -> bool;
    fn create_command(&self) -> Arc<dyn Command>;
    async fn open(&self) -> Result<()>;
    async fn close(&self) -> Result<()>;
    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>>;
}
