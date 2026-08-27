use async_trait::async_trait;
use datasources::{
    named_stub_connection, Command, Connection, Datasource, DatasourceFactory, Repository, Result,
    StubCommand, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(XmlConnection, "xml");

pub struct XmlDatasource;

impl XmlDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<XmlConnection> {
        Arc::new(XmlConnection::new())
    }
}

#[async_trait]
impl Datasource for XmlDatasource {
    fn provider(&self) -> &str {
        "xml"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new("xml"))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        _table: Option<&str>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("xml"))
    }
}

impl DatasourceFactory for XmlDatasource {
    fn provider(&self) -> &str {
        "xml"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(XmlDatasource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(XmlDatasource::factory());
        assert_eq!(create("xml", None).unwrap().provider(), "xml");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = XmlDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }
}
