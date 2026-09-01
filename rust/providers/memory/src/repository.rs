use crate::command::MemoryCommand;
use crate::connection::MemoryConnection;
use crate::filter::{matches, parse_filter};
use async_trait::async_trait;
use datasources::{
    identity_filter, Command, Query, Row, ReadonlyRepository, Repository, Response, Result,
    TextQuery, Value, Where,
};
use std::sync::Arc;

pub struct MemoryRepository {
    connection: Arc<MemoryConnection>,
}

impl MemoryRepository {
    pub fn new(connection: Arc<MemoryConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ReadonlyRepository for MemoryRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        let text = query.map(|q| q.get_text()).unwrap_or_default();
        let reader = MemoryCommand::new(self.connection.clone())
            .execute_reader(&text, None)
            .await?;
        if reader.next().await? {
            Ok(reader.current())
        } else {
            Ok(None)
        }
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        let text = query.map(|q| q.get_text()).unwrap_or_default();
        let reader = MemoryCommand::new(self.connection.clone())
            .execute_reader(&text, None)
            .await?;
        datasources::collect_reader(&*reader).await
    }

    async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.get_single(Some(&TextQuery::new(where_clause.get_text())))
            .await
    }

    async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.get(Some(&TextQuery::new(where_clause.get_text()))).await
    }
}

#[async_trait]
impl Repository for MemoryRepository {
    async fn add(&self, item: &Row) -> Result<Response> {
        self.connection.ensure_open()?;
        self.connection.collection(|items| items.push(item.clone()));
        Ok(Response::new(Value::Null, 1))
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(item)
        };
        let affected = self.connection.collection(|items| {
            let mut count = 0i64;
            for row in items.iter_mut() {
                if matches(row, &filter) {
                    row.extend(item.clone());
                    count += 1;
                }
            }
            count
        });
        Ok(Response::new(Value::Null, affected))
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.connection.ensure_open()?;
        let filter = if let Some(w) = by_where {
            parse_filter(Some(&w.get_text()))?
        } else {
            identity_filter(item)
        };
        let affected = self.connection.collection(|items| {
            let before = items.len();
            items.retain(|row| !matches(row, &filter));
            (before - items.len()) as i64
        });
        Ok(Response::new(Value::Null, affected))
    }
}
