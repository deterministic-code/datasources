use crate::connection::PostgresConnection;
use async_trait::async_trait;
use datasources::{
    DatasourceError, Query, Row, ReadonlyRepository, Repository, Response, Result, Where,
};
use std::sync::Arc;

pub struct PostgresRepository {
    #[allow(dead_code)]
    connection: Arc<PostgresConnection>,
    pub table: String,
}

impl PostgresRepository {
    pub fn new(connection: Arc<PostgresConnection>, table: Option<String>) -> Self {
        Self {
            connection,
            table: table.unwrap_or_else(|| "items".to_string()),
        }
    }
}

#[async_trait]
impl ReadonlyRepository for PostgresRepository {
    async fn get_single(&self, _query: Option<&dyn Query>) -> Result<Option<Row>> {
        Err(DatasourceError::not_implemented("postgres"))
    }

    async fn get(&self, _query: Option<&dyn Query>) -> Result<Vec<Row>> {
        Err(DatasourceError::not_implemented("postgres"))
    }

    async fn get_single_where(&self, _where_clause: &dyn Where) -> Result<Option<Row>> {
        self.get_single(None).await
    }

    async fn get_where(&self, _where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.get(None).await
    }
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn add(&self, _item: &Row) -> Result<Response> {
        Err(DatasourceError::not_implemented("postgres"))
    }

    async fn update(&self, _item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(DatasourceError::not_implemented("postgres"))
    }

    async fn delete(&self, _item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(DatasourceError::not_implemented("postgres"))
    }
}
