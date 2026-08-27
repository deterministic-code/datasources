use crate::connection::JsonConnection;
use crate::filter::{matches, parse_filter, record_from_json, value_to_json};
use async_trait::async_trait;
use datasources::{
    identity_filter, parameters_to_map, ArrayReader, Command, DatasourceError, MiddlewareAware,
    MiddlewareHook, MiddlewareHost, Parameter, Reader, Response, Result, Value,
};
use serde_json::{Map, Value as JsonValue};
use std::sync::Arc;

pub struct JsonCommand {
    host: MiddlewareHost,
    connection: Arc<JsonConnection>,
}

impl JsonCommand {
    pub fn new(connection: Arc<JsonConnection>) -> Self {
        Self {
            host: MiddlewareHost::new(),
            connection,
        }
    }
}

impl MiddlewareAware for JsonCommand {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Command for JsonCommand {
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
                    let mut obj: Map<String, JsonValue> = params
                        .iter()
                        .map(|(k, v)| (k.clone(), value_to_json(v)))
                        .collect();
                    match op.as_str() {
                        "insert" => {
                            connection
                                .collection_mut(|items| items.push(JsonValue::Object(obj.clone())));
                            connection.persist()?;
                            Ok(Response::new(Value::Null, 1))
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
                            Ok(Response::new(Value::Null, affected))
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
                            Ok(Response::new(Value::Null, affected))
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
