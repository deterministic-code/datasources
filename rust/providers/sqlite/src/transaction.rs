use crate::driver::SqliteDriver;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Transaction};
use std::sync::Arc;

pub struct SqliteTransaction {
    pub(crate) driver: Arc<dyn SqliteDriver>,
    pub(crate) isolation: IsolationLevel,
}

impl SqliteTransaction {
    pub fn new(driver: Arc<dyn SqliteDriver>, isolation: IsolationLevel) -> Self {
        Self { driver, isolation }
    }
}

#[async_trait]
impl Transaction for SqliteTransaction {
    fn isolation(&self) -> IsolationLevel {
        self.isolation
    }

    async fn commit(&self) -> Result<()> {
        self.driver.commit().await
    }

    async fn abort(&self) -> Result<()> {
        self.driver.rollback().await
    }
}
