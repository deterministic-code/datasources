use crate::connection::JsonState;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Transaction};
use std::sync::Arc;

pub struct JsonTransaction {
    pub(crate) state: Arc<JsonState>,
    pub(crate) isolation: IsolationLevel,
}

#[async_trait]
impl Transaction for JsonTransaction {
    fn isolation(&self) -> IsolationLevel {
        self.isolation
    }

    async fn commit(&self) -> Result<()> {
        *self.state.in_transaction.lock().unwrap() = false;
        *self.state.snapshot.lock().unwrap() = None;
        if !self.state.path.is_empty() {
            self.state.store.write(
                &self.state.path,
                &self.state.data.lock().unwrap().to_string(),
            )?;
        }
        Ok(())
    }

    async fn abort(&self) -> Result<()> {
        if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
            *self.state.data.lock().unwrap() = snapshot;
        }
        *self.state.in_transaction.lock().unwrap() = false;
        Ok(())
    }
}
