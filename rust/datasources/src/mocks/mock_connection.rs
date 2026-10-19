use crate::command::Command;
use crate::connection::Connection;
use crate::datasource_error::DatasourceError;
use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use super::mock_command::MockCommand;
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
    state: Mutex<CallState>,
    transaction: Arc<dyn Transaction>,
    mock_transaction: Option<Arc<MockTransaction>>,
    command: Arc<dyn Command>,
    provider: String,
}

impl MockConnection {
    pub fn new() -> Arc<Self> {
        Self::with_command(Arc::new(MockCommand::new()))
    }

    pub fn with_command(command: Arc<dyn Command>) -> Arc<Self> {
        Self::with_command_provider(command, "mock")
    }

    pub fn with_command_provider(
        command: Arc<dyn Command>,
        provider: impl Into<String>,
    ) -> Arc<Self> {
        let mock_transaction = Arc::new(MockTransaction::new_unbound());
        Arc::new(Self {
            state: Mutex::new(CallState::default()),
            transaction: Arc::clone(&mock_transaction) as Arc<dyn Transaction>,
            mock_transaction: Some(mock_transaction),
            command,
            provider: provider.into(),
        })
    }

    pub fn with_transaction(transaction: Arc<dyn Transaction>) -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(CallState::default()),
            transaction,
            mock_transaction: None,
            command: Arc::new(MockCommand::new()),
            provider: "mock".into(),
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

    pub fn start_calls(&self) -> usize {
        self.state.lock().expect("mock").start_calls
    }

    pub fn is_open(&self) -> bool {
        self.state.lock().expect("mock").opened
    }

    pub fn last_isolation(&self) -> IsolationLevel {
        self.state.lock().expect("mock").last_isolation
    }
}

#[async_trait]
impl Connection for MockConnection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn provider(&self) -> &str {
        &self.provider
    }

    fn is_open(&self) -> bool {
        self.state.lock().expect("mock").opened
    }

    fn create_command(&self) -> Arc<dyn Command> {
        self.command.clone()
    }

    async fn open(&self) -> Result<()> {
        let mut state = self.state.lock().expect("mock");
        if state.opened {
            return Err(DatasourceError::message("Connection is already open"));
        }
        state.open_calls += 1;
        if let Some(error) = &state.open_error {
            return Err(DatasourceError::message(error.clone()));
        }
        state.opened = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        let mut state = self.state.lock().expect("mock");
        state.close_calls += 1;
        if let Some(error) = &state.close_error {
            return Err(DatasourceError::message(error.clone()));
        }
        state.opened = false;
        Ok(())
    }

    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        let mut state = self.state.lock().expect("mock");
        state.start_calls += 1;
        if let Some(error) = &state.start_error {
            return Err(DatasourceError::message(error.clone()));
        }
        let isolation = isolation.unwrap_or(IsolationLevel::ReadCommitted);
        state.last_isolation = isolation;
        if let Some(transaction) = &self.mock_transaction {
            transaction.set_isolation(isolation);
        }
        Ok(self.transaction.clone())
    }
}
