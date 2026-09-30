use crate::connection::{map_err, MysqlConnection};
use crate::converters;
use crate::engine::to_mysql;
use crate::reader::MysqlReader;
use crate::statement_factory::MysqlStatementFactory;
use async_trait::async_trait;
use datasources::{
    parameters_to_map, split_statements, ArrayReader, Command, CommandMiddleware,
    CommandMiddlewareHost, DatasourceError, Native, Parameter, Reader, Response, Result, ReturnStatement,
    Statement, StatementKind, Value,
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
                            MysqlReader::open(
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
                    let mapped = parameters_to_map(params.as_deref());
                    let result = MysqlCommand::execute_sql(&connection, &sql, &mapped).await?;
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
        let text = MysqlStatementFactory.transform(statement).get_text();
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
        let text = MysqlStatementFactory.transform(statement).get_text();
        self.execute_native(&Native::new(text), parameters).await
    }
}

impl MysqlCommand {
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
        MysqlValue::Date(..) | MysqlValue::Time(..) => Value::Text(format!("{value:?}")),
    }
}

pub(crate) fn named_params(params: &HashMap<String, Value>) -> Params {
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

pub(crate) fn row_record(row: Row) -> datasources::Row {
    let columns = row.columns_ref().to_vec();
    let mut record = datasources::Row::new();
    for (index, column) in columns.iter().enumerate() {
        let value = row.as_ref(index).cloned().unwrap_or(MysqlValue::NULL);
        record.insert(column.name_str().into_owned(), from_mysql_value(value));
    }
    record
}
