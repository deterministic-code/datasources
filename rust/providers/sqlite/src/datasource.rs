use crate::command::SqliteCommand;
use crate::connection::SqliteConnection;
use crate::driver::{require_driver, SqliteDriver};
use crate::rusqlite_driver::RusqliteDriver;
use crate::repository::SqliteRepository;
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result,
};
use std::sync::Arc;

pub struct SqliteDatasource {
    filename: Option<String>,
    driver: Option<Arc<dyn SqliteDriver>>,
    table: Option<String>,
}

impl SqliteDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self::new(
            Some(":memory:".into()),
            Some(Arc::new(RusqliteDriver::new())),
            None,
        ))
    }

    pub fn new(
        filename: Option<String>,
        driver: Option<Arc<dyn SqliteDriver>>,
        table: Option<String>,
    ) -> Self {
        Self {
            filename,
            driver,
            table,
        }
    }

    fn typed_connection(&self) -> Result<Arc<SqliteConnection>> {
        Ok(Arc::new(SqliteConnection::new(
            self.filename.clone(),
            require_driver(self.driver.clone())?,
        )))
    }
}

#[async_trait]
impl Datasource for SqliteDatasource {
    fn provider(&self) -> &str {
        "sqlite"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self
            .typed_connection()
            .unwrap_or_else(|_| Arc::new(SqliteConnection::new(None, Arc::new(RusqliteDriver::new())))))
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        let conn = self
            .typed_connection()
            .unwrap_or_else(|_| Arc::new(SqliteConnection::new(None, Arc::new(RusqliteDriver::new()))));
        Arc::new(SqliteCommand::new(conn))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        let conn = self
            .typed_connection()
            .unwrap_or_else(|_| Arc::new(SqliteConnection::new(None, Arc::new(RusqliteDriver::new()))));
        Arc::new(SqliteRepository::new(conn, self.table.clone()))
    }
}

impl DatasourceFactory for SqliteDatasource {
    fn provider(&self) -> &str {
        "sqlite"
    }

    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        Arc::new(SqliteDatasource::new(
            self.filename.clone(),
            self.driver.clone(),
            self.table.clone(),
        ))
    }
}
