use async_trait::async_trait;
use datasources::{
    named_stub_connection, Command, Connection, Datasource, DatasourceFactory, Repository, Result,
    StubCommand, StubRepository,
};
use std::sync::Arc;

named_stub_connection!(TextConnection, "text");

pub struct TextDatasource;

impl TextDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }

    fn new_connection(&self) -> Arc<TextConnection> {
        Arc::new(TextConnection::new())
    }
}

#[async_trait]
impl Datasource for TextDatasource {
    fn provider(&self) -> &str {
        "text"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.new_connection())
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(StubCommand::new("text"))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
        _table: Option<&str>,
    ) -> Arc<dyn Repository> {
        Arc::new(StubRepository::new("text"))
    }
}

impl DatasourceFactory for TextDatasource {
    fn provider(&self) -> &str {
        "text"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(TextDatasource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(TextDatasource::factory());
        assert_eq!(create("text", None).unwrap().provider(), "text");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = TextDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }
}
