use super::not_implemented::fail;
use super::stub_command::StubCommand;
use crate::command::Command;
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn provider(&self) -> &str {
        &self.provider
    }

    fn is_open(&self) -> bool {
        false
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(StubCommand::new(&self.provider))
    }

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
    ($name:ident, $driver:ident, $provider:literal) => {
        pub struct $name {
            inner: $crate::StubConnection,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    inner: $crate::StubConnection::new($provider),
                }
            }

            pub fn from_options(
                _: Option<&std::collections::HashMap<String, String>>,
            ) -> std::sync::Arc<dyn $crate::Connection> {
                std::sync::Arc::new(Self::new())
            }
        }

        pub struct $driver;

        impl $crate::Driver for $driver {
            fn connection(
                &self,
                options: Option<&std::collections::HashMap<String, String>>,
            ) -> std::sync::Arc<dyn $crate::Connection> {
                $name::from_options(options)
            }

            fn command(
                &self,
                connection: std::sync::Arc<dyn $crate::Connection>,
            ) -> std::sync::Arc<dyn $crate::Command> {
                connection.create_command()
            }

            fn transaction(
                &self,
                _: std::sync::Arc<dyn $crate::Connection>,
                _: $crate::IsolationLevel,
            ) -> std::sync::Arc<dyn $crate::Transaction> {
                std::sync::Arc::new($crate::StubTransaction::new($provider))
            }

            fn types(
                &self,
            ) -> std::collections::HashMap<
                String,
                Box<dyn $crate::TypeConverter<$crate::Value, $crate::Value> + Send + Sync>,
            > {
                std::collections::HashMap::new()
            }
        }

        #[async_trait::async_trait]
        impl $crate::Connection for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn provider(&self) -> &str {
                self.inner.provider()
            }
            fn is_open(&self) -> bool {
                self.inner.is_open()
            }
            fn create_command(&self) -> std::sync::Arc<dyn $crate::Command> {
                self.inner.create_command()
            }
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
