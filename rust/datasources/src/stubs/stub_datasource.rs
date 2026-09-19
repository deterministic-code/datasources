use super::stub_connection::StubConnection;
use super::stub_repository::StubRepository;
use crate::command::Command;
use crate::connection::Connection;
use crate::datasource::Datasource;
use crate::repository::CRUDRepository;
use crate::result::Result;
use async_trait::async_trait;
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

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        connection
            .unwrap_or_else(|| self.new_connection())
            .create_command()
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn CRUDRepository> {
        Arc::new(StubRepository::new(self.provider.clone()))
    }
}
