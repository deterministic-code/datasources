use crate::connection_driver::{map_err, SqliteConnectionDriver};
use crate::driver::SqliteExecuteResult;
use datasources::{DatasourceError, Result, Row, Value};
use rusqlite::types::{ToSql, ValueRef};
use std::collections::HashMap;

pub struct SqliteCommandDriver<'a> {
    connection: &'a SqliteConnectionDriver,
}

impl<'a> SqliteCommandDriver<'a> {
    pub fn new(connection: &'a SqliteConnectionDriver) -> Self {
        Self { connection }
    }

    pub fn execute(
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
        let db = self.connection.db.lock().unwrap();
        let conn = db
            .as_ref()
            .ok_or_else(|| DatasourceError::message("SQLite driver is not open"))?;
        let pairs = bind_pairs(params);
        let bound = as_params(&pairs);
        let mut statement = conn.prepare(sql).map_err(map_err)?;
        let rows_affected = if statement.column_count() > 0 {
            let mut rows = statement.query(bound.as_slice()).map_err(map_err)?;
            while rows.next().map_err(map_err)?.is_some() {}
            conn.changes() as i64
        } else {
            statement.execute(bound.as_slice()).map_err(map_err)? as i64
        };
        Ok(SqliteExecuteResult {
            rows_affected,
            result: Value::Int(conn.last_insert_rowid()),
        })
    }

    pub fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Row>> {
        if sql.trim().is_empty() {
            return Ok(Vec::new());
        }
        let db = self.connection.db.lock().unwrap();
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
