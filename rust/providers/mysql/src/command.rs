use crate::connection::{map_err, MysqlConnection};
use crate::converters;
use crate::engine::to_mysql;
use crate::reader::MysqlReader;
use crate::statement_renderer::MysqlStatementRenderer;
use async_trait::async_trait;
use datasources::{
    parameters_to_map, split_statements, statement_parameters, ArrayReader, Command, CommandMiddleware,
    CommandMiddlewareHost, DatasourceError, FirstResultReader, Native, Parameter, ParameterDirection, Reader, Response, Result, ReturnStatement,
    Statement, StatementKind, StatementTextRenderer, Value,
};
use mysql_async::prelude::Queryable;
use mysql_async::{Params, Row, Value as MysqlValue};
use std::collections::HashMap;
use std::sync::Arc;

struct MysqlExecuteResult {
    rows_affected: i64,
    result: Value,
}

pub struct MysqlCommand {
    host: CommandMiddlewareHost,
    connection: Arc<MysqlConnection>,
    statements: StatementTextRenderer,
}

impl MysqlCommand {
    pub fn new(connection: Arc<MysqlConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
            statements: StatementTextRenderer::new(Some(Box::new(MysqlStatementRenderer)), None),
        }
    }

    async fn execute_sql(
        connection: &MysqlConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<MysqlExecuteResult> {
        let sql = to_mysql(sql);
        let mut slot = connection.lock_conn().await?;
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
            result: Value::Null,
        })
    }
}

#[async_trait]
impl Command for MysqlCommand {
    fn connection(&self) -> Arc<dyn datasources::Connection> {
        Arc::clone(&self.connection) as Arc<dyn datasources::Connection>
    }

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
                    let head = statements[0].trim_start();
                    if head.len() >= 6 {
                        let verb = head[..6].to_ascii_uppercase();
                        if verb == "INSERT" || verb == "UPDATE" || verb == "DELETE" {
                            let mapped = parameters_to_map(params.as_deref());
                            let result = MysqlCommand::execute_sql(&connection, &sql, &mapped).await?;
                            let logged = logged_response(&Response::new(
                                result.result,
                                result.rows_affected,
                                params.map(|list| list.to_vec()),
                            ));
                            return Ok((
                                Box::new(ArrayReader::new(vec![])) as Box<dyn Reader>,
                                logged,
                            ));
                        }
                    }
                    FirstResultReader::capture(Box::new(
                        MysqlReader::open(
                            &connection,
                            statements,
                            parameters_to_map(params.as_deref()),
                        )
                        .await?,
                    ))
                    .await
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
                    let mapped = parameters_to_map(params.as_deref());
                    let mut rows_affected = 0;
                    let mut result = Value::Null;
                    for part in split_statements(&sql) {
                        let names = statement_parameters(&part);
                        let used = mapped
                            .iter()
                            .filter(|(name, _)| {
                                names.iter().any(|found| {
                                    found == name.trim_start_matches(['$', '@', ':'])
                                })
                            })
                            .map(|(name, value)| (name.clone(), value.clone()))
                            .collect();
                        let executed = MysqlCommand::execute_sql(&connection, &part, &used).await?;
                        rows_affected += executed.rows_affected;
                        result = executed.result;
                    }
                    let response = Response::new(
                        result,
                        rows_affected,
                        params.map(|list| list.to_vec()),
                    );
                    let logged = logged_response(&response);
                    Ok((response, logged))
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
        if parameters.is_some_and(|list| {
            list.iter()
                .any(|parameter| parameter.direction != ParameterDirection::Input)
        }) {
            return self.execute_procedure(&text, parameters).await;
        }
        self.execute_native(&Native::new(text), parameters).await
    }
}

