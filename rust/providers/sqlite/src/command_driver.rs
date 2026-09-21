use crate::connection_driver::{map_err, SqliteConnectionDriver};
use crate::converters::{
    BinaryConverter, BoolConverter, DateTimeConverter, DecimalConverter, UuidConverter,
};
use crate::driver::SqliteExecuteResult;
use datasources::{DatasourceError, Result, Row, TypeConverter, Value};
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
        if sql.trim_start().len() >= 6
            && sql.trim_start()[..6].eq_ignore_ascii_case("SELECT")
            && statement.column_count() > 0
        {
            let mut rows = statement.query(bound.as_slice()).map_err(map_err)?;
            return Ok(match rows.next().map_err(map_err)? {
                Some(row) => SqliteExecuteResult {
                    rows_affected: 1,
                    result: from_sql(row.get_ref(0).map_err(map_err)?),
                },
                None => SqliteExecuteResult {
                    rows_affected: 0,
                    result: Value::Null,
                },
            });
        }
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

fn reverse(value: &Value) -> Value {
    match value {
        Value::DateTime(_) => DateTimeConverter.reverse_convert(value),
        Value::Bool(_) => BoolConverter.reverse_convert(value),
        Value::Uuid(_) => UuidConverter.reverse_convert(value),
        Value::Decimal(_) => DecimalConverter.reverse_convert(value),
        Value::Binary(_) => BinaryConverter.reverse_convert(value),
        _ => return value.clone(),
    }
    .unwrap_or_else(|_| value.clone())
}

fn to_sql(value: &Value) -> rusqlite::types::Value {
    match reverse(value) {
        Value::Null => rusqlite::types::Value::Null,
        Value::Int(value) => rusqlite::types::Value::Integer(value),
        Value::Float(value) => rusqlite::types::Value::Real(value),
        Value::Text(value) => rusqlite::types::Value::Text(value),
        Value::Binary(value) => rusqlite::types::Value::Blob(value),
        other => rusqlite::types::Value::Text(other.wire_text()),
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
