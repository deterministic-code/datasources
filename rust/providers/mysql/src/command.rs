use crate::connection::{map_err, MysqlConnection};
use crate::converters::{
    BinaryConverter, BoolConverter, DateTimeConverter, DecimalConverter, UuidConverter,
};
use crate::engine::to_mysql;
use async_trait::async_trait;
use datasources::{
    parameters_to_map, split_statements, ArrayReader, Command, CommandMiddleware,
    CommandMiddlewareHost, DatasourceError, Parameter, Reader, Response, Result, TypeConverter,
    Value,
};
use mysql_async::prelude::Queryable;
use mysql_async::{Params, Row, Value as MysqlValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct MysqlExecuteResult {
    rows_affected: i64,
    result: Value,
}

pub struct MysqlCommand {
    host: CommandMiddlewareHost,
    connection: Arc<MysqlConnection>,
}

impl MysqlCommand {
    pub fn new(connection: Arc<MysqlConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
        }
    }

    async fn execute_sql(
        connection: &MysqlConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<MysqlExecuteResult> {
        if sql.trim_start().len() >= 6 && sql.trim_start()[..6].eq_ignore_ascii_case("SELECT") {
            let rows = Self::query_sql(connection, sql, params).await?;
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
        let mut slot = connection.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        if params.is_empty() {
            conn.query_drop(sql.as_str()).await.map_err(map_err)?;
        } else {
            conn.exec_drop(sql, named_params(params))
                .await
                .map_err(map_err)?;
        }
        Ok(MysqlExecuteResult {
            rows_affected: conn.affected_rows() as i64,
            result: Value::Int(conn.last_insert_id().unwrap_or(0) as i64),
        })
    }

    async fn query_sql(
        connection: &MysqlConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<Vec<datasources::Row>> {
        let sql = to_mysql(sql);
        let params = named_params(params);
        let mut slot = connection.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let rows: Vec<Row> = conn.exec(sql, params).await.map_err(map_err)?;
        Ok(rows.into_iter().map(row_record).collect())
    }
}

#[async_trait]
impl Command for MysqlCommand {
    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.add_middleware(middleware);
    }

    async fn execute_reader(
        &self,
        text: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |sql, params| {
                let connection = connection.clone();
                async move {
                    if sql.trim().is_empty() {
                        return Ok((
                            Box::new(ArrayReader::new(vec![])) as Box<dyn Reader>,
                            vec![],
                        ));
                    }
                    connection.ensure_open()?;
                    let statements = split_statements(&sql);
                    if statements.is_empty() {
                        return Ok((
                            Box::new(ArrayReader::new(vec![])) as Box<dyn Reader>,
                            vec![],
                        ));
                    }
                    Ok((
                        Box::new(MysqlReader::new(
                            connection,
                            statements,
                            parameters_to_map(params.as_deref()),
                        )) as Box<dyn Reader>,
                        vec![],
                    ))
                }
            })
            .await
    }

    async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response> {
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |sql, params| {
                let connection = connection.clone();
                async move {
                    if sql.trim().is_empty() {
                        return Ok((
                            Response::of(Value::Null, 0, params.map(|list| list.to_vec())),
                            vec![],
                        ));
                    }
                    connection.ensure_open()?;
                    let mapped = parameters_to_map(params.as_deref());
                    let result = MysqlCommand::execute_sql(&connection, &sql, &mapped).await?;
                    Ok((
                        Response::of(
                            result.result,
                            result.rows_affected,
                            params.map(|list| list.to_vec()),
                        ),
                        vec![],
                    ))
                }
            })
            .await
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

struct BatchState {
    index: usize,
    rows: Vec<datasources::Row>,
    at: usize,
    loaded: bool,
    current: Option<datasources::Row>,
}

struct MysqlReader {
    connection: Arc<MysqlConnection>,
    statements: Vec<String>,
    params: HashMap<String, Value>,
    state: Mutex<BatchState>,
}

impl MysqlReader {
    fn new(
        connection: Arc<MysqlConnection>,
        statements: Vec<String>,
        params: HashMap<String, Value>,
    ) -> Self {
        Self {
            connection,
            statements,
            params,
            state: Mutex::new(BatchState {
                index: 0,
                rows: Vec::new(),
                at: 0,
                loaded: false,
                current: None,
            }),
        }
    }
}

#[async_trait]
impl Reader for MysqlReader {
    fn current(&self) -> Option<datasources::Row> {
        self.state.lock().expect("reader").current.clone()
    }

    async fn next(&self) -> Result<bool> {
        let index = {
            let state = self.state.lock().expect("reader");
            if state.loaded {
                None
            } else if state.index >= self.statements.len() {
                return Ok(false);
            } else {
                Some(state.index)
            }
        };
        if let Some(index) = index {
            let rows =
                MysqlCommand::query_sql(&self.connection, &self.statements[index], &self.params)
                    .await?;
            let mut state = self.state.lock().expect("reader");
            state.rows = rows;
            state.at = 0;
            state.loaded = true;
        }
        let mut state = self.state.lock().expect("reader");
        if state.at >= state.rows.len() {
            state.current = None;
            return Ok(false);
        }
        state.current = Some(state.rows[state.at].clone());
        state.at += 1;
        Ok(true)
    }

    async fn next_result(&self) -> Result<bool> {
        let mut state = self.state.lock().expect("reader");
        state.index += 1;
        state.loaded = false;
        state.current = None;
        Ok(state.index < self.statements.len())
    }
}
