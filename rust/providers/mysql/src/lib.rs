use async_trait::async_trait;
use datasources::{
    named_stub_connection, Command, Connection, Datasource, DatasourceFactory, Repository, Result,
    StubCommand, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(MysqlConnection, "mysql");

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

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new("mysql"))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        _table: Option<&str>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(MysqlDatasource::factory());
        assert_eq!(create("mysql", None).unwrap().provider(), "mysql");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = MysqlDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }
}
