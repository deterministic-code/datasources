use crate::command::JsonCommand;
use crate::connection::JsonConnection;
use crate::filter::{json_from_record, matches, parse_filter, value_to_json};
use async_trait::async_trait;
use datasources::{
    identity_filter, ICRUDRepository, Command, IRepository, Query, IReadonlyRepository, Response,
    Result, Row, QueryStatement, Value, Where,
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

impl IRepository for JsonRepository {}

#[async_trait]
impl IReadonlyRepository for JsonRepository {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        let text = query.map(|q| q.get_text()).unwrap_or_default();
        let reader = JsonCommand::new(self.connection.clone())
            .execute_reader(&text, None)
            .await?;
        if reader.next().await? {
            Ok(reader.current())
        } else {
            Ok(None)
        }
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        let text = query.map(|q| q.get_text()).unwrap_or_default();
        let reader = JsonCommand::new(self.connection.clone())
            .execute_reader(&text, None)
            .await?;
        datasources::collect_reader(&*reader).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.read_single(Some(&QueryStatement::text(where_clause.get_text())))
            .await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.read(Some(&QueryStatement::text(where_clause.get_text())))
            .await
    }
}

#[async_trait]
impl ICRUDRepository for JsonRepository {
    async fn create(&self, item: &Row) -> Result<Response> {
        self.connection.ensure_open()?;
        self.connection
            .collection_mut(|items| items.push(json_from_record(item)));
        self.connection.persist()?;
        Ok(Response::new(Value::Null, 1))
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(item)
                .into_iter()
                .map(|(k, v)| (k, value_to_json(&v)))
                .collect()
        };
        let json = json_from_record(item);
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

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(item)
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
