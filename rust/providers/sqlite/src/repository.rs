use crate::connection::SqliteConnection;
use async_trait::async_trait;
use datasources::{
    DatasourceError, Query, Record, ReadonlyRepository, Repository, Response, Result, Where,
};
use std::sync::Arc;

pub struct SqliteRepository {
    #[allow(dead_code)]
    connection: Arc<SqliteConnection>,
    pub table: String,
}

impl SqliteRepository {
    pub fn new(connection: Arc<SqliteConnection>, table: Option<String>) -> Self {
        Self {
            connection,
            table: table.unwrap_or_else(|| "items".to_string()),
        }
    }
}

#[async_trait]
impl ReadonlyRepository for SqliteRepository {
    async fn get_single(&self, _query: Option<&dyn Query>) -> Result<Option<Record>> {
        Err(DatasourceError::not_implemented("sqlite"))
    }

    async fn get(&self, _query: Option<&dyn Query>) -> Result<Vec<Record>> {
        Err(DatasourceError::not_implemented("sqlite"))
    }

    async fn get_single_where(&self, _where_clause: &dyn Where) -> Result<Option<Record>> {
        self.get_single(None).await
    }

    async fn get_where(&self, _where_clause: &dyn Where) -> Result<Vec<Record>> {
        self.get(None).await
    }
}

#[async_trait]
impl Repository for SqliteRepository {
    async fn add(&self, _obj: &Record) -> Result<Response> {
        Err(DatasourceError::not_implemented("sqlite"))
    }

    async fn update(&self, _obj: &Record, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(DatasourceError::not_implemented("sqlite"))
    }

    async fn delete(&self, _obj: &Record, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(DatasourceError::not_implemented("sqlite"))
    }
}
