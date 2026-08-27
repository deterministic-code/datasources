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

#[macro_export]
macro_rules! named_stub_connection {
    ($name:ident, $provider:literal) => {
        pub struct $name {
            inner: $crate::StubConnection,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    inner: $crate::StubConnection::new($provider),
                }
            }
        }

        impl $crate::MiddlewareAware for $name {
            fn use_middleware(
                &self,
                before: Option<$crate::MiddlewareHook>,
                after: Option<$crate::MiddlewareHook>,
            ) {
                self.inner.use_middleware(before, after);
            }
        }

        #[async_trait::async_trait]
        impl $crate::Connection for $name {
            async fn open(&self) -> $crate::Result<()> {
                self.inner.open().await
            }
            async fn close(&self) -> $crate::Result<()> {
                self.inner.close().await
            }
            async fn start_transaction(
                &self,
            ) -> $crate::Result<std::sync::Arc<dyn $crate::Transaction>> {
                self.inner.start_transaction().await
            }
        }
    };
}
