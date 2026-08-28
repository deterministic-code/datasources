use super::not_implemented::fail;
use crate::connection::Connection;
use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::Arc;

pub struct StubConnection {
    provider: String,
}

impl StubConnection {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
        }
    }
}

#[async_trait]
impl Connection for StubConnection {
    async fn open(&self) -> Result<()> {
        Err(fail(&self.provider))
    }

    async fn close(&self) -> Result<()> {
        Err(fail(&self.provider))
    }

    async fn start_transaction(
        &self,
        _isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        Err(fail(&self.provider))
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
                isolation: Option<$crate::IsolationLevel>,
            ) -> $crate::Result<std::sync::Arc<dyn $crate::Transaction>> {
                self.inner.start_transaction(isolation).await
            }
        }
    };
}
