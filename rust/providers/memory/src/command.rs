use crate::connection::MemoryConnection;
use crate::filter::{matches, parse_filter};
use async_trait::async_trait;
use datasources::{
    identity_filter, parameters_to_map, ArrayReader, Command, CommandMiddleware,
    CommandMiddlewareHost, DatasourceError, IWhere, Parameter, Reader, Response, Result, Row,
    Statement, StatementEvaluator, StatementKind, Value, Where,
};
use std::sync::Arc;

pub struct MemoryCommand {
    host: CommandMiddlewareHost,
    connection: Arc<MemoryConnection>,
}

impl MemoryCommand {
    pub fn new(connection: Arc<MemoryConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
        }
    }
}

#[async_trait]
impl Command for MemoryCommand {
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
            .run(text, parameters, move |query, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let mut filter = parse_filter(Some(&query))?;
                    filter.extend(parameters_to_map(params.as_deref()));
                    let rows = connection.collection(|items| {
                        items
                            .iter()
                            .filter(|row| matches(row, &filter))
                            .cloned()
                            .collect::<Vec<_>>()
                    });
                    Ok((
                        Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                        rows,
                    ))
                }
            })
            .await
    }

    async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response> {
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |query, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let op = query.trim().to_ascii_lowercase();
                    let params = parameters_to_map(params.as_deref());
                    match op.as_str() {
                        "insert" => {
                            connection.collection(|items| items.push(params.clone()));
                            Ok((Response::new(Value::Null, 1), vec![]))
                        }
                        "update" => {
                            let mut obj = params.clone();
                            let where_text = obj.remove("$where").and_then(|v| match v {
                                Value::Text(text) => Some(text),
                                _ => None,
                            });
                            let filter = if let Some(text) = where_text {
                                parse_filter(Some(&text))?
                            } else {
                                identity_filter(&obj)
                            };
                            let affected = connection.collection(|items| {
                                let mut count = 0i64;
                                for item in items.iter_mut() {
                                    if matches(item, &filter) {
                                        item.extend(obj.clone());
                                        count += 1;
                                    }
                                }
                                count
                            });
                            Ok((Response::new(Value::Null, affected), vec![]))
                        }
                        "delete" => {
                            let filter = if params.is_empty() {
                                Row::new()
                            } else {
                                identity_filter(&params)
                            };
                            let affected = connection.collection(|items| {
                                let before = items.len();
                                items.retain(|row| !matches(row, &filter));
                                (before - items.len()) as i64
                            });
                            Ok((Response::new(Value::Null, affected), vec![]))
                        }
                        _ => Err(DatasourceError::message(format!(
                            "Unsupported Memory command: {op}"
                        ))),
                    }
                }
            })
            .await
    }

    async fn execute_reader_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let Some(condition) = statement
            .predicate()
            .and_then(|predicate| predicate.as_any().downcast_ref::<Where>())
            .map(|clause| clause.condition.clone())
        else {
            return self
                .execute_reader(&filter_text(statement), parameters)
                .await;
        };
        let connection = self.connection.clone();
        self.host
            .run(statement.get_text(), parameters, move |_query, _params| {
                let connection = connection.clone();
                let condition = condition.clone();
                async move {
                    connection.ensure_open()?;
                    let evaluator = StatementEvaluator;
                    let rows = connection.collection(|items| {
                        items
                            .iter()
                            .filter(|row| evaluator.evaluate(&condition, row) == Value::Bool(true))
                            .cloned()
                            .collect::<Vec<_>>()
                    });
                    Ok((
                        Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                        rows,
                    ))
                }
            })
            .await
    }

    async fn execute_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        match statement.kind() {
            StatementKind::Insert => self.execute("insert", parameters).await,
            StatementKind::Update => {
                let params = with_where(parameters, statement.predicate());
                self.execute("update", Some(&params)).await
            }
            StatementKind::Delete => {
                let params = with_where(parameters, statement.predicate());
                self.execute("delete", Some(&params)).await
            }
            _ => self.execute(&statement.get_text(), parameters).await,
        }
    }
}

fn filter_text(statement: &dyn Statement) -> String {
    if let Some(where_clause) = statement.predicate() {
        let text = where_clause.get_text();
        if text.trim_start().starts_with('{') {
            return text;
        }
    }
    let text = statement.get_text();
    if text.trim_start().starts_with('{') || text.trim().is_empty() {
        text
    } else {
        String::new()
    }
}

fn with_where(
    parameters: Option<&[Parameter]>,
    where_clause: Option<&dyn IWhere>,
) -> Vec<Parameter> {
    let mut params = parameters.unwrap_or(&[]).to_vec();
    if let Some(text) = where_clause.map(|predicate| predicate.get_text()) {
        if text.trim_start().starts_with('{') {
            params.push(Parameter::new("$where", Value::Text(text)));
        }
    }
    params
}
