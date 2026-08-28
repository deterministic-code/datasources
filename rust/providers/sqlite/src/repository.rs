use crate::connection::SqliteConnection;
use async_trait::async_trait;
use datasources::{DatasourceError, Query, Record, Repository, Response, Result, Where};
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
impl Repository for SqliteRepository {
    async fn get_single(&self, _query: Option<&dyn Query>) -> Result<Option<Record>> {
        Err(DatasourceError::not_implemented("sqlite"))
    }

    async fn get(&self, _query: Option<&dyn Query>) -> Result<Vec<Record>> {
        Err(DatasourceError::not_implemented("sqlite"))
    }

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
