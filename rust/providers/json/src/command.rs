use crate::connection::JsonConnection;
use crate::filter::{matches, parse_filter, record_from_json, value_to_json};
use async_trait::async_trait;
use datasources::{
    parameters_to_map, ArrayReader, Command, CommandMiddleware, CommandMiddlewareHost,
    DatasourceError, IWhere, Parameter, Reader, Response, Result, Statement, StatementKind, Value,
};
use serde_json::{Map, Value as JsonValue};
use std::sync::Arc;

pub struct JsonCommand {
    host: CommandMiddlewareHost,
    connection: Arc<JsonConnection>,
}

impl JsonCommand {
    pub fn new(connection: Arc<JsonConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
        }
    }
}

#[async_trait]
impl Command for JsonCommand {
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
                    for (k, v) in parameters_to_map(params.as_deref()) {
                        filter.insert(k, value_to_json(&v));
                    }
                    let rows = connection.collection_mut(|items| {
                        items
                            .iter()
                            .filter(|row| matches(row, &filter))
                            .map(record_from_json)
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
                    let mut obj: Map<String, JsonValue> = params
                        .iter()
                        .map(|(k, v)| (k.clone(), value_to_json(v)))
                        .collect();
                    match op.as_str() {
                        "insert" => {
                            connection
                                .collection_mut(|items| items.push(JsonValue::Object(obj.clone())));
                            connection.persist()?;
                            Ok((Response::new(Value::Null, 1), vec![]))
                        }
                        "update" => {
                            let where_text = obj
                                .remove("$where")
                                .and_then(|v| v.as_str().map(ToOwned::to_owned));
                            let filter = if let Some(text) = where_text {
                                parse_filter(Some(&text))?
                            } else {
                                obj.clone()
                            };
                            let affected = connection.collection_mut(|items| {
                                let mut count = 0;
                                for item in items.iter_mut() {
                                    if matches(item, &filter) {
                                        if let JsonValue::Object(map) = item {
                                            for (k, v) in &obj {
                                                map.insert(k.clone(), v.clone());
                                            }
                                        }
                                        count += 1;
                                    }
                                }
                                count
                            });
                            connection.persist()?;
                            Ok((Response::new(Value::Null, affected), vec![]))
                        }
                        "delete" => {
                            let filter = obj.clone();
                            let affected = connection.collection_mut(|items| {
                                let before = items.len();
                                items.retain(|row| !matches(row, &filter));
                                (before - items.len()) as i64
                            });
                            connection.persist()?;
                            Ok((Response::new(Value::Null, affected), vec![]))
                        }
                        _ => Err(DatasourceError::message(format!(
                            "Unsupported JSON command: {op}"
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
        if let Some(inserted) = statement
            .as_any()
            .downcast_ref::<datasources::SelectInsertedIdStatement>()
        {
            let connection = self.connection.clone();
            let text = format!("{}.{}", inserted.table, inserted.column);
            return self
                .host
                .run(&text, parameters, move |_query, _params| {
                    let connection = connection.clone();
                    async move {
                        connection.ensure_open()?;
                        Ok((
                            Box::new(ArrayReader::new(vec![])) as Box<dyn Reader>,
                            vec![],
                        ))
                    }
                })
                .await;
        }
        self.execute_reader(&filter_text(statement), parameters)
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
