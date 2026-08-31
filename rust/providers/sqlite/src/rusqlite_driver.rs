use crate::driver::{SqliteDriver, SqliteExecuteResult};
use async_trait::async_trait;
use datasources::{DatasourceError, IsolationLevel, Row, Result, Value};
use rusqlite::types::{ToSql, ValueRef};
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct RusqliteDriver {
    db: Mutex<Option<Connection>>,
}

impl RusqliteDriver {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(None),
        }
    }
}

impl Default for RusqliteDriver {
    fn default() -> Self {
        Self::new()
    }
}

fn map_err(error: rusqlite::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

fn named_key(name: &str) -> String {
    match name.chars().next() {
        Some('$' | '@' | ':') => name.to_string(),
        _ => format!("${name}"),
    }
}

fn to_sql(value: &Value) -> rusqlite::types::Value {
    match value {
        Value::Null => rusqlite::types::Value::Null,
        Value::Bool(flag) => rusqlite::types::Value::Integer(if *flag { 1 } else { 0 }),
        Value::Int(value) => rusqlite::types::Value::Integer(*value),
        Value::Float(value) => rusqlite::types::Value::Real(*value),
        Value::Text(value) => rusqlite::types::Value::Text(value.clone()),
        Value::DateTime(value) => rusqlite::types::Value::Text(Value::iso_datetime(*value)),
        Value::Uuid(value) => rusqlite::types::Value::Text(value.to_string()),
        Value::Decimal(value) => rusqlite::types::Value::Text(value.to_string()),
        Value::Binary(value) => rusqlite::types::Value::Blob(value.clone()),
    }
}

fn from_sql(value: ValueRef<'_>) -> Value {
    match value {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(value) => Value::Int(value),
        ValueRef::Real(value) => Value::Float(value),
        ValueRef::Text(bytes) => Value::Text(String::from_utf8_lossy(bytes).into_owned()),
        ValueRef::Blob(bytes) => Value::Binary(bytes.to_vec()),
    }
}

fn bind_pairs(params: &HashMap<String, Value>) -> Vec<(String, rusqlite::types::Value)> {
    params
        .iter()
        .map(|(name, value)| (named_key(name), to_sql(value)))
        .collect()
}

fn as_params(pairs: &[(String, rusqlite::types::Value)]) -> Vec<(&str, &dyn ToSql)> {
    pairs
        .iter()
        .map(|(name, value)| (name.as_str(), value as &dyn ToSql))
        .collect()
}

#[async_trait]
impl SqliteDriver for RusqliteDriver {
    async fn open(&self, filename: &str) -> Result<()> {
        let mut db = self.db.lock().unwrap();
        if db.is_some() {
            return Ok(());
        }
        let conn = if filename.starts_with("file:") {
            Connection::open_with_flags(
                filename,
                OpenFlags::SQLITE_OPEN_READ_WRITE
                    | OpenFlags::SQLITE_OPEN_CREATE
                    | OpenFlags::SQLITE_OPEN_URI
                    | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
            )
        } else {
            Connection::open(filename)
        }
        .map_err(map_err)?;
        conn.busy_timeout(std::time::Duration::ZERO)
            .map_err(map_err)?;
        *db = Some(conn);
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.db.lock().unwrap() = None;
        Ok(())
    }

    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<SqliteExecuteResult> {
        if sql.trim().is_empty() {
            return Ok(SqliteExecuteResult {
                rows_affected: 0,
                result: Value::Null,
            });
        }
        let db = self.db.lock().unwrap();
        let conn = db
            .as_ref()
            .ok_or_else(|| DatasourceError::message("SQLite driver is not open"))?;
        let pairs = bind_pairs(params);
        let bound = as_params(&pairs);
        let rows_affected = conn.execute(sql, bound.as_slice()).map_err(map_err)? as i64;
        Ok(SqliteExecuteResult {
            rows_affected,
            result: Value::Int(conn.last_insert_rowid()),
        })
    }

    async fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Row>> {
        if sql.trim().is_empty() {
            return Ok(Vec::new());
        }
        let db = self.db.lock().unwrap();
        let conn = db
            .as_ref()
            .ok_or_else(|| DatasourceError::message("SQLite driver is not open"))?;
        let pairs = bind_pairs(params);
        let bound = as_params(&pairs);
        let mut statement = conn.prepare(sql).map_err(map_err)?;
        let column_names: Vec<String> = statement
            .column_names()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        let mut rows = statement.query(bound.as_slice()).map_err(map_err)?;
        let mut records = Vec::new();
        while let Some(row) = rows.next().map_err(map_err)? {
            let mut record = Row::new();
            for (index, name) in column_names.iter().enumerate() {
                record.insert(name.clone(), from_sql(row.get_ref(index).map_err(map_err)?));
            }
            records.push(record);
        }
        Ok(records)
    }

    async fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        let db = self.db.lock().unwrap();
        let conn = db
            .as_ref()
            .ok_or_else(|| DatasourceError::message("SQLite driver is not open"))?;
        let flag = matches!(isolation, IsolationLevel::ReadUncommitted) as i64;
        conn.pragma_update(None, "read_uncommitted", flag)
            .map_err(map_err)?;
        conn.execute("BEGIN", []).map_err(map_err).map(|_| ())
    }

    async fn commit(&self) -> Result<()> {
        self.execute("COMMIT", &HashMap::new()).await.map(|_| ())
    }

    async fn rollback(&self) -> Result<()> {
        self.execute("ROLLBACK", &HashMap::new()).await.map(|_| ())
    }
}
