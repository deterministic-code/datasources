use crate::driver::{
    isolation_sql, to_postgres, with_returning, PostgresDriver, PostgresExecuteResult,
};
use async_trait::async_trait;
use datasources::{DatasourceError, IsolationLevel, Record, Result, Value};
use std::collections::HashMap;
use tokio::sync::Mutex;
use tokio_postgres::{NoTls, Row};
use tokio_postgres::types::ToSql;

pub struct TokioPostgresDriver {
    schema: Option<String>,
    client: Mutex<Option<tokio_postgres::Client>>,
}

impl TokioPostgresDriver {
    pub fn new(schema: Option<String>) -> Self {
        Self {
            schema,
            client: Mutex::new(None),
        }
    }
}

fn map_err(error: tokio_postgres::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

fn box_value(value: &Value) -> Box<dyn ToSql + Sync + Send> {
    match value {
        Value::Null => Box::new(Option::<i64>::None),
        Value::Bool(flag) => Box::new(*flag),
        Value::Int(value) => Box::new(*value),
        Value::Float(value) => Box::new(*value),
        Value::Text(value) => Box::new(value.clone()),
    }
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
    if let Ok(value) = row.try_get::<_, String>(index) {
        return Value::Text(value);
    }
    if let Ok(value) = row.try_get::<_, Option<String>>(index) {
        return value.map(Value::Text).unwrap_or(Value::Null);
    }
    Value::Null
}

fn row_record(row: &Row) -> Record {
    let mut record = Record::new();
    for (index, column) in row.columns().iter().enumerate() {
        record.insert(column.name().to_string(), from_cell(row, index));
    }
    record
}

#[async_trait]
impl PostgresDriver for TokioPostgresDriver {
    async fn open(&self, url: &str) -> Result<()> {
        let mut slot = self.client.lock().await;
        if slot.is_some() {
            return Ok(());
        }
        let (client, connection) = tokio_postgres::connect(url, NoTls).await.map_err(map_err)?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        if let Some(schema) = &self.schema {
            client
                .batch_execute(&format!("SET search_path TO {schema}"))
                .await
                .map_err(map_err)?;
        }
        *slot = Some(client);
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.client.lock().await = None;
        Ok(())
    }

    async fn execute(
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
        let slot = self.client.lock().await;
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

    async fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Record>> {
        if sql.trim().is_empty() {
            return Ok(Vec::new());
        }
        let (text, values) = to_postgres(sql, params);
        let boxed: Vec<Box<dyn ToSql + Sync + Send>> = values.iter().map(box_value).collect();
        let refs: Vec<&(dyn ToSql + Sync)> = boxed.iter().map(|v| v.as_ref() as _).collect();
        let slot = self.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Postgres driver is not open"))?;
        let rows = client.query(&text, &refs).await.map_err(map_err)?;
        Ok(rows.iter().map(row_record).collect())
    }

    async fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        let slot = self.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Postgres driver is not open"))?;
        client
            .batch_execute(&format!(
                "BEGIN ISOLATION LEVEL {}",
                isolation_sql(isolation)
            ))
            .await
            .map_err(map_err)
    }

    async fn commit(&self) -> Result<()> {
        let slot = self.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Postgres driver is not open"))?;
        client.batch_execute("COMMIT").await.map_err(map_err)
    }

    async fn rollback(&self) -> Result<()> {
        let slot = self.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Postgres driver is not open"))?;
        client.batch_execute("ROLLBACK").await.map_err(map_err)
    }
}
