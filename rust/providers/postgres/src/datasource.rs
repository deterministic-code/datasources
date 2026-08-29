use crate::command::PostgresCommand;
use crate::connection::PostgresConnection;
use crate::driver::{require_driver, PostgresDriver};
use crate::repository::PostgresRepository;
use async_trait::async_trait;
use datasources::{Command, Connection, Datasource, DatasourceFactory, Repository, Result};
use std::sync::Arc;

pub struct PostgresDatasource {
    url: Option<String>,
    schema: Option<String>,
    driver: Option<Arc<dyn PostgresDriver>>,
    table: Option<String>,
}

impl PostgresDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self::new(None, None, None, None))
    }

    pub fn new(
        url: Option<String>,
        schema: Option<String>,
        driver: Option<Arc<dyn PostgresDriver>>,
        table: Option<String>,
    ) -> Self {
        Self {
            url,
            schema,
            driver,
            table,
        }
    }

    fn typed_connection(&self) -> Result<Arc<PostgresConnection>> {
        Ok(Arc::new(PostgresConnection::new(
            self.url.clone(),
            self.schema.clone(),
            require_driver(self.driver.clone())?,
        )))
    }

    fn connection(&self) -> Arc<PostgresConnection> {
        self.typed_connection()
            .expect("Postgres driver is required")
    }
}

#[async_trait]
impl Datasource for PostgresDatasource {
    fn provider(&self) -> &str {
        "postgres"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.typed_connection()?)
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(PostgresCommand::new(self.connection()))
    }

    fn create_repository(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Repository> {
        Arc::new(PostgresRepository::new(
            self.connection(),
            self.table.clone(),
        ))
    }
}

impl DatasourceFactory for PostgresDatasource {
    fn provider(&self) -> &str {
        "postgres"
    }

    fn create(&self, options: Option<&std::collections::HashMap<String, String>>) -> Arc<dyn Datasource> {
        Arc::new(PostgresDatasource::new(
            options.and_then(|o| o.get("url")).cloned().or_else(|| self.url.clone()),
            options
                .and_then(|o| o.get("schema"))
                .cloned()
                .or_else(|| self.schema.clone()),
            self.driver.clone(),
            options
                .and_then(|o| o.get("table"))
                .cloned()
                .or_else(|| self.table.clone()),
        ))
    }
}
