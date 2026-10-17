use super::not_implemented::fail;
use crate::connection::Connection;
use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::Arc;

pub struct StubTransaction {
    provider: String,
    connection: Arc<dyn Connection>,
}

impl StubTransaction {
    pub fn new(provider: impl Into<String>, connection: Arc<dyn Connection>) -> Self {
        Self {
            provider: provider.into(),
            connection,
        }
    }
}

#[async_trait]
impl Transaction for StubTransaction {
    fn connection(&self) -> Arc<dyn Connection> {
        Arc::clone(&self.connection)
    }

    fn isolation(&self) -> IsolationLevel {
        IsolationLevel::ReadCommitted
    }

    async fn commit(&self) -> Result<()> {
        Err(fail(&self.provider))
    }

    async fn abort(&self) -> Result<()> {
        Err(fail(&self.provider))
    }
}
