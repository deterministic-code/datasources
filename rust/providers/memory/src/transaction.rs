use crate::connection::MemoryState;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Transaction};
use std::sync::Arc;

pub struct MemoryTransaction {
    pub(crate) state: Arc<MemoryState>,
    pub(crate) isolation: IsolationLevel,
}

#[async_trait]
impl Transaction for MemoryTransaction {
    fn isolation(&self) -> IsolationLevel {
        self.isolation
    }

    async fn commit(&self) -> Result<()> {
        *self.state.in_transaction.lock().unwrap() = false;
        *self.state.snapshot.lock().unwrap() = None;
        Ok(())
    }

    async fn abort(&self) -> Result<()> {
        if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
            self.state.store.restore(snapshot);
        }
        *self.state.in_transaction.lock().unwrap() = false;
        Ok(())
    }
}
