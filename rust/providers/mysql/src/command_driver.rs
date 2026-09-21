use crate::connection_driver::{map_err, MysqlConnectionDriver};
use crate::converters::{
    BinaryConverter, BoolConverter, DateTimeConverter, DecimalConverter, UuidConverter,
};
use crate::driver::{to_mysql, MysqlExecuteResult};
use datasources::{DatasourceError, Result, TypeConverter, Value};
use mysql_async::prelude::Queryable;
use mysql_async::{Params, Row, Value as MysqlValue};
use std::collections::HashMap;

pub struct MysqlCommandDriver<'a> {
    connection: &'a MysqlConnectionDriver,
}

impl<'a> MysqlCommandDriver<'a> {
    pub fn new(connection: &'a MysqlConnectionDriver) -> Self {
        Self { connection }
    }

    pub async fn execute(
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
        if sql.trim_start().len() >= 6 && sql.trim_start()[..6].eq_ignore_ascii_case("SELECT") {
            let rows = self.query(sql, params).await?;
            let result = rows
                .first()
                .and_then(|row| row.values().next().cloned())
                .unwrap_or(Value::Null);
            return Ok(MysqlExecuteResult {
                rows_affected: if rows.is_empty() { 0 } else { 1 },
                result,
            });
        }
        let sql = to_mysql(sql);
        let params = named_params(params);
        let mut slot = self.connection.conn.lock().await;
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

    pub async fn query(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<Vec<datasources::Row>> {
        if sql.trim().is_empty() {
            return Ok(Vec::new());
        }
        let sql = to_mysql(sql);
        let params = named_params(params);
        let mut slot = self.connection.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        let rows: Vec<Row> = conn.exec(sql, params).await.map_err(map_err)?;
        Ok(rows.into_iter().map(row_record).collect())
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

fn to_mysql_value(value: &Value) -> MysqlValue {
    match reverse(value) {
        Value::Null => MysqlValue::NULL,
        Value::Int(value) => MysqlValue::Int(value),
        Value::Float(value) => MysqlValue::Double(value),
        Value::Text(value) => MysqlValue::Bytes(value.into_bytes()),
        Value::Binary(value) => MysqlValue::Bytes(value),
        other => MysqlValue::Bytes(other.wire_text().into_bytes()),
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

fn row_record(row: Row) -> datasources::Row {
    let columns = row.columns_ref().to_vec();
    let mut record = datasources::Row::new();
    for (index, column) in columns.iter().enumerate() {
        let value = row.as_ref(index).cloned().unwrap_or(MysqlValue::NULL);
        record.insert(column.name_str().into_owned(), from_mysql_value(value));
    }
    record
}
