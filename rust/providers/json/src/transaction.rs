use crate::connection::JsonState;
use async_trait::async_trait;
use datasources::{
    IsolationLevel, MiddlewareAware, MiddlewareHook, MiddlewareHost, Result, Transaction,
};
use std::sync::Arc;

pub struct JsonTransaction {
    pub(crate) host: MiddlewareHost,
    pub(crate) state: Arc<JsonState>,
    pub(crate) isolation: IsolationLevel,
}

impl MiddlewareAware for JsonTransaction {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Transaction for JsonTransaction {
    fn isolation(&self) -> IsolationLevel {
        self.isolation
    }

    async fn commit(&self) -> Result<()> {
        self.host
            .run("commit", None, || async {
                *self.state.in_transaction.lock().unwrap() = false;
                *self.state.snapshot.lock().unwrap() = None;
                if !self.state.path.is_empty() {
                    self.state.store.write(
                        &self.state.path,
                        &self.state.data.lock().unwrap().to_string(),
                    )?;
                }
                Ok(())
            })
            .await
    }

    async fn abort(&self) -> Result<()> {
        self.host
            .run("abort", None, || async {
                if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
                    *self.state.data.lock().unwrap() = snapshot;
                }
                *self.state.in_transaction.lock().unwrap() = false;
                Ok(())
            })
            .await
    }
}
