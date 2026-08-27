use async_trait::async_trait;
use datasources::{
    named_stub_connection, Command, Connection, Datasource, DatasourceFactory, Repository, Result,
    StubCommand, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(YamlConnection, "yaml");

pub struct YamlDatasource;

impl YamlDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<YamlConnection> {
        Arc::new(YamlConnection::new())
    }
}

#[async_trait]
impl Datasource for YamlDatasource {
    fn provider(&self) -> &str {
        "yaml"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new("yaml"))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        _table: Option<&str>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("yaml"))
    }
}

impl DatasourceFactory for YamlDatasource {
    fn provider(&self) -> &str {
        "yaml"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(YamlDatasource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(YamlDatasource::factory());
        assert_eq!(create("yaml", None).unwrap().provider(), "yaml");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = YamlDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }
}
