use crate::command_driver::SqliteCommandDriver;
use crate::connection_driver::SqliteConnectionDriver;
use crate::driver::{SqliteDriver, SqliteExecuteResult};
use crate::transaction_driver::SqliteTransactionDriver;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Row, Value};
use std::collections::HashMap;

pub struct RusqliteDriver {
    connection: SqliteConnectionDriver,
}

impl RusqliteDriver {
    pub fn new() -> Self {
        Self {
            connection: SqliteConnectionDriver::new(),
        }
    }
}

impl Default for RusqliteDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SqliteDriver for RusqliteDriver {
    async fn open(&self, filename: &str) -> Result<()> {
        self.connection.open(filename)
    }

    async fn close(&self) -> Result<()> {
        self.connection.close()
    }

    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<SqliteExecuteResult> {
        SqliteCommandDriver::new(&self.connection).execute(sql, params)
    }

    async fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Row>> {
        SqliteCommandDriver::new(&self.connection).query(sql, params)
    }

    async fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        SqliteTransactionDriver::new(&self.connection).begin(isolation)
    }

    async fn commit(&self) -> Result<()> {
        SqliteTransactionDriver::new(&self.connection).commit()
    }

    async fn rollback(&self) -> Result<()> {
        SqliteTransactionDriver::new(&self.connection).rollback()
    }
}
