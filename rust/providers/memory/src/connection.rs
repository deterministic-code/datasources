use crate::store::{MemoryData, MemoryStore};
use crate::transaction::MemoryTransaction;
use async_trait::async_trait;
use datasources::{
    Connection, DatasourceError, MiddlewareAware, MiddlewareHook, MiddlewareHost, Record, Result,
    Transaction,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) struct MemoryState {
    pub(crate) store: Arc<MemoryStore>,
    pub(crate) collection: Mutex<String>,
    pub(crate) opened: Mutex<bool>,
    pub(crate) in_transaction: Mutex<bool>,
    pub(crate) snapshot: Mutex<Option<HashMap<String, Vec<Record>>>>,
    pub(crate) data: Option<MemoryData>,
}

pub struct MemoryConnection {
    host: MiddlewareHost,
    pub(crate) state: Arc<MemoryState>,
}

impl MemoryConnection {
    pub fn new(
        store: Arc<MemoryStore>,
        collection: impl Into<String>,
        data: Option<MemoryData>,
    ) -> Self {
        Self {
            host: MiddlewareHost::new(),
            state: Arc::new(MemoryState {
                store,
                collection: Mutex::new(collection.into()),
                opened: Mutex::new(false),
                in_transaction: Mutex::new(false),
                snapshot: Mutex::new(None),
                data,
            }),
        }
    }

    pub fn set_collection(&self, name: impl Into<String>) {
        *self.state.collection.lock().unwrap() = name.into();
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

    pub fn collection<R>(&self, f: impl FnOnce(&mut Vec<Record>) -> R) -> R {
        self.state.store.collection(&self.collection_name(), f)
    }

    pub fn finish_commit(&self) {
        *self.state.in_transaction.lock().unwrap() = false;
        *self.state.snapshot.lock().unwrap() = None;
    }

    pub fn finish_abort(&self) {
        if let Some(snapshot) = self.state.snapshot.lock().unwrap().take() {
            self.state.store.restore(snapshot);
        }
        *self.state.in_transaction.lock().unwrap() = false;
    }

    pub fn in_transaction(&self) -> bool {
        *self.state.in_transaction.lock().unwrap()
    }
}

impl MiddlewareAware for MemoryConnection {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Connection for MemoryConnection {
    async fn open(&self) -> Result<()> {
        self.host
            .run("open", None, || async {
                if *self.state.opened.lock().unwrap() {
                    return Ok(());
                }
                if let Some(data) = &self.state.data {
                    self.state.store.seed(data, &self.collection_name());
                }
                self.collection(|_| {});
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
                *self.state.snapshot.lock().unwrap() = Some(self.state.store.snapshot());
                *self.state.in_transaction.lock().unwrap() = true;
                Ok(Arc::new(MemoryTransaction {
                    host: MiddlewareHost::new(),
                    state: self.state.clone(),
                }) as Arc<dyn Transaction>)
            })
            .await
    }
}
