use crate::command::YamlCommand;
use crate::connection::YamlConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result, StubRepository,
};
use std::sync::Arc;

pub struct YamlDatasource;

impl YamlDatasource {
    pub fn instance() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<YamlConnection> {
        Arc::new(YamlConnection::new())
    }
}

#[async_trait]
impl Datasource for YamlDatasource {
    fn provider(&self) -> &str {
        "yaml"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(YamlCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("yaml"))
    }
}

impl DatasourceFactory for YamlDatasource {
    fn provider(&self) -> &str {
        "yaml"
    }
    fn create(
        &self,
        _options: Option<&std::collections::HashMap<String, String>>,
    ) -> Arc<dyn Datasource> {
        Arc::new(YamlDatasource)
    }
}
