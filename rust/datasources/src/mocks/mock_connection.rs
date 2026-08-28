use crate::connection::Connection;
use crate::datasource_error::DatasourceError;
use crate::isolation_level::IsolationLevel;
use crate::middleware_aware::MiddlewareAware;
use crate::middleware_hook::MiddlewareHook;
use crate::middleware_host::MiddlewareHost;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use super::mock_transaction::MockTransaction;

#[derive(Default)]
struct CallState {
    open_calls: usize,
    close_calls: usize,
    start_calls: usize,
    open_error: Option<String>,
    close_error: Option<String>,
    start_error: Option<String>,
    opened: bool,
    last_isolation: IsolationLevel,
}

pub struct MockConnection {
    host: MiddlewareHost,
    state: Mutex<CallState>,
    transaction: Arc<dyn Transaction>,
}

impl MockConnection {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            host: MiddlewareHost::new(),
            state: Mutex::new(CallState::default()),
            transaction: Arc::new(MockTransaction::new_unbound()),
        })
    }

    pub fn with_transaction(transaction: Arc<dyn Transaction>) -> Arc<Self> {
        Arc::new(Self {
            host: MiddlewareHost::new(),
            state: Mutex::new(CallState::default()),
            transaction,
        })
    }

    pub fn set_open_error(&self, message: impl Into<String>) {
        self.state.lock().expect("mock").open_error = Some(message.into());
    }

    pub fn set_close_error(&self, message: impl Into<String>) {
        self.state.lock().expect("mock").close_error = Some(message.into());
    }

    pub fn set_start_error(&self, message: impl Into<String>) {
        self.state.lock().expect("mock").start_error = Some(message.into());
    }

    pub fn open_calls(&self) -> usize {
        self.state.lock().expect("mock").open_calls
    }

    pub fn opened(&self) -> bool {
        self.state.lock().expect("mock").opened
    }

    pub fn last_isolation(&self) -> IsolationLevel {
        self.state.lock().expect("mock").last_isolation
    }
}

impl MiddlewareAware for MockConnection {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Connection for MockConnection {
    async fn open(&self) -> Result<()> {
        self.host
            .run("open", None, || async {
                let mut state = self.state.lock().expect("mock");
                state.open_calls += 1;
                if let Some(error) = &state.open_error {
                    return Err(DatasourceError::message(error.clone()));
                }
                state.opened = true;
                Ok(())
            })
            .await
    }

    async fn close(&self) -> Result<()> {
        self.host
            .run("close", None, || async {
                let mut state = self.state.lock().expect("mock");
                state.close_calls += 1;
                if let Some(error) = &state.close_error {
                    return Err(DatasourceError::message(error.clone()));
                }
                state.opened = false;
                Ok(())
            })
            .await
    }

    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        self.host
            .run("startTransaction", None, || async {
                let mut state = self.state.lock().expect("mock");
                state.start_calls += 1;
                if let Some(error) = &state.start_error {
                    return Err(DatasourceError::message(error.clone()));
                }
                state.last_isolation = isolation.unwrap_or_default();
                Ok(self.transaction.clone())
            })
            .await
    }
}
