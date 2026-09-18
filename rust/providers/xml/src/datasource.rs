use crate::command::XmlCommand;
use crate::connection::XmlConnection;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, CRUDRepository, Result, StubRepository,
};
use std::sync::Arc;

pub struct XmlDatasource;

impl XmlDatasource {
    pub fn instance() -> Arc<Self> {
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

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(XmlCommand::new(
            connection.unwrap_or_else(|| self.new_connection()),
        ))
    }

    fn create_repository(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> {
        Arc::new(StubRepository::new("xml"))
    }
}

impl DatasourceFactory for XmlDatasource {
    fn provider(&self) -> &str {
        "xml"
    }
    fn create(
        &self,
        _options: Option<&std::collections::HashMap<String, String>>,
    ) -> Arc<dyn Datasource> {
        Arc::new(XmlDatasource)
    }
}
