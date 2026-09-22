use crate::driver::MysqlDriver;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Transaction};
use std::sync::Arc;

pub struct MysqlTransaction {
    pub(crate) driver: Arc<dyn MysqlDriver>,
    pub(crate) isolation: IsolationLevel,
}

impl MysqlTransaction {
    pub fn new(driver: Arc<dyn MysqlDriver>, isolation: IsolationLevel) -> Self {
        Self { driver, isolation }
    }
}

#[async_trait]
impl Transaction for MysqlTransaction {
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
