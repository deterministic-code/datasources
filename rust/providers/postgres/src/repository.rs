use crate::connection::PostgresConnection;
use async_trait::async_trait;
use datasources::{DatasourceError, Query, Record, Repository, Response, Result, Where};
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
impl Repository for PostgresRepository {
    async fn get_single(&self, _query: Option<&dyn Query>) -> Result<Option<Record>> {
        Err(DatasourceError::not_implemented("postgres"))
    }

    async fn get(&self, _query: Option<&dyn Query>) -> Result<Vec<Record>> {
        Err(DatasourceError::not_implemented("postgres"))
    }

    async fn add(&self, _obj: &Record) -> Result<Response> {
        Err(DatasourceError::not_implemented("postgres"))
    }

    async fn update(&self, _obj: &Record, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(DatasourceError::not_implemented("postgres"))
    }

    async fn delete(&self, _obj: &Record, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(DatasourceError::not_implemented("postgres"))
    }
}
