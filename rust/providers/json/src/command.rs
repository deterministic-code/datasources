use crate::connection::JsonConnection;
use crate::filter::{matches, parse_filter, record_from_json, value_to_json};
use async_trait::async_trait;
use datasources::{
    identity_filter, parameters_to_map, ArrayReader, Command, CommandMiddleware,
    CommandMiddlewareHost, DatasourceError, Parameter, Reader, Response, Result, Value,
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
    fn use_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.use_middleware(middleware);
    }

    async fn execute_reader(
        &self,
        text: Option<&str>,
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

    async fn execute(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
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
                                identity_filter(&record_from_json(&JsonValue::Object(obj.clone())))
                                    .into_iter()
                                    .map(|(k, v)| (k, value_to_json(&v)))
                                    .collect()
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
                            let filter = if obj.is_empty() {
                                Map::new()
                            } else {
                                identity_filter(&record_from_json(&JsonValue::Object(obj.clone())))
                                    .into_iter()
                                    .map(|(k, v)| (k, value_to_json(&v)))
                                    .collect()
                            };
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
}
