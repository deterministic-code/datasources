use super::not_implemented::fail;
use crate::middleware_aware::MiddlewareAware;
use crate::middleware_hook::MiddlewareHook;
use crate::middleware_host::MiddlewareHost;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;

pub struct StubTransaction {
    provider: String,
    host: MiddlewareHost,
}

impl StubTransaction {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            host: MiddlewareHost::new(),
        }
    }
}

impl MiddlewareAware for StubTransaction {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Transaction for StubTransaction {
    async fn commit(&self) -> Result<()> {
        self.host
            .run("commit", None, || async { Err(fail(&self.provider)) })
            .await
    }

    async fn abort(&self) -> Result<()> {
        self.host
            .run("abort", None, || async { Err(fail(&self.provider)) })
            .await
    }
}
