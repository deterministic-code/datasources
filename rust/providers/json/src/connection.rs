use crate::command::JsonCommand;
use crate::file_store::{FileStore, MemoryFileStore};
use crate::transaction::JsonTransaction;
use async_trait::async_trait;
use datasources::{
    Command, Connection, DatasourceError, IsolationLevel, Result, Transaction,
};
use serde_json::{Map, Value as JsonValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) struct JsonState {
    pub(crate) path: String,
    pub(crate) collection: Mutex<Option<String>>,
    pub(crate) store: Arc<dyn FileStore>,
    pub(crate) data: Mutex<JsonValue>,
    pub(crate) opened: Mutex<bool>,
    pub(crate) in_transaction: Mutex<bool>,
    pub(crate) snapshot: Mutex<Option<JsonValue>>,
    pub(crate) initial: Option<JsonValue>,
}

pub struct JsonConnection {
    pub(crate) state: Arc<JsonState>,
}

impl JsonConnection {
    pub fn new(
        path: impl Into<String>,
        collection: Option<String>,
        store: Arc<dyn FileStore>,
        initial: Option<JsonValue>,
    ) -> Self {
        Self {
            state: Arc::new(JsonState {
                path: path.into(),
                collection: Mutex::new(collection),
                store,
                data: Mutex::new(JsonValue::Array(vec![])),
                opened: Mutex::new(false),
                in_transaction: Mutex::new(false),
                snapshot: Mutex::new(None),
                initial,
            }),
        }
    }

    pub fn from_options(options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        let path = options
            .and_then(|o| o.get("path"))
            .cloned()
            .unwrap_or_default();
        Arc::new(JsonConnection::new(
            path,
            None,
            Arc::new(MemoryFileStore::new()),
            None,
        ))
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn path(&self) -> &str {
        &self.state.path
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.state.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }

    fn empty_data(collection: &Option<String>) -> JsonValue {
        if collection.is_some() {
            JsonValue::Object(Map::new())
        } else {
            JsonValue::Array(vec![])
        }
    }

    pub fn collection_mut<R>(&self, f: impl FnOnce(&mut Vec<JsonValue>) -> R) -> R {
        let collection = self.state.collection.lock().unwrap().clone();
        let mut data = self.state.data.lock().unwrap();
        if let Some(name) = collection {
            if !data.is_object() {
                *data = JsonValue::Object(Map::new());
            }
            let obj = data.as_object_mut().unwrap();
            if !obj.get(&name).map(JsonValue::is_array).unwrap_or(false) {
                obj.insert(name.clone(), JsonValue::Array(vec![]));
            }
            f(obj.get_mut(&name).unwrap().as_array_mut().unwrap())
        } else {
            if !data.is_array() {
                *data = JsonValue::Array(vec![]);
            }
            f(data.as_array_mut().unwrap())
        }
    }

    pub fn persist(&self) -> Result<()> {
        if *self.state.in_transaction.lock().unwrap() || self.state.path.is_empty() {
            return Ok(());
        }
        let data = self.state.data.lock().unwrap().to_string();
        self.state.store.write(&self.state.path, &data)
    }
}

#[async_trait]
impl Connection for JsonConnection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn provider(&self) -> &str {
        "json"
    }

    fn is_open(&self) -> bool {
        *self.state.opened.lock().unwrap()
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(JsonCommand::new(Arc::new(JsonConnection {
            state: self.state.clone(),
        })))
    }

    async fn open(&self) -> Result<()> {
        if *self.state.opened.lock().unwrap() {
            return Ok(());
        }
        let collection = self.state.collection.lock().unwrap().clone();
        let next = if let Some(initial) = &self.state.initial {
            initial.clone()
        } else if !self.state.path.is_empty() && self.state.store.exists(&self.state.path) {
            let contents = self.state.store.read(&self.state.path)?;
            serde_json::from_str(&contents).map_err(|e| DatasourceError::message(e.to_string()))?
        } else {
            JsonConnection::empty_data(&collection)
        };
        *self.state.data.lock().unwrap() = next;
        if collection.is_some() {
            self.collection_mut(|_| {});
        }
        *self.state.opened.lock().unwrap() = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.state.opened.lock().unwrap() = false;
        Ok(())
    }

    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        self.ensure_open()?;
        *self.state.snapshot.lock().unwrap() = Some(self.state.data.lock().unwrap().clone());
        *self.state.in_transaction.lock().unwrap() = true;
        Ok(Arc::new(JsonTransaction {
            state: self.state.clone(),
            isolation: isolation.unwrap_or_default(),
        }) as Arc<dyn Transaction>)
    }
}
