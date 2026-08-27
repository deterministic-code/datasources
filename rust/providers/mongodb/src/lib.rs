use async_trait::async_trait;
use datasources::{
    named_stub_command, named_stub_connection, Command, Connection, Datasource, DatasourceFactory,
    Repository, Result, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(MongoDbConnection, "mongodb");
named_stub_command!(MongoDbCommand, "mongodb");

pub struct MongoDbDatasource;

impl MongoDbDatasource {
    pub fn factory() -> Arc<Self> {
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
        _table: Option<&str>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("mongodb"))
    }
}

impl DatasourceFactory for MongoDbDatasource {
    fn provider(&self) -> &str {
        "mongodb"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(MongoDbDatasource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(MongoDbDatasource::factory());
        assert_eq!(create("mongodb", None).unwrap().provider(), "mongodb");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = MongoDbDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }

    #[tokio::test]
    async fn creates_command() {
        let ds = MongoDbDatasource;
        let connection = ds.create_connection().await.unwrap();
        let command = ds.create_command(Some(connection));
        assert!(command.execute(None, None).await.is_err());
        assert!(command.execute_reader(None, None).await.is_err());
        let implicit = ds.create_command(None);
        assert!(implicit.execute(None, None).await.is_err());
        assert!(implicit.execute_reader(None, None).await.is_err());
    }
}
