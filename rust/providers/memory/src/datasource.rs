use crate::command::MemoryCommand;
use crate::connection::MemoryConnection;
use crate::repository::MemoryRepository;
use crate::store::{MemoryData, MemoryStore};
use async_trait::async_trait;
use datasources::{
    Command, Connection, Datasource, DatasourceFactory, Repository, Result,
};
use std::sync::Arc;

pub struct MemoryDatasource {
    store: Arc<MemoryStore>,
    collection: Option<String>,
    data: Option<MemoryData>,
}

impl MemoryDatasource {
    pub fn instance() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn new(
        store: Arc<MemoryStore>,
        collection: Option<String>,
        data: Option<MemoryData>,
    ) -> Self {
        Self {
            store,
            collection,
            data,
        }
    }

    fn typed_connection(&self) -> Arc<MemoryConnection> {
        Arc::new(MemoryConnection::new(
            self.store.clone(),
            self.collection.clone().unwrap_or_default(),
            self.data.clone(),
        ))
    }
}

impl Default for MemoryDatasource {
    fn default() -> Self {
        Self::new(Arc::new(MemoryStore::new()), None, None)
    }
}

#[async_trait]
impl Datasource for MemoryDatasource {
    fn provider(&self) -> &str {
        "memory"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.typed_connection())
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(MemoryCommand::new(self.typed_connection()))
    }

    fn create_repository(
        &self,
        _connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository> {
        Arc::new(MemoryRepository::new(self.typed_connection()))
    }
}

impl DatasourceFactory for MemoryDatasource {
    fn provider(&self) -> &str {
        "memory"
    }

    fn create(&self, _options: Option<&std::collections::HashMap<String, String>>) -> Arc<dyn Datasource> {
        Arc::new(MemoryDatasource::new(
            self.store.clone(),
            self.collection.clone(),
            self.data.clone(),
        ))
    }
}
