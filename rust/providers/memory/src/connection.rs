use crate::command::MemoryCommand;
use crate::store::{MemoryData, MemoryStore};
use crate::transaction::MemoryTransaction;
use async_trait::async_trait;
use datasources::{
    Command, Connection, DatasourceError, IsolationLevel, Result, Row, Transaction, Value,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) struct MemoryState {
    pub(crate) store: Arc<MemoryStore>,
    pub(crate) collection: Mutex<String>,
    pub(crate) opened: Mutex<bool>,
    pub(crate) in_transaction: Mutex<bool>,
    pub(crate) snapshot: Mutex<Option<HashMap<String, Vec<Row>>>>,
    pub(crate) inserted: Mutex<HashMap<String, Value>>,
    pub(crate) data: Option<MemoryData>,
}

pub struct MemoryConnection {
    pub(crate) state: Arc<MemoryState>,
}

impl MemoryConnection {
    pub fn new(
        store: Arc<MemoryStore>,
        collection: impl Into<String>,
        data: Option<MemoryData>,
    ) -> Self {
        Self {
            state: Arc::new(MemoryState {
                store,
                collection: Mutex::new(collection.into()),
                opened: Mutex::new(false),
                in_transaction: Mutex::new(false),
                snapshot: Mutex::new(None),
                inserted: Mutex::new(HashMap::new()),
                data,
            }),
        }
    }

    pub fn from_options(_: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        Arc::new(MemoryConnection::new(
            Arc::new(MemoryStore::new()),
            "",
            None,
        ))
    }

    pub fn collection_name(&self) -> String {
        self.state.collection.lock().unwrap().clone()
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.state.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }

    pub fn collection<R>(&self, f: impl FnOnce(&mut Vec<Row>) -> R) -> R {
        self.state.store.collection(&self.collection_name(), f)
    }

    pub fn with_rows<R>(
        &self,
        table: Option<&str>,
        create: bool,
        f: impl FnOnce(&mut Vec<Row>) -> R,
    ) -> R {
        let named = table.filter(|name| !name.is_empty());
        let key = match named {
            Some(name) if create || self.state.store.has_collection(name) => name.to_string(),
            _ => self.collection_name(),
        };
        self.state.store.collection(&key, f)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn finish_commit(&self) {
        *self.state.in_transaction.lock().unwrap() = false;
        *self.state.snapshot.lock().unwrap() = None;
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn finish_abort(&self) {
        if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
            self.state.store.restore(snapshot);
        }
        *self.state.in_transaction.lock().unwrap() = false;
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn in_transaction(&self) -> bool {
        *self.state.in_transaction.lock().unwrap()
    }

    pub(crate) fn remember_inserted(&self, table: &str, id: Value) {
        self.state
            .inserted
            .lock()
            .expect("inserted")
            .insert(table.to_string(), id);
    }

    pub(crate) fn inserted(&self, table: &str) -> Value {
        self.state
            .inserted
            .lock()
            .expect("inserted")
            .get(table)
            .cloned()
            .unwrap_or(Value::Null)
    }
}

#[async_trait]
impl Connection for MemoryConnection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn provider(&self) -> &str {
        "memory"
    }

    fn is_open(&self) -> bool {
        *self.state.opened.lock().unwrap()
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(MemoryCommand::new(Arc::new(MemoryConnection {
            state: self.state.clone(),
        })))
    }

    async fn open(&self) -> Result<()> {
        if *self.state.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is already open"));
        }
        if let Some(data) = &self.state.data {
            self.state.store.seed(data, &self.collection_name());
        }
        self.collection(|_| {});
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
        *self.state.snapshot.lock().unwrap() = Some(self.state.store.snapshot());
        *self.state.in_transaction.lock().unwrap() = true;
        Ok(Arc::new(MemoryTransaction {
            state: self.state.clone(),
            isolation: isolation.unwrap_or(IsolationLevel::Serializable),
        }) as Arc<dyn Transaction>)
    }
}
