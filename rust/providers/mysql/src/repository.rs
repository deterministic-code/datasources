use crate::connection::MysqlConnection;
use async_trait::async_trait;
use datasources::{
    DatasourceError, Query, Record, ReadonlyRepository, Repository, Response, Result, Where,
};
use std::sync::Arc;

pub struct MysqlRepository {
    #[allow(dead_code)]
    connection: Arc<MysqlConnection>,
    pub table: String,
}

impl MysqlRepository {
    pub fn new(connection: Arc<MysqlConnection>, table: Option<String>) -> Self {
        Self {
            connection,
            table: table.unwrap_or_else(|| "items".to_string()),
        }
    }
}

#[async_trait]
impl ReadonlyRepository for MysqlRepository {
    async fn get_single(&self, _query: Option<&dyn Query>) -> Result<Option<Record>> {
        Err(DatasourceError::not_implemented("mysql"))
    }

    async fn get(&self, _query: Option<&dyn Query>) -> Result<Vec<Record>> {
        Err(DatasourceError::not_implemented("mysql"))
    }

    async fn get_single_where(&self, _where_clause: &dyn Where) -> Result<Option<Record>> {
        self.get_single(None).await
    }

    async fn get_where(&self, _where_clause: &dyn Where) -> Result<Vec<Record>> {
        self.get(None).await
    }
}

#[async_trait]
impl Repository for MysqlRepository {
    async fn add(&self, _obj: &Record) -> Result<Response> {
        Err(DatasourceError::not_implemented("mysql"))
    }

    async fn update(&self, _obj: &Record, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(DatasourceError::not_implemented("mysql"))
    }

    async fn delete(&self, _obj: &Record, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(DatasourceError::not_implemented("mysql"))
    }
}
