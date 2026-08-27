use async_trait::async_trait;
use datasources::{
    named_stub_command, named_stub_connection, Command, Connection, Datasource, DatasourceFactory,
    Repository, Result, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(DynamoDbConnection, "dynamodb");
named_stub_command!(DynamoDbCommand, "dynamodb");

pub struct DynamoDbDatasource;

impl DynamoDbDatasource {
    pub fn factory() -> Arc<Self> {
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

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("dynamodb"))
    }
}

impl DatasourceFactory for DynamoDbDatasource {
    fn provider(&self) -> &str {
        "dynamodb"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(DynamoDbDatasource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(DynamoDbDatasource::factory());
        assert_eq!(create("dynamodb", None).unwrap().provider(), "dynamodb");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = DynamoDbDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }

    #[tokio::test]
    async fn creates_command() {
        let ds = DynamoDbDatasource;
        let connection = ds.create_connection().await.unwrap();
        let command = ds.create_command(Some(connection));
        assert!(command.execute(None, None).await.is_err());
        assert!(command.execute_reader(None, None).await.is_err());
        let implicit = ds.create_command(None);
        assert!(implicit.execute(None, None).await.is_err());
        assert!(implicit.execute_reader(None, None).await.is_err());
    }
}
