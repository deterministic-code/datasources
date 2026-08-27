use crate::command::PostgresCommand;
use crate::connection::PostgresConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result, StubRepository,
};
use std::sync::Arc;

pub struct PostgresDatasource;

impl PostgresDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<PostgresConnection> {
        Arc::new(PostgresConnection::new())
    }
}

#[async_trait]
impl Datasource for PostgresDatasource {
    fn provider(&self) -> &str {
        "postgres"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(PostgresCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("postgres"))
    }
}

impl DatasourceFactory for PostgresDatasource {
    fn provider(&self) -> &str {
        "postgres"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(PostgresDatasource)
    }
}
