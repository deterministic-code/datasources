use crate::file_store::FileStore;
use crate::transaction::JsonTransaction;
use async_trait::async_trait;
use datasources::{
    Connection, DatasourceError, MiddlewareAware, MiddlewareHook, MiddlewareHost, Result,
    Transaction,
};
use serde_json::{Map, Value as JsonValue};
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
    host: MiddlewareHost,
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
            host: MiddlewareHost::new(),
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

    pub fn path(&self) -> &str {
        &self.state.path
    }

    pub fn set_collection(&self, name: impl Into<String>) {
        *self.state.collection.lock().unwrap() = Some(name.into());
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

impl MiddlewareAware for JsonConnection {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Connection for JsonConnection {
    async fn open(&self) -> Result<()> {
        self.host
            .run("open", None, || async {
                if *self.state.opened.lock().unwrap() {
                    return Ok(());
                }
                let collection = self.state.collection.lock().unwrap().clone();
                let next = if let Some(initial) = &self.state.initial {
                    initial.clone()
                } else if !self.state.path.is_empty() && self.state.store.exists(&self.state.path) {
                    let contents = self.state.store.read(&self.state.path)?;
                    serde_json::from_str(&contents)
                        .map_err(|e| DatasourceError::message(e.to_string()))?
                } else {
                    JsonConnection::empty_data(&collection)
                };
                *self.state.data.lock().unwrap() = next;
                if collection.is_some() {
                    self.collection_mut(|_| {});
                }
                *self.state.opened.lock().unwrap() = true;
                Ok(())
            })
            .await
    }

    async fn close(&self) -> Result<()> {
        self.host
            .run("close", None, || async {
                *self.state.opened.lock().unwrap() = false;
                Ok(())
            })
            .await
    }

    async fn start_transaction(&self) -> Result<Arc<dyn Transaction>> {
        self.host
            .run("startTransaction", None, || async {
                self.ensure_open()?;
                *self.state.snapshot.lock().unwrap() =
                    Some(self.state.data.lock().unwrap().clone());
                *self.state.in_transaction.lock().unwrap() = true;
                Ok(Arc::new(JsonTransaction {
                    host: MiddlewareHost::new(),
                    state: self.state.clone(),
                }) as Arc<dyn Transaction>)
            })
            .await
    }
}
