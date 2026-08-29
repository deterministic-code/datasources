use crate::command::MongoDbCommand;
use crate::connection::MongoDbConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result, StubRepository,
};
use std::sync::Arc;

pub struct MongoDbDatasource;

impl MongoDbDatasource {
    pub fn instance() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<MongoDbConnection> {
        Arc::new(MongoDbConnection::new())
    }
}

#[async_trait]
impl Datasource for MongoDbDatasource {
    fn provider(&self) -> &str {
        "mongodb"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(MongoDbCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("mongodb"))
    }
}

impl DatasourceFactory for MongoDbDatasource {
    fn provider(&self) -> &str {
        "mongodb"
    }
    fn create(&self, _options: Option<&std::collections::HashMap<String, String>>) -> Arc<dyn Datasource> {
        Arc::new(MongoDbDatasource)
    }
}
