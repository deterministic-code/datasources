use crate::driver::{isolation_sql, to_mysql, MysqlDriver, MysqlExecuteResult};
use async_trait::async_trait;
use datasources::{DatasourceError, IsolationLevel, Record, Result, Value};
use mysql_async::prelude::Queryable;
use mysql_async::{Conn, Params, Row, Value as MysqlValue};
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct MysqlAsyncDriver {
    conn: Mutex<Option<Conn>>,
}

impl MysqlAsyncDriver {
    pub fn new() -> Self {
        Self {
            conn: Mutex::new(None),
        }
    }
}

impl Default for MysqlAsyncDriver {
    fn default() -> Self {
        Self::new()
    }
}

fn map_err(error: mysql_async::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

fn to_mysql_value(value: &Value) -> MysqlValue {
    match value {
        Value::Null => MysqlValue::NULL,
        Value::Bool(flag) => MysqlValue::Int(if *flag { 1 } else { 0 }),
        Value::Int(value) => MysqlValue::Int(*value),
        Value::Float(value) => MysqlValue::Double(*value),
        Value::Text(value) => MysqlValue::Bytes(value.as_bytes().to_vec()),
    }
}

fn from_mysql_value(value: MysqlValue) -> Value {
    match value {
        MysqlValue::NULL => Value::Null,
        MysqlValue::Int(value) => Value::Int(value),
        MysqlValue::UInt(value) => Value::Int(value as i64),
        MysqlValue::Float(value) => Value::Float(value as f64),
        MysqlValue::Double(value) => Value::Float(value),
        MysqlValue::Bytes(bytes) => Value::Text(String::from_utf8_lossy(&bytes).into_owned()),
        MysqlValue::Date(..) | MysqlValue::Time(..) => Value::Text(format!("{value:?}")),
    }
}

fn named_params(params: &HashMap<String, Value>) -> Params {
    if params.is_empty() {
        return Params::Empty;
    }
    Params::from(
        params
            .iter()
            .map(|(name, value)| (name.as_str(), to_mysql_value(value)))
            .collect::<Vec<_>>(),
    )
}

fn row_record(row: Row) -> Record {
    let columns = row.columns_ref().to_vec();
    let mut record = Record::new();
    for (index, column) in columns.iter().enumerate() {
        let value = row.as_ref(index).cloned().unwrap_or(MysqlValue::NULL);
        record.insert(column.name_str().into_owned(), from_mysql_value(value));
    }
    record
}

#[async_trait]
impl MysqlDriver for MysqlAsyncDriver {
    async fn open(&self, url: &str) -> Result<()> {
        let mut slot = self.conn.lock().await;
        if slot.is_some() {
            return Ok(());
        }
        let mut conn = Conn::from_url(url.to_string()).await.map_err(map_err)?;
        conn.query_drop("SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')")
            .await
            .map_err(map_err)?;
        *slot = Some(conn);
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.conn.lock().await = None;
        Ok(())
    }

    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<MysqlExecuteResult> {
        if sql.trim().is_empty() {
            return Ok(MysqlExecuteResult {
                rows_affected: 0,
                result: Value::Null,
            });
        }
        let sql = to_mysql(sql);
        let params = named_params(params);
        let mut slot = self.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        conn.exec_drop(sql, params).await.map_err(map_err)?;
        Ok(MysqlExecuteResult {
            rows_affected: conn.affected_rows() as i64,
            result: conn
                .last_insert_id()
                .map(|id| Value::Int(id as i64))
                .unwrap_or(Value::Null),
        })
    }

    async fn query(&self, sql: &str, params: &HashMap<String, Value>) -> Result<Vec<Record>> {
        if sql.trim().is_empty() {
            return Ok(Vec::new());
        }
        let sql = to_mysql(sql);
        let params = named_params(params);
        let mut slot = self.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        let rows: Vec<Row> = conn.exec(sql, params).await.map_err(map_err)?;
        Ok(rows.into_iter().map(row_record).collect())
    }

    async fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        let sql = format!(
            "SET TRANSACTION ISOLATION LEVEL {}",
            isolation_sql(isolation)
        );
        let mut slot = self.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        conn.query_drop(sql).await.map_err(map_err)?;
        conn.query_drop("START TRANSACTION").await.map_err(map_err)
    }

    async fn commit(&self) -> Result<()> {
        let mut slot = self.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        conn.query_drop("COMMIT").await.map_err(map_err)
    }

    async fn rollback(&self) -> Result<()> {
        let mut slot = self.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        conn.query_drop("ROLLBACK").await.map_err(map_err)
    }
}
