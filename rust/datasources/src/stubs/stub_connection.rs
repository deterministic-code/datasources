use super::not_implemented::fail;
use crate::connection::Connection;
use crate::middleware_aware::MiddlewareAware;
use crate::middleware_hook::MiddlewareHook;
use crate::middleware_host::MiddlewareHost;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::Arc;

pub struct StubConnection {
    provider: String,
    host: MiddlewareHost,
}

impl StubConnection {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            host: MiddlewareHost::new(),
        }
    }
}

impl MiddlewareAware for StubConnection {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Connection for StubConnection {
    async fn open(&self) -> Result<()> {
        self.host
            .run("open", None, || async { Err(fail(&self.provider)) })
            .await
    }

    async fn close(&self) -> Result<()> {
        self.host
            .run("close", None, || async { Err(fail(&self.provider)) })
            .await
    }

    async fn start_transaction(&self) -> Result<Arc<dyn Transaction>> {
        self.host
            .run("startTransaction", None, || async { Err(fail(&self.provider)) })
            .await
    }
}
