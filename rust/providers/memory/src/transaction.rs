use crate::connection::MemoryState;
use async_trait::async_trait;
use datasources::{
    IsolationLevel, MiddlewareAware, MiddlewareHook, MiddlewareHost, Result, Transaction,
};
use std::sync::Arc;

pub struct MemoryTransaction {
    pub(crate) host: MiddlewareHost,
    pub(crate) state: Arc<MemoryState>,
    pub(crate) isolation: IsolationLevel,
}

impl MiddlewareAware for MemoryTransaction {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Transaction for MemoryTransaction {
    fn isolation(&self) -> IsolationLevel {
        self.isolation
    }

    async fn commit(&self) -> Result<()> {
        self.host
            .run("commit", None, || async {
                *self.state.in_transaction.lock().unwrap() = false;
                *self.state.snapshot.lock().unwrap() = None;
                Ok(())
            })
            .await
    }

    async fn abort(&self) -> Result<()> {
        self.host
            .run("abort", None, || async {
                if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
                    self.state.store.restore(snapshot);
                }
                *self.state.in_transaction.lock().unwrap() = false;
                Ok(())
            })
            .await
    }
}
