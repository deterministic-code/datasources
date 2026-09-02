use async_trait::async_trait;
use datasources::{DatasourceError, IsolationLevel, Row, Result, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct MysqlExecuteResult {
    pub rows_affected: i64,
    pub result: Value,
}

#[async_trait]
pub trait MysqlDriver: Send + Sync {
    async fn open(&self, url: &str) -> Result<()>;
    async fn close(&self) -> Result<()>;
    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<MysqlExecuteResult>;
    async fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Row>>;
    async fn begin(&self, isolation: IsolationLevel) -> Result<()>;
    async fn commit(&self) -> Result<()>;
    async fn rollback(&self) -> Result<()>;
}

pub fn require_driver(driver: Option<Arc<dyn MysqlDriver>>) -> Result<Arc<dyn MysqlDriver>> {
    driver.ok_or_else(|| DatasourceError::message("MySQL driver is required"))
}

pub fn without_returning(sql: &str) -> String {
    let upper = sql.to_ascii_uppercase();
    match upper.rfind(" RETURNING ") {
        Some(index) => sql[..index].to_string(),
        None => sql.to_string(),
    }
}

pub fn to_mysql(sql: &str) -> String {
    let sql = without_returning(sql);
    let mut out = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(next) = chars.peek() {
                if next.is_ascii_alphabetic() || *next == '_' {
                    out.push(':');
                    continue;
                }
            }
        }
        out.push(ch);
    }
    out
}

pub fn isolation_sql(isolation: IsolationLevel) -> &'static str {
    match isolation {
        IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel::ReadCommitted => "READ COMMITTED",
        IsolationLevel::RepeatableRead | IsolationLevel::Serializable => "REPEATABLE READ",
    }
}
