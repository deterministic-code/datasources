use crate::command::JsonCommand;
use crate::connection::JsonConnection;
use crate::filter::{json_from_record, matches, parse_filter, value_to_json};
use async_trait::async_trait;
use datasources::{
    identity_filter, Command, Query, Record, Repository, Response, Result, Value, Where,
};
use serde_json::Value as JsonValue;
use std::sync::Arc;

pub struct JsonRepository {
    connection: Arc<JsonConnection>,
}

impl JsonRepository {
    pub fn new(connection: Arc<JsonConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl Repository for JsonRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Record>> {
        let text = query.map(|q| q.get_text());
        let reader = JsonCommand::new(self.connection.clone())
            .execute_reader(text.as_deref(), None)
            .await?;
        if reader.next().await? {
            Ok(reader.current())
        } else {
            Ok(None)
        }
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Record>> {
        let text = query.map(|q| q.get_text());
        let reader = JsonCommand::new(self.connection.clone())
            .execute_reader(text.as_deref(), None)
            .await?;
        datasources::collect_reader(&*reader).await
    }

    async fn add(&self, obj: &Record) -> Result<Response> {
        self.connection.ensure_open()?;
        self.connection
            .collection_mut(|items| items.push(json_from_record(obj)));
        self.connection.persist()?;
        Ok(Response::new(Value::Null, 1))
    }

    async fn update(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(obj)
                .into_iter()
                .map(|(k, v)| (k, value_to_json(&v)))
                .collect()
        };
        let json = json_from_record(obj);
        let affected = self.connection.collection_mut(|items| {
            let mut count = 0i64;
            for item in items.iter_mut() {
                if matches(item, &filter) {
                    if let (JsonValue::Object(target), JsonValue::Object(src)) = (item, &json) {
                        for (k, v) in src {
                            target.insert(k.clone(), v.clone());
                        }
                    }
                    count += 1;
                }
            }
            count
        });
        self.connection.persist()?;
        Ok(Response::new(Value::Null, affected))
    }

    async fn delete(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(obj)
                .into_iter()
                .map(|(k, v)| (k, value_to_json(&v)))
                .collect()
        };
        let affected = self.connection.collection_mut(|items| {
            let before = items.len();
            items.retain(|row| !matches(row, &filter));
            (before - items.len()) as i64
        });
        self.connection.persist()?;
        Ok(Response::new(Value::Null, affected))
    }
}
