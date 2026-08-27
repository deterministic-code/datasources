use async_trait::async_trait;
use datasources::{DatasourceError, Record, Result, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
    async fn begin(&self) -> Result<()>;
    async fn commit(&self) -> Result<()>;
    async fn rollback(&self) -> Result<()>;
}

pub fn require_driver(driver: Option<Arc<dyn SqliteDriver>>) -> Result<Arc<dyn SqliteDriver>> {
    driver.ok_or_else(|| DatasourceError::message("SQLite driver is required"))
}

pub struct ScriptedDriver {
    pub opened: Mutex<bool>,
    pub closed: Mutex<bool>,
    pub begins: Mutex<usize>,
    pub commits: Mutex<usize>,
    pub rollbacks: Mutex<usize>,
    pub executes: Mutex<Vec<String>>,
    pub queries: Mutex<Vec<String>>,
    pub query_result: Mutex<Vec<Record>>,
    pub execute_result: Mutex<SqliteExecuteResult>,
}

impl ScriptedDriver {
    pub fn new() -> Self {
        Self {
            opened: Mutex::new(false),
            closed: Mutex::new(false),
            begins: Mutex::new(0),
            commits: Mutex::new(0),
            rollbacks: Mutex::new(0),
            executes: Mutex::new(Vec::new()),
            queries: Mutex::new(Vec::new()),
            query_result: Mutex::new(Vec::new()),
            execute_result: Mutex::new(SqliteExecuteResult {
                rows_affected: 1,
                result: Value::Int(10),
            }),
        }
    }
}

#[async_trait]
impl SqliteDriver for ScriptedDriver {
    async fn open(&self, _filename: &str) -> Result<()> {
        *self.opened.lock().unwrap() = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.closed.lock().unwrap() = true;
        Ok(())
    }

    async fn execute(
        &self,
        sql: &str,
        _params: &HashMap<String, Value>,
    ) -> Result<SqliteExecuteResult> {
        self.executes.lock().unwrap().push(sql.to_string());
        Ok(self.execute_result.lock().unwrap().clone())
    }

    async fn query(&self, sql: &str, _params: &HashMap<String, Value>) -> Result<Vec<Record>> {
        self.queries.lock().unwrap().push(sql.to_string());
        Ok(self.query_result.lock().unwrap().clone())
    }

    async fn begin(&self) -> Result<()> {
        *self.begins.lock().unwrap() += 1;
        Ok(())
    }

    async fn commit(&self) -> Result<()> {
        *self.commits.lock().unwrap() += 1;
        Ok(())
    }

    async fn rollback(&self) -> Result<()> {
        *self.rollbacks.lock().unwrap() += 1;
        Ok(())
    }
}
