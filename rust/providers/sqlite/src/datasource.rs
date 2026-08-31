use crate::command::SqliteCommand;
use crate::connection::SqliteConnection;
use crate::driver::{require_driver, SqliteDriver};
use async_trait::async_trait;
use datasources::converters::RowTypeConverter;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result, TableRepository,
    TableRepositoryBuilderOptions,
};
use std::sync::Arc;

pub struct SqliteDatasource {
    filename: Option<String>,
    driver: Option<Arc<dyn SqliteDriver>>,
    table: Option<String>,
}

impl SqliteDatasource {
    pub fn instance() -> Arc<Self> {
        Arc::new(Self::new(Some(":memory:".into()), None, None))
    }

    pub fn new(
        filename: Option<String>,
        driver: Option<Arc<dyn SqliteDriver>>,
        table: Option<String>,
    ) -> Self {
        crate::converters::register();
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

    fn connection(&self) -> Arc<SqliteConnection> {
        self.typed_connection().expect("SQLite driver is required")
    }
}

#[async_trait]
impl Datasource for SqliteDatasource {
    fn provider(&self) -> &str {
        "sqlite"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.typed_connection()?)
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(SqliteCommand::new(self.connection()))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        let mut options = TableRepositoryBuilderOptions::with_factory(self.connection());
        options.table = self.table.clone().unwrap_or_else(|| "items".into());
        options.mapping = Some(RowTypeConverter::new(vec![]));
        Arc::new(TableRepository::new(options).expect("repository"))
    }
}

impl DatasourceFactory for SqliteDatasource {
    fn provider(&self) -> &str {
        "sqlite"
    }

    fn create(&self, options: Option<&std::collections::HashMap<String, String>>) -> Arc<dyn Datasource> {
        Arc::new(SqliteDatasource::new(
            options
                .and_then(|o| o.get("filename"))
                .cloned()
                .or_else(|| self.filename.clone()),
            self.driver.clone(),
            options
                .and_then(|o| o.get("table"))
                .cloned()
                .or_else(|| self.table.clone()),
        ))
    }
}
