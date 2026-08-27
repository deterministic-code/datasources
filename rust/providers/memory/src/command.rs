use crate::connection::MemoryConnection;
use crate::filter::{matches, parse_filter};
use async_trait::async_trait;
use datasources::{
    identity_filter, parameters_to_map, ArrayReader, Command, DatasourceError, MiddlewareAware,
    MiddlewareHook, MiddlewareHost, Parameter, Reader, Record, Response, Result, Value,
};
use std::sync::Arc;

pub struct MemoryCommand {
    host: MiddlewareHost,
    connection: Arc<MemoryConnection>,
}

impl MemoryCommand {
    pub fn new(connection: Arc<MemoryConnection>) -> Self {
        Self {
            host: MiddlewareHost::new(),
            connection,
        }
    }
}

impl MiddlewareAware for MemoryCommand {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Command for MemoryCommand {
    async fn execute_reader(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let extras = Some((
            text.map(ToOwned::to_owned),
            parameters.map(|items| items.to_vec()),
        ));
        let connection = self.connection.clone();
        let text_owned = text.map(ToOwned::to_owned);
        let params = parameters.map(|items| items.to_vec());
        self.host
            .run("executeReader", extras, move || {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let mut filter = parse_filter(text_owned.as_deref())?;
                    filter.extend(parameters_to_map(params.as_deref()));
                    let rows = connection.collection(|items| {
                        items
                            .iter()
                            .filter(|row| matches(row, &filter))
                            .cloned()
                            .collect::<Vec<_>>()
                    });
                    Ok(Box::new(ArrayReader::new(rows)) as Box<dyn Reader>)
                }
            })
            .await
    }

    async fn execute(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let extras = Some((
            text.map(ToOwned::to_owned),
            parameters.map(|items| items.to_vec()),
        ));
        let connection = self.connection.clone();
        let op = text.unwrap_or("").trim().to_ascii_lowercase();
        let params = parameters_to_map(parameters);
        self.host
            .run("execute", extras, move || {
                let connection = connection.clone();
                let op = op.clone();
                async move {
                    connection.ensure_open()?;
                    match op.as_str() {
                        "insert" => {
                            connection.collection(|items| items.push(params.clone()));
                            Ok(Response::new(Value::Null, 1))
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
                            Ok(Response::new(Value::Null, affected))
                        }
                        "delete" => {
                            let filter = if params.is_empty() {
                                Record::new()
                            } else {
                                identity_filter(&params)
                            };
                            let affected = connection.collection(|items| {
                                let before = items.len();
                                items.retain(|row| !matches(row, &filter));
                                (before - items.len()) as i64
                            });
                            Ok(Response::new(Value::Null, affected))
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
