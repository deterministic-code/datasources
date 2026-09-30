use super::mock_command::MockCommand;
use super::mock_connection::MockConnection;
use super::mock_repository::MockRepository;
use crate::command::Command;
use crate::connection::Connection;
use crate::converters::{RowTypeConverter, TypeConverter};
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::datasource::Datasource;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use crate::value::Value;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MockDatasource {
    provider: String,
    connection: Arc<dyn Connection>,
    command: Arc<dyn Command>,
    repository: Arc<dyn ICRUDRepository>,
}

impl MockDatasource {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            connection: MockConnection::new(),
            command: Arc::new(MockCommand::new()),
            repository: Arc::new(MockRepository::new()),
        }
    }
}

#[async_trait]
impl Datasource for MockDatasource {
    fn provider(&self) -> &str {
        &self.provider
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.connection.clone())
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        self.command.clone()
    }

    fn bind_connection(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Connection> {
        connection.unwrap_or_else(|| self.connection.clone())
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        HashMap::new()
    }

    fn row_converter(&self, columns: &HashMap<String, String>) -> RowTypeConverter {
        RowTypeConverter::new(columns.keys().cloned().collect())
    }

    fn repository_from(
        &self,
        _connection: Arc<dyn Connection>,
        _options: CRUDRepositoryOptions,
    ) -> Arc<dyn ICRUDRepository> {
        self.repository.clone()
    }

    fn readonly_repository_from(
        &self,
        _connection: Arc<dyn Connection>,
        _options: ReadonlyRepositoryOptions,
    ) -> Arc<dyn IReadonlyRepository> {
        as_readonly(self.repository.clone())
    }
}
