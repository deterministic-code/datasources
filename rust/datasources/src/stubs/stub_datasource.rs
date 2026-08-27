use super::stub_command::StubCommand;
use super::stub_connection::StubConnection;
use super::stub_repository::StubRepository;
use crate::command::Command;
use crate::connection::Connection;
use crate::datasource::Datasource;
use crate::datasource_factory::DatasourceFactory;
use crate::repository::Repository;
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

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new(self.provider.clone()))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new(self.provider.clone()))
    }
}

impl DatasourceFactory for StubDatasource {
    fn provider(&self) -> &str {
        &self.provider
    }

    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(StubDatasource::new(self.provider.clone()))
    }
}
