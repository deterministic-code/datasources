use crate::command::TextCommand;
use crate::connection::TextConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result, StubRepository,
};
use std::sync::Arc;

pub struct TextDatasource;

impl TextDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<TextConnection> {
        Arc::new(TextConnection::new())
    }
}

#[async_trait]
impl Datasource for TextDatasource {
    fn provider(&self) -> &str {
        "text"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(TextCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("text"))
    }
}

impl DatasourceFactory for TextDatasource {
    fn provider(&self) -> &str {
        "text"
    }
    fn create(&self, _options: Option<&std::collections::HashMap<String, String>>) -> Arc<dyn Datasource> {
        Arc::new(TextDatasource)
    }
}
