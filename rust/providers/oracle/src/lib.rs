use async_trait::async_trait;
use datasources::{
    named_stub_connection, Command, Connection, Datasource, DatasourceFactory, Repository, Result,
    StubCommand, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(OracleConnection, "oracle");

pub struct OracleDatasource;

impl OracleDatasource {
    pub fn factory() -> Arc<Self> {
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

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new("oracle"))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        _table: Option<&str>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("oracle"))
    }
}

impl DatasourceFactory for OracleDatasource {
    fn provider(&self) -> &str {
        "oracle"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(OracleDatasource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(OracleDatasource::factory());
        assert_eq!(create("oracle", None).unwrap().provider(), "oracle");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = OracleDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }
}
