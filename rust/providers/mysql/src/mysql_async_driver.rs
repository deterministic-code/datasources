use crate::command_driver::MysqlCommandDriver;
use crate::connection_driver::MysqlConnectionDriver;
use crate::driver::{MysqlDriver, MysqlExecuteResult};
use crate::transaction_driver::MysqlTransactionDriver;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Value};
use std::collections::HashMap;

pub struct MysqlAsyncDriver {
    connection: MysqlConnectionDriver,
}

impl MysqlAsyncDriver {
    pub fn new() -> Self {
        Self {
            connection: MysqlConnectionDriver::new(),
        }
    }
}

impl Default for MysqlAsyncDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MysqlDriver for MysqlAsyncDriver {
    async fn open(&self, url: &str) -> Result<()> {
        self.connection.open(url).await
    }

    async fn close(&self) -> Result<()> {
        self.connection.close().await
    }

    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<MysqlExecuteResult> {
        MysqlCommandDriver::new(&self.connection)
            .execute(sql, params)
            .await
    }

    async fn query(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<Vec<datasources::Row>> {
        MysqlCommandDriver::new(&self.connection)
            .query(sql, params)
            .await
    }

    async fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        MysqlTransactionDriver::new(&self.connection)
            .begin(isolation)
            .await
    }

    async fn commit(&self) -> Result<()> {
        MysqlTransactionDriver::new(&self.connection).commit().await
    }

    async fn rollback(&self) -> Result<()> {
        MysqlTransactionDriver::new(&self.connection)
            .rollback()
            .await
    }
}
