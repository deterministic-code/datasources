use crate::command::Command;
use crate::connection::Connection;
use crate::crud_repository::CRUDRepository;
use crate::driver::Driver;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use crate::table_repository_builder_options::RepositoryBuilderOptions;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Datasource: Send + Sync {
    fn provider(&self) -> &str;
    async fn create_connection(&self) -> Result<Arc<dyn Connection>>;
    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command>;
    fn create_repository(&self, connection: Option<Arc<dyn Connection>>)
        -> Arc<dyn ICRUDRepository>;
    fn create_readonly_repository(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn IReadonlyRepository> {
        as_readonly(self.create_repository(connection))
    }
}

pub struct BuiltDatasource {
    provider: String,
    driver: Arc<dyn Driver>,
    options: Option<HashMap<String, String>>,
    connection: Mutex<Option<Arc<dyn Connection>>>,
}

impl BuiltDatasource {
    pub fn new(
        provider: impl Into<String>,
        driver: Arc<dyn Driver>,
        options: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            provider: provider.into(),
            driver,
            options,
            connection: Mutex::new(None),
        }
    }

    fn default_connection(&self) -> Arc<dyn Connection> {
        let mut slot = self.connection.lock().expect("connection");
        slot.get_or_insert_with(|| self.driver.connection(self.options.as_ref()))
            .clone()
    }
}

#[async_trait]
impl Datasource for BuiltDatasource {
    fn provider(&self) -> &str {
        &self.provider
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.default_connection())
    }

    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        self.driver
            .command(connection.unwrap_or_else(|| self.default_connection()))
    }

    fn create_repository(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn ICRUDRepository> {
        let connection = connection.unwrap_or_else(|| self.default_connection());
        let mut options = RepositoryBuilderOptions::with_factory(connection);
        options.types = self.driver.row_type_converters();
        Arc::new(CRUDRepository::new(options).expect("repository"))
    }
}
