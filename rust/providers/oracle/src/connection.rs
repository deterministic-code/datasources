use async_trait::async_trait;
use datasources::{Command, Connection, IsolationLevel, Result, StubConnection, Transaction};
use std::sync::Arc;

pub struct OracleConnection {
    inner: StubConnection,
}

impl OracleConnection {
    pub fn new() -> Self {
        Self {
            inner: StubConnection::new("oracle"),
        }
    }

    pub fn from_options(_: Option<&std::collections::HashMap<String, String>>) -> Arc<dyn Connection> {
        Arc::new(Self::new())
    }
}

#[async_trait]
impl Connection for OracleConnection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn provider(&self) -> &str {
        self.inner.provider()
    }
    fn is_open(&self) -> bool {
        self.inner.is_open()
    }
    fn create_command(&self) -> Arc<dyn Command> {
        self.inner.create_command()
    }
    async fn open(&self) -> Result<()> {
        self.inner.open().await
    }
    async fn close(&self) -> Result<()> {
        self.inner.close().await
    }
    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        self.inner.start_transaction(isolation).await
    }
}
