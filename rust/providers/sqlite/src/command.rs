use crate::connection::{map_err, SqliteConnection};
use crate::converters;
use crate::statement_renderer::SqliteStatementRenderer;
use async_trait::async_trait;
use datasources::{
    parameters_to_map, ArrayReader, Command, CommandMiddleware, CommandMiddlewareHost,
    DatasourceError, FirstResultReader, Native, Parameter, Reader, Response, Result, ReturnStatement, split_statements,
    statement_parameters, Statement,
    StatementKind, StatementTextRenderer, Value,
};
use rusqlite::types::{ToSql, ValueRef};
use std::collections::HashMap;
use std::sync::Arc;

struct SqliteExecuteResult {
    rows_affected: i64,
    result: Value,
}

pub struct SqliteCommand {
    host: CommandMiddlewareHost,
    connection: Arc<SqliteConnection>,
    statements: StatementTextRenderer,
}

impl SqliteCommand {
    pub fn new(connection: Arc<SqliteConnection>) -> Self {
        Self::with_renderer(
            connection,
            StatementTextRenderer::new(Some(Box::new(SqliteStatementRenderer)), None),
        )
    }

    pub fn with_renderer(connection: Arc<SqliteConnection>, statements: StatementTextRenderer) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
            statements,
        }
    }

    fn execute_sql(
        connection: &SqliteConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<SqliteExecuteResult> {
        let db = connection.db.lock().unwrap();
        let conn = db
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let (sql, values) = to_sqlite(sql, params);
        let mut statement = conn.prepare(&sql).map_err(map_err)?;
        let rows_affected = if statement.column_count() > 0 {
            let mut rows = statement
                .query(rusqlite::params_from_iter(values.iter()))
                .map_err(map_err)?;
            while rows.next().map_err(map_err)?.is_some() {}
            conn.changes() as i64
        } else {
            statement
                .execute(rusqlite::params_from_iter(values.iter()))
                .map_err(map_err)? as i64
        };
        Ok(SqliteExecuteResult {
            rows_affected,
            result: Value::Null,
        })
    }
}

#[async_trait]
impl Command for SqliteCommand {
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
                    let reader = crate::reader::SqliteReader::open(
                        &connection,
                        &sql,
                        &parameters_to_map(params.as_deref()),
                    )?;
                    FirstResultReader::capture(Box::new(reader)).await
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
                    let map = parameters_to_map(params.as_deref());
                    let mut rows_affected = 0;
                    let mut result = Value::Null;
                    for part in split_statements(&sql) {
                        let names = statement_parameters(&part);
                        let used = map
                            .iter()
                            .filter(|(name, _)| {
                                names.iter().any(|found| {
                                    found == name.trim_start_matches(['$', '@', ':'])
                                })
                            })
                            .map(|(name, value)| (name.clone(), value.clone()))
                            .collect();
                        let executed = SqliteCommand::execute_sql(&connection, &part, &used)?;
                        rows_affected += executed.rows_affected;
                        result = executed.result;
                    }
                    Ok((
                        Response::new(
                            result,
                            rows_affected,
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
        self.execute_reader_native(&Native::new(text), parameters)
            .await
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

impl SqliteCommand {
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

fn to_sqlite(sql: &str, params: &HashMap<String, Value>) -> (String, Vec<rusqlite::types::Value>) {
    let mut values = Vec::new();
    let mut out = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$'
            && i + 1 < chars.len()
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            i += 1;
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            values.push(to_sql(
                params
                    .get(&name)
                    .or_else(|| params.get(&format!("${name}")))
                    .unwrap_or(&Value::Null),
            ));
            out.push('?');
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    (out, values)
}

fn named_key(name: &str) -> String {
    match name.chars().next() {
        Some('$' | '@' | ':') => name.to_string(),
        _ => format!("${name}"),
    }
}

fn to_sql(value: &Value) -> rusqlite::types::Value {
    match converters::reverse(value) {
        Value::Null => rusqlite::types::Value::Null,
        Value::Int(value) => rusqlite::types::Value::Integer(value),
        Value::Float(value) => rusqlite::types::Value::Real(value),
        Value::Text(value) => rusqlite::types::Value::Text(value),
        Value::Binary(value) => rusqlite::types::Value::Blob(value),
        other => rusqlite::types::Value::Text(other.wire_text()),
    }
}

pub(crate) fn from_sql(value: ValueRef<'_>) -> Value {
    match value {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(value) => Value::Int(value),
        ValueRef::Real(value) => Value::Float(value),
        ValueRef::Text(bytes) => Value::Text(String::from_utf8_lossy(bytes).into_owned()),
        ValueRef::Blob(bytes) => Value::Binary(bytes.to_vec()),
    }
}

pub(crate) fn bind_pairs(params: &HashMap<String, Value>) -> Vec<(String, rusqlite::types::Value)> {
    params
        .iter()
        .map(|(name, value)| (named_key(name), to_sql(value)))
        .collect()
}

pub(crate) fn as_params(pairs: &[(String, rusqlite::types::Value)]) -> Vec<(&str, &dyn ToSql)> {
    pairs
        .iter()
        .map(|(name, value)| (name.as_str(), value as &dyn ToSql))
        .collect()
}
