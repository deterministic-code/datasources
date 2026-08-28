use super::not_implemented::fail;
use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;

pub struct StubTransaction {
    provider: String,
}

impl StubTransaction {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
        }
    }
}

#[async_trait]
impl Transaction for StubTransaction {
    fn isolation(&self) -> IsolationLevel {
        IsolationLevel::Serializable
    }

    async fn commit(&self) -> Result<()> {
        Err(fail(&self.provider))
    }

    async fn abort(&self) -> Result<()> {
        Err(fail(&self.provider))
    }
}
