use async_trait::async_trait;
use datasources::{
    named_stub_connection, Command, Connection, Datasource, DatasourceFactory, Repository, Result,
    StubCommand, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(PostgresConnection, "postgres");

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

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new("postgres"))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        _table: Option<&str>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(PostgresDatasource::factory());
        assert_eq!(create("postgres", None).unwrap().provider(), "postgres");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = PostgresDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }
}
