use crate::bind::{to_postgres, with_returning};
use crate::connection::{map_err, PostgresConnection};
use crate::pg_value::PgValue;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use datasources::{
    parameters_to_map, split_statements, ArrayReader, Command, CommandMiddleware,
    CommandMiddlewareHost, DatasourceError, Parameter, Reader, Response, Result, Value,
};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;
use uuid::Uuid;

struct PostgresExecuteResult {
    rows_affected: i64,
    result: Value,
}

pub struct PostgresCommand {
    host: CommandMiddlewareHost,
    connection: Arc<PostgresConnection>,
}

impl PostgresCommand {
    pub fn new(connection: Arc<PostgresConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
        }
    }

    async fn execute_sql(
        connection: &PostgresConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<PostgresExecuteResult> {
        let rewritten = with_returning(sql);
        let (text, values) = to_postgres(&rewritten, params);
        let boxed: Vec<Box<dyn ToSql + Sync + Send>> = values.iter().map(box_value).collect();
        let refs: Vec<&(dyn ToSql + Sync)> = boxed.iter().map(|v| v.as_ref() as _).collect();
        let slot = connection.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        if rewritten.to_ascii_lowercase().contains("returning")
            || rewritten.trim_start().len() >= 6
                && rewritten.trim_start()[..6].eq_ignore_ascii_case("SELECT")
            || rewritten.trim_start().len() >= 4
                && rewritten.trim_start()[..4].eq_ignore_ascii_case("CALL")
        {
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

    async fn query_sql(
        connection: &PostgresConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<Vec<datasources::Row>> {
        let (text, values) = to_postgres(sql, params);
        let boxed: Vec<Box<dyn ToSql + Sync + Send>> = values.iter().map(box_value).collect();
        let refs: Vec<&(dyn ToSql + Sync)> = boxed.iter().map(|v| v.as_ref() as _).collect();
        let slot = connection.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let rows = client.query(&text, &refs).await.map_err(map_err)?;
        Ok(rows.iter().map(row_record).collect())
    }
}

#[async_trait]
impl Command for PostgresCommand {
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
                        Box::new(PostgresReader::new(
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
                    let result = PostgresCommand::execute_sql(
                        &connection,
                        &sql,
                        &parameters_to_map(params.as_deref()),
                    )
                    .await?;
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

struct BatchState {
    index: usize,
    rows: Vec<datasources::Row>,
    at: usize,
    loaded: bool,
    current: Option<datasources::Row>,
}

struct PostgresReader {
    connection: Arc<PostgresConnection>,
    statements: Vec<String>,
    params: HashMap<String, Value>,
    state: Mutex<BatchState>,
}

impl PostgresReader {
    fn new(
        connection: Arc<PostgresConnection>,
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
impl Reader for PostgresReader {
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
            let rows = PostgresCommand::query_sql(
                &self.connection,
                &self.statements[index],
                &self.params,
            )
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
