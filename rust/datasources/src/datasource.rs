use crate::command::Command;
use crate::connection::Connection;
use crate::repository::{as_readonly, CRUDRepository, ReadonlyRepository};
use crate::result::Result;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Datasource: Send + Sync {
    fn provider(&self) -> &str;
    async fn create_connection(&self) -> Result<Arc<dyn Connection>>;
    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command>;
    fn create_repository(&self, connection: Option<Arc<dyn Connection>>)
        -> Arc<dyn CRUDRepository>;
    fn create_readonly_repository(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn ReadonlyRepository> {
        as_readonly(self.create_repository(connection))
    }
}

pub struct BuiltDatasource {
    provider: String,
    new_connection: Arc<dyn Fn() -> Arc<dyn Connection> + Send + Sync>,
    new_repository: Arc<dyn Fn(Arc<dyn Connection>) -> Arc<dyn CRUDRepository> + Send + Sync>,
    connection: Mutex<Option<Arc<dyn Connection>>>,
}

impl BuiltDatasource {
    pub fn new(
        provider: impl Into<String>,
        new_connection: impl Fn() -> Arc<dyn Connection> + Send + Sync + 'static,
        new_repository: impl Fn(Arc<dyn Connection>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static,
    ) -> Self {
        Self {
            provider: provider.into(),
            new_connection: Arc::new(new_connection),
            new_repository: Arc::new(new_repository),
            connection: Mutex::new(None),
        }
    }

    fn default_connection(&self) -> Arc<dyn Connection> {
        let mut slot = self.connection.lock().expect("connection");
        slot.get_or_insert_with(|| (self.new_connection)()).clone()
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
        connection
            .unwrap_or_else(|| self.default_connection())
            .create_command()
    }

    fn create_repository(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn CRUDRepository> {
        (self.new_repository)(connection.unwrap_or_else(|| self.default_connection()))
    }
}
