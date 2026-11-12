use crate::command::MemoryCommand;
use crate::engine::{MemoryData, MemoryEngine};
use crate::transaction::MemoryTransaction;
use async_trait::async_trait;
use datasources::{
    Command, Connection, DatasourceError, IsolationLevel, Result, Row, Transaction,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) struct MemoryState {
    pub(crate) engine: Arc<MemoryEngine>,
    pub(crate) opened: Mutex<bool>,
    pub(crate) data: Option<MemoryData>,
}

pub struct MemoryConnection {
    pub(crate) state: Arc<MemoryState>,
}

impl MemoryConnection {
    pub fn new(
        collection: impl Into<String>,
        data: Option<MemoryData>,
    ) -> Self {
        Self::from_engine(Arc::new(MemoryEngine::new(collection)), data)
    }

    pub fn from_engine(engine: Arc<MemoryEngine>, data: Option<MemoryData>) -> Self {
        Self {
            state: Arc::new(MemoryState {
                engine,
                opened: Mutex::new(false),
                data,
            }),
        }
    }

    pub fn from_options(_: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        Arc::new(MemoryConnection::new("", None))
    }

    pub fn collection_name(&self) -> String {
        self.state.engine.collection_name()
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.state.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }

    pub fn collection<R>(&self, f: impl FnOnce(&mut Vec<Row>) -> R) -> R {
        self.state.engine.collection(&self.collection_name(), f)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn finish_commit(&self) {
        self.state.engine.commit();
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn finish_abort(&self) {
        self.state.engine.abort();
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn in_transaction(&self) -> bool {
        self.state.engine.in_transaction()
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
            self.state.engine.seed(data, &self.collection_name());
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
        self.state.engine.begin();
        Ok(Arc::new(MemoryTransaction {
            state: self.state.clone(),
            isolation: isolation.unwrap_or(IsolationLevel::ReadCommitted),
        }) as Arc<dyn Transaction>)
    }
}
