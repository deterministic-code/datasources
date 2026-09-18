use crate::command::JsonCommand;
use crate::connection::JsonConnection;
use crate::file_store::{FileStore, MemoryFileStore};
use crate::repository::JsonRepository;
use async_trait::async_trait;
use datasources::{Command, Connection, Datasource, DatasourceFactory, CRUDRepository, Result};
use serde_json::Value as JsonValue;
use std::sync::Arc;

pub struct JsonDatasource {
    path: String,
    collection: Option<String>,
    store: Arc<dyn FileStore>,
    initial: Option<JsonValue>,
}

impl JsonDatasource {
    pub fn instance() -> Arc<Self> {
        Arc::new(Self::new(
            "",
            None,
            Arc::new(MemoryFileStore::new()),
            Some(JsonValue::Array(vec![])),
        ))
    }

    pub fn new(
        path: impl Into<String>,
        collection: Option<String>,
        store: Arc<dyn FileStore>,
        initial: Option<JsonValue>,
    ) -> Self {
        Self {
            path: path.into(),
            collection,
            store,
            initial,
        }
    }

    fn typed_connection(&self) -> Arc<JsonConnection> {
        Arc::new(JsonConnection::new(
            self.path.clone(),
            self.collection.clone(),
            self.store.clone(),
            self.initial.clone(),
        ))
    }
}

#[async_trait]
impl Datasource for JsonDatasource {
    fn provider(&self) -> &str {
        "json"
    }

    async fn create_connection(&self) -> Result<Arc<dyn Connection>> {
        Ok(self.typed_connection())
    }

    fn create_command(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command> {
        Arc::new(JsonCommand::new(self.typed_connection()))
    }

    fn create_repository(&self, _connection: Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> {
        Arc::new(JsonRepository::new(self.typed_connection()))
    }
}

impl DatasourceFactory for JsonDatasource {
    fn provider(&self) -> &str {
        "json"
    }

    fn create(
        &self,
        options: Option<&std::collections::HashMap<String, String>>,
    ) -> Arc<dyn Datasource> {
        Arc::new(JsonDatasource::new(
            options
                .and_then(|o| o.get("path"))
                .cloned()
                .unwrap_or_else(|| self.path.clone()),
            self.collection.clone(),
            self.store.clone(),
            self.initial.clone(),
        ))
    }
}
