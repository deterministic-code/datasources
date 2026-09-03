use crate::connection::MemoryConnection;
use crate::filter::{matches, parse_filter};
use async_trait::async_trait;
use datasources::{
    identity_filter, parameters_to_map, ArrayReader, Command, CommandMiddleware,
    CommandMiddlewareHost, DatasourceError, Parameter, Reader, Response, Result, Row, Value,
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
    fn use_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.use_middleware(middleware);
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
}
