use async_trait::async_trait;
use datasources::{DatasourceError, IsolationLevel, Record, Result, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct PostgresExecuteResult {
    pub rows_affected: i64,
    pub result: Value,
}

#[async_trait]
pub trait PostgresDriver: Send + Sync {
    async fn open(&self, url: &str) -> Result<()>;
    async fn close(&self) -> Result<()>;
    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<PostgresExecuteResult>;
    async fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Record>>;
    async fn begin(&self, isolation: IsolationLevel) -> Result<()>;
    async fn commit(&self) -> Result<()>;
    async fn rollback(&self) -> Result<()>;
}

pub fn require_driver(driver: Option<Arc<dyn PostgresDriver>>) -> Result<Arc<dyn PostgresDriver>> {
    driver.ok_or_else(|| DatasourceError::message("Postgres driver is required"))
}

pub fn to_postgres(sql: &str, params: &HashMap<String, Value>) -> (String, Vec<Value>) {
    let mut names = Vec::<String>::new();
    let mut out = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' && i + 1 < chars.len() && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            i += 1;
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            let index = match names.iter().position(|n| n == &name) {
                Some(index) => index,
                None => {
                    names.push(name);
                    names.len() - 1
                }
            };
            out.push('$');
            out.push_str(&(index + 1).to_string());
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    let values = names
        .iter()
        .map(|name| params.get(name).cloned().unwrap_or(Value::Null))
        .collect();
    (out, values)
}

pub fn with_returning(sql: &str) -> String {
    let trimmed = sql.trim();
    let head = trimmed.get(..6).unwrap_or("").eq_ignore_ascii_case("insert");
    let returning = trimmed.to_ascii_lowercase().contains("returning");
    if head && !returning {
        format!("{} RETURNING id", trimmed.trim_end_matches(';'))
    } else {
        sql.to_string()
    }
}

pub fn isolation_sql(isolation: IsolationLevel) -> &'static str {
    match isolation {
        IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel::ReadCommitted => "READ COMMITTED",
        IsolationLevel::RepeatableRead => "REPEATABLE READ",
        IsolationLevel::Serializable => "SERIALIZABLE",
    }
}
