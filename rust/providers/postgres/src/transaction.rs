use crate::driver::PostgresDriver;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Transaction};
use std::sync::Arc;

pub struct PostgresTransaction {
    pub(crate) driver: Arc<dyn PostgresDriver>,
    pub(crate) isolation: IsolationLevel,
}

impl PostgresTransaction {
    pub fn new(driver: Arc<dyn PostgresDriver>, isolation: IsolationLevel) -> Self {
        Self { driver, isolation }
    }
}

#[async_trait]
impl Transaction for PostgresTransaction {
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
