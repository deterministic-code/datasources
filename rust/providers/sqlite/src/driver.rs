use async_trait::async_trait;
use datasources::{DatasourceError, IsolationLevel, Record, Result, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct SqliteExecuteResult {
    pub rows_affected: i64,
    pub result: Value,
}

#[async_trait]
pub trait SqliteDriver: Send + Sync {
    async fn open(&self, filename: &str) -> Result<()>;
    async fn close(&self) -> Result<()>;
    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<SqliteExecuteResult>;
    async fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Record>>;
    async fn begin(&self, isolation: IsolationLevel) -> Result<()>;
    async fn commit(&self) -> Result<()>;
    async fn rollback(&self) -> Result<()>;
}

pub fn require_driver(driver: Option<Arc<dyn SqliteDriver>>) -> Result<Arc<dyn SqliteDriver>> {
    driver.ok_or_else(|| DatasourceError::message("SQLite driver is required"))
}
