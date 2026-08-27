use async_trait::async_trait;
use datasources::{
    named_stub_connection, Command, Connection, Datasource, DatasourceFactory, Repository, Result,
    StubCommand, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(SqlServerConnection, "sqlserver");

pub struct SqlServerDatasource;

impl SqlServerDatasource {
    pub fn factory() -> Arc<Self> {
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

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new("sqlserver"))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        _table: Option<&str>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("sqlserver"))
    }
}

impl DatasourceFactory for SqlServerDatasource {
    fn provider(&self) -> &str {
        "sqlserver"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(SqlServerDatasource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(SqlServerDatasource::factory());
        assert_eq!(create("sqlserver", None).unwrap().provider(), "sqlserver");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = SqlServerDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }
}
