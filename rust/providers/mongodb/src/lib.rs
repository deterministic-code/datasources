use async_trait::async_trait;
use datasources::{
    named_stub_connection, Command, Connection, Datasource, DatasourceFactory, Repository, Result,
    StubCommand, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(MongoDbConnection, "mongodb");

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

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new("mongodb"))
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
}
