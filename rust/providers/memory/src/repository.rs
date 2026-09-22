use crate::command::MemoryCommand;
use crate::connection::MemoryConnection;
use crate::filter::{matches, parse_filter};
use async_trait::async_trait;
use datasources::{
    identity_filter, ICRUDRepository, Command, IRepository, Query, IReadonlyRepository, Response,
    Result, Row, QueryStatement, Value, Where,
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

impl IRepository for MemoryRepository {}

#[async_trait]
impl IReadonlyRepository for MemoryRepository {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
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

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        let text = query.map(|q| q.get_text()).unwrap_or_default();
        let reader = MemoryCommand::new(self.connection.clone())
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
impl ICRUDRepository for MemoryRepository {
    async fn create(&self, item: &Row) -> Result<Response> {
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
