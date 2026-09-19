use super::mock_command::MockCommand;
use super::mock_connection::MockConnection;
use super::mock_repository::MockRepository;
use crate::command::Command;
use crate::connection::Connection;
use crate::datasource::Datasource;
use crate::repository::CRUDRepository;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct MockDatasource {
    provider: String,
    connection: Arc<dyn Connection>,
    command: Arc<dyn Command>,
    repository: Arc<dyn CRUDRepository>,
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

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn CRUDRepository> {
        self.repository.clone()
    }
}
