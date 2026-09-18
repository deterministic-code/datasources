use crate::command::DynamoDbCommand;
use crate::connection::DynamoDbConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, CRUDRepository, Result, StubRepository,
};
use std::sync::Arc;

pub struct DynamoDbDatasource;

impl DynamoDbDatasource {
    pub fn instance() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<DynamoDbConnection> {
        Arc::new(DynamoDbConnection::new())
    }
}

#[async_trait]
impl Datasource for DynamoDbDatasource {
    fn provider(&self) -> &str {
        "dynamodb"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(DynamoDbCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> {
        Arc::new(StubRepository::new("dynamodb"))
    }
}

impl DatasourceFactory for DynamoDbDatasource {
    fn provider(&self) -> &str {
        "dynamodb"
    }
    fn create(
        &self,
        _options: Option<&std::collections::HashMap<String, String>>,
    ) -> Arc<dyn Datasource> {
        Arc::new(DynamoDbDatasource)
    }
}
