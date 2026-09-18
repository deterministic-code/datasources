use crate::command::SqlServerCommand;
use crate::connection::SqlServerConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, CRUDRepository, Result, StubRepository,
};
use std::sync::Arc;

pub struct SqlServerDatasource;

impl SqlServerDatasource {
    pub fn instance() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<SqlServerConnection> {
        Arc::new(SqlServerConnection::new())
    }
}

#[async_trait]
impl Datasource for SqlServerDatasource {
    fn provider(&self) -> &str {
        "sqlserver"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(SqlServerCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> {
        Arc::new(StubRepository::new("sqlserver"))
    }
}

impl DatasourceFactory for SqlServerDatasource {
    fn provider(&self) -> &str {
        "sqlserver"
    }
    fn create(
        &self,
        _options: Option<&std::collections::HashMap<String, String>>,
    ) -> Arc<dyn Datasource> {
        Arc::new(SqlServerDatasource)
    }
}
