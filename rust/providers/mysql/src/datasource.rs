use crate::command::MysqlCommand;
use crate::connection::MysqlConnection;
use crate::driver::{require_driver, MysqlDriver};
use crate::repository::MysqlRepository;
use async_trait::async_trait;
use datasources::{Command, Connection, Datasource, DatasourceFactory, Repository, Result};
use std::sync::Arc;

pub struct MysqlDatasource {
    url: Option<String>,
    driver: Option<Arc<dyn MysqlDriver>>,
    table: Option<String>,
}

impl MysqlDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self::new(None, None, None))
    }

    pub fn new(
        url: Option<String>,
        driver: Option<Arc<dyn MysqlDriver>>,
        table: Option<String>,
    ) -> Self {
        Self { url, driver, table }
    }

    fn typed_connection(&self) -> Result<Arc<MysqlConnection>> {
        Ok(Arc::new(MysqlConnection::new(
            self.url.clone(),
            require_driver(self.driver.clone())?,
        )))
    }

    fn connection(&self) -> Arc<MysqlConnection> {
        self.typed_connection().expect("MySQL driver is required")
    }
}

#[async_trait]
impl Datasource for MysqlDatasource {
    fn provider(&self) -> &str {
        "mysql"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.typed_connection()?)
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(MysqlCommand::new(self.connection()))
    }

    fn create_repository(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Repository> {
        Arc::new(MysqlRepository::new(self.connection(), self.table.clone()))
    }
}

impl DatasourceFactory for MysqlDatasource {
    fn provider(&self) -> &str {
        "mysql"
    }

    fn create(&self, options: Option<&std::collections::HashMap<String, String>>) -> Arc<dyn Datasource> {
        Arc::new(MysqlDatasource::new(
            options.and_then(|o| o.get("url")).cloned().or_else(|| self.url.clone()),
            self.driver.clone(),
            options
                .and_then(|o| o.get("table"))
                .cloned()
                .or_else(|| self.table.clone()),
        ))
    }
}
