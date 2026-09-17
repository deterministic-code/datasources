use crate::connection_driver::{map_err, PostgresConnectionDriver};
use crate::driver::{to_postgres, with_returning, PostgresExecuteResult};
use crate::pg_value::PgValue;
use chrono::{DateTime, NaiveDateTime, Utc};
use datasources::{DatasourceError, Result, Value};
use rust_decimal::Decimal;
use std::collections::HashMap;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct PostgresCommandDriver<'a> {
    connection: &'a PostgresConnectionDriver,
}

impl<'a> PostgresCommandDriver<'a> {
    pub fn new(connection: &'a PostgresConnectionDriver) -> Self {
        Self { connection }
    }

    pub async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<PostgresExecuteResult> {
        if sql.trim().is_empty() {
            return Ok(PostgresExecuteResult {
                rows_affected: 0,
                result: Value::Null,
            });
        }
        let rewritten = with_returning(sql);
        let (text, values) = to_postgres(&rewritten, params);
        let boxed: Vec<Box<dyn ToSql + Sync + Send>> = values.iter().map(box_value).collect();
        let refs: Vec<&(dyn ToSql + Sync)> = boxed.iter().map(|v| v.as_ref() as _).collect();
        let slot = self.connection.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Postgres driver is not open"))?;
        if rewritten.to_ascii_lowercase().contains("returning") {
            let rows = client.query(&text, &refs).await.map_err(map_err)?;
            let result = rows
                .first()
                .map(|row| from_cell(row, 0))
                .unwrap_or(Value::Null);
            return Ok(PostgresExecuteResult {
                rows_affected: rows.len() as i64,
                result,
            });
        }
        let rows = client.execute(&text, &refs).await.map_err(map_err)?;
        Ok(PostgresExecuteResult {
            rows_affected: rows as i64,
            result: Value::Null,
        })
    }

    pub async fn query(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<Vec<datasources::Row>> {
        if sql.trim().is_empty() {
            return Ok(Vec::new());
        }
        let (text, values) = to_postgres(sql, params);
        let boxed: Vec<Box<dyn ToSql + Sync + Send>> = values.iter().map(box_value).collect();
        let refs: Vec<&(dyn ToSql + Sync)> = boxed.iter().map(|v| v.as_ref() as _).collect();
        let slot = self.connection.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Postgres driver is not open"))?;
        let rows = client.query(&text, &refs).await.map_err(map_err)?;
        Ok(rows.iter().map(row_record).collect())
    }

    pub async fn exec(&self, sql: &str) -> Result<()> {
        let slot = self.connection.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Postgres driver is not open"))?;
        client.batch_execute(sql).await.map_err(map_err)
    }
}

fn box_value(value: &Value) -> Box<dyn ToSql + Sync + Send> {
    Box::new(PgValue(value.clone()))
}

fn from_cell(row: &Row, index: usize) -> Value {
    if let Ok(value) = row.try_get::<_, i64>(index) {
        return Value::Int(value);
    }
    if let Ok(value) = row.try_get::<_, i32>(index) {
        return Value::Int(value.into());
    }
    if let Ok(value) = row.try_get::<_, bool>(index) {
        return Value::Bool(value);
    }
    if let Ok(value) = row.try_get::<_, f64>(index) {
        return Value::Float(value);
    }
    if let Ok(value) = row.try_get::<_, Uuid>(index) {
        return Value::Uuid(value);
    }
    if let Ok(value) = row.try_get::<_, Option<Uuid>>(index) {
        return value.map(Value::Uuid).unwrap_or(Value::Null);
    }
    if let Ok(value) = row.try_get::<_, DateTime<Utc>>(index) {
        return Value::DateTime(value);
    }
    if let Ok(value) = row.try_get::<_, Option<DateTime<Utc>>>(index) {
        return value.map(Value::DateTime).unwrap_or(Value::Null);
    }
    if let Ok(value) = row.try_get::<_, NaiveDateTime>(index) {
        return Value::DateTime(value.and_utc());
    }
    if let Ok(value) = row.try_get::<_, Option<NaiveDateTime>>(index) {
        return value
            .map(|stamp| Value::DateTime(stamp.and_utc()))
            .unwrap_or(Value::Null);
    }
    if let Ok(value) = row.try_get::<_, Decimal>(index) {
        return Value::Decimal(value);
    }
    if let Ok(value) = row.try_get::<_, Option<Decimal>>(index) {
        return value.map(Value::Decimal).unwrap_or(Value::Null);
    }
    if let Ok(value) = row.try_get::<_, String>(index) {
        return Value::Text(value);
    }
    if let Ok(value) = row.try_get::<_, Option<String>>(index) {
        return value.map(Value::Text).unwrap_or(Value::Null);
    }
    if let Ok(value) = row.try_get::<_, Vec<u8>>(index) {
        return Value::Binary(value);
    }
    Value::Null
}

fn row_record(row: &Row) -> datasources::Row {
    let mut record = datasources::Row::new();
    for (index, column) in row.columns().iter().enumerate() {
        record.insert(column.name().to_string(), from_cell(row, index));
    }
    record
}
