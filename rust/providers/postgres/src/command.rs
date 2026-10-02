use crate::bind::to_postgres;
use crate::connection::{map_err, PostgresConnection};
use crate::pg_value::PgValue;
use crate::reader::PostgresReader;
use crate::statement_renderer::PostgresStatementRenderer;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use datasources::{
    parameters_to_map, split_statements, ArrayReader, Command, CommandMiddleware,
    CommandMiddlewareHost, DatasourceError, Native, Parameter, ParameterDirection, Reader, Response,
    Result, ReturnStatement, Statement, StatementKind, StatementTextRenderer, Value,
};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;
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
    statements: StatementTextRenderer,
}

impl PostgresCommand {
    pub fn new(connection: Arc<PostgresConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
            statements: StatementTextRenderer::new(Some(Box::new(PostgresStatementRenderer)), None),
        }
    }

    async fn execute_sql(
        connection: &PostgresConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<PostgresExecuteResult> {
        let (text, values) = to_postgres(sql, params);
        let boxed: Vec<Box<dyn ToSql + Sync + Send>> = values.iter().map(box_value).collect();
        let refs: Vec<&(dyn ToSql + Sync)> = boxed.iter().map(|v| v.as_ref() as _).collect();
        let slot = connection.lock_client().await?;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let rows = client.execute(&text, &refs).await.map_err(map_err)?;
        Ok(PostgresExecuteResult {
            rows_affected: rows as i64,
            result: Value::Null,
        })
    }

    async fn query_first(
        connection: &PostgresConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<Option<Row>> {
        let (text, values) = to_postgres(sql, params);
        let boxed: Vec<Box<dyn ToSql + Sync + Send>> = values.iter().map(box_value).collect();
        let refs: Vec<&(dyn ToSql + Sync)> = boxed.iter().map(|v| v.as_ref() as _).collect();
        let slot = connection.lock_client().await?;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let rows = client.query(&text, &refs).await.map_err(map_err)?;
        Ok(rows.into_iter().next())
    }
}

#[async_trait]
impl Command for PostgresCommand {
    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.add_middleware(middleware);
    }

    async fn execute_reader_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let connection = self.connection.clone();
        self.host
            .run(text.text(), parameters, move |sql, params| {
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
                        Box::new(
                            PostgresReader::open(
                                &connection,
                                statements,
                                parameters_to_map(params.as_deref()),
                            )
                            .await?,
                        ) as Box<dyn Reader>,
                        vec![],
                    ))
                }
            })
            .await
    }

    async fn execute_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let connection = self.connection.clone();
        self.host
            .run(text.text(), parameters, move |sql, params| {
                let connection = connection.clone();
                async move {
                    if sql.trim().is_empty() {
                        return Ok((
                            Response::new(Value::Null, 0, params.map(|list| list.to_vec())),
                            vec![],
                        ));
                    }
                    connection.ensure_open()?;
                    let outputs = params.as_ref().is_some_and(|list| {
                        list.iter()
                            .any(|parameter| parameter.direction != ParameterDirection::Input)
                    });
                    if outputs {
                        let row = PostgresCommand::query_first(
                            &connection,
                            &sql,
                            &parameters_to_map(params.as_deref()),
                        )
                        .await?;
                        let record = row.as_ref().map(row_record);
                        let applied = params.map(|list| {
                            list.iter()
                                .map(|parameter| {
                                    let value = (parameter.direction != ParameterDirection::Input)
                                        .then(|| {
                                            record
                                                .as_ref()
                                                .and_then(|cells| cells.get(&parameter.name))
                                        })
                                        .flatten()
                                        .cloned();
                                    match value {
                                        Some(value) => Parameter {
                                            value,
                                            ..parameter.clone()
                                        },
                                        None => parameter.clone(),
                                    }
                                })
                                .collect()
                        });
                        return Ok((
                            Response::new(Value::Null, i64::from(row.is_some()), applied),
                            vec![],
                        ));
                    }
                    let result = PostgresCommand::execute_sql(
                        &connection,
                        &sql,
                        &parameters_to_map(params.as_deref()),
                    )
                    .await?;
                    Ok((
                        Response::new(
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

    async fn execute_reader_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let text = self.statements.render(statement);
        self.execute_reader_native(&Native::new(text), parameters).await
    }

    async fn execute_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        if statement.kind() == StatementKind::Select || statement.as_any().is::<ReturnStatement>() {
            return self.first_cell(statement, parameters).await;
        }
        let text = self.statements.render(statement);
        self.execute_native(&Native::new(text), parameters).await
    }
}

impl PostgresCommand {
    async fn first_cell(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let reader = self.execute_reader_statement(statement, parameters).await?;
        if !reader.next().await? {
            return Ok(Response::new(
                Value::Null,
                0,
                parameters.map(|list| list.to_vec()),
            ));
        }
        let row = reader
            .current()
            .ok_or_else(|| DatasourceError::message("Reader current must be a row"))?;
        let cell = row.into_values().next().unwrap_or(Value::Null);
        Ok(Response::new(cell, 1, parameters.map(|list| list.to_vec())))
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

pub(crate) fn row_record(row: &Row) -> datasources::Row {
    let mut record = datasources::Row::new();
    for (index, column) in row.columns().iter().enumerate() {
        record.insert(column.name().to_string(), from_cell(row, index));
    }
    record
}