impl MysqlCommand {
    async fn execute_procedure(
        &self,
        sql: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let connection = self.connection.clone();
        let sql = sql.to_string();
        self.host
            .run(&sql, parameters, move |sql, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let response = MysqlCommand::run_procedure(&connection, &sql, params.as_deref()).await?;
                    let logged = logged_response(&response);
                    Ok((response, logged))
                }
            })
            .await
    }

    async fn run_procedure(
        connection: &MysqlConnection,
        sql: &str,
        params: Option<&[Parameter]>,
    ) -> Result<Response> {
        let list = params.unwrap_or(&[]);
        let mut slot = connection.lock_conn().await?;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        for parameter in list {
            if parameter.direction == ParameterDirection::Input
                || parameter.direction == ParameterDirection::InputOutput
            {
                let mut one = HashMap::new();
                one.insert("value".into(), parameter.value.clone());
                conn.exec_drop(
                    format!("SET @{} = :value", parameter.name),
                    named_params(&one),
                )
                .await
                .map_err(map_err)?;
            }
        }
        conn.query_drop(session_vars(&to_mysql(sql)))
            .await
            .map_err(map_err)?;
        let outs: Vec<_> = list
            .iter()
            .filter(|parameter| parameter.direction != ParameterDirection::Input)
            .collect();
        if outs.is_empty() {
            return Ok(Response::new(
                Value::Null,
                0,
                params.map(|list| list.to_vec()),
            ));
        }
        let select = format!(
            "SELECT {}",
            outs.iter()
                .map(|parameter| format!("@{} AS `{}`", parameter.name, parameter.name))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let row: Option<mysql_async::Row> = conn.query_first(select).await.map_err(map_err)?;
        let record = row.map(row_record);
        let applied = list
            .iter()
            .map(|parameter| {
                if parameter.direction == ParameterDirection::Input {
                    parameter.clone()
                } else {
                    let value = record
                        .as_ref()
                        .and_then(|cells| cells.get(&parameter.name))
                        .cloned()
                        .unwrap_or(Value::Null);
                    Parameter {
                        value,
                        ..parameter.clone()
                    }
                }
            })
            .collect();
        Ok(Response::new(Value::Null, 0, Some(applied)))
    }

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

fn session_vars(sql: &str) -> String {
    let mut out = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ':'
            && i + 1 < chars.len()
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            out.push('@');
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                out.push(chars[i]);
                i += 1;
            }
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn logged_response(response: &Response) -> Vec<datasources::Row> {
    vec![[
        (
            "result".into(),
            match &response.result {
                Value::Null => Value::Text("null".into()),
                other => other.clone(),
            },
        ),
        ("rowsAffected".into(), Value::Int(response.rows_affected)),
    ]
    .into_iter()
    .collect()]
}

fn to_mysql_value(value: &Value) -> MysqlValue {
    match converters::reverse(value) {
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
        MysqlValue::Date(year, month, day, hour, min, sec, micros) => chrono::NaiveDate::from_ymd_opt(
            year as i32,
            month as u32,
            day as u32,
        )
        .and_then(|date| date.and_hms_micro_opt(hour as u32, min as u32, sec as u32, micros))
        .map(|naive| Value::DateTime(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc)))
        .unwrap_or_else(|| Value::Text(format!("{value:?}"))),
        MysqlValue::Time(..) => Value::Text(format!("{value:?}")),
    }
}

pub(crate) fn named_params(params: &HashMap<String, Value>) -> Params {
    if params.is_empty() {
        return Params::Empty;
    }
    let owned: Vec<(String, MysqlValue)> = params
        .iter()
        .map(|(name, value)| {
            (
                name.trim_start_matches(['$', '@', ':']).to_ascii_lowercase(),
                to_mysql_value(value),
            )
        })
        .collect();
    Params::from(
        owned
            .iter()
            .map(|(name, value)| (name.as_str(), value.clone()))
            .collect::<Vec<_>>(),
    )
}

pub(crate) fn row_record(row: Row) -> datasources::Row {
    let columns = row.columns_ref().to_vec();
    let mut record = datasources::Row::new();
    for (index, column) in columns.iter().enumerate() {
        let value = row.as_ref(index).cloned().unwrap_or(MysqlValue::NULL);
        record.insert(column.name_str().into_owned(), from_mysql_value(value));
    }
    record
}
