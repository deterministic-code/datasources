use crate::command::MysqlCommand;
use crate::connection::MysqlConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result, StubRepository,
};
use std::sync::Arc;

pub struct MysqlDatasource;

impl MysqlDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<MysqlConnection> {
        Arc::new(MysqlConnection::new())
    }
}

#[async_trait]
impl Datasource for MysqlDatasource {
    fn provider(&self) -> &str {
        "mysql"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(MysqlCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("mysql"))
    }
}

impl DatasourceFactory for MysqlDatasource {
    fn provider(&self) -> &str {
        "mysql"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(MysqlDatasource)
    }
}
