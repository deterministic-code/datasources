use super::stub_connection::StubConnection;
use super::stub_repository::StubRepository;
use crate::command::Command;
use crate::datasource_error::DatasourceError;
use crate::connection::Connection;
use crate::converters::TypeConverter;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::datasource::Datasource;
use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use crate::value::Value;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub struct StubDatasource {
    provider: String,
}

impl StubDatasource {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
        }
    }

    fn new_connection(&self) -> Arc<StubConnection> {
        Arc::new(StubConnection::new(self.provider.clone()))
    }
}

#[async_trait]
impl Datasource for StubDatasource {
    fn provider(&self) -> &str {
        &self.provider
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    async fn create_command(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn Command>> {
        let connection = match connection {
            Some(connection) => connection,
            None => {
                let connection: Arc<dyn Connection> = self.new_connection();
                if connection.is_open() {
                    return Err(DatasourceError::message("Connection is already open"));
                }
                connection.open().await?;
                connection
            }
        };
        Ok(connection.create_command())
    }

    fn bind_connection(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Connection> {
        connection.unwrap_or_else(|| self.new_connection())
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        HashMap::new()
    }

    fn repository_from(
        &self,
        _connection: Arc<dyn Connection>,
        _options: CRUDRepositoryOptions,
    ) -> Arc<dyn ICRUDRepository> {
        Arc::new(StubRepository::new(self.provider.clone()))
    }

    fn readonly_repository_from(
        &self,
        _connection: Arc<dyn Connection>,
        _options: ReadonlyRepositoryMappingOptions,
    ) -> Arc<dyn IReadonlyRepository> {
        as_readonly(Arc::new(StubRepository::new(self.provider.clone())))
    }
}
