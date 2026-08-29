use crate::command::OracleCommand;
use crate::connection::OracleConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result, StubRepository,
};
use std::sync::Arc;

pub struct OracleDatasource;

impl OracleDatasource {
    pub fn instance() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<OracleConnection> {
        Arc::new(OracleConnection::new())
    }
}

#[async_trait]
impl Datasource for OracleDatasource {
    fn provider(&self) -> &str {
        "oracle"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(OracleCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("oracle"))
    }
}

impl DatasourceFactory for OracleDatasource {
    fn provider(&self) -> &str {
        "oracle"
    }
    fn create(&self, _options: Option<&std::collections::HashMap<String, String>>) -> Arc<dyn Datasource> {
        Arc::new(OracleDatasource)
    }
}
