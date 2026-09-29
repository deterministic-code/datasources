use crate::datasource_error::DatasourceError;
use crate::isolation_level::IsolationLevel;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct MockTransaction {
    isolation: Mutex<IsolationLevel>,
    commit_calls: Mutex<usize>,
    abort_calls: Mutex<usize>,
    commit_error: Mutex<Option<String>>,
    abort_error: Mutex<Option<String>>,
    committed: Mutex<bool>,
    aborted: Mutex<bool>,
}

impl MockTransaction {
    pub fn new_unbound() -> Self {
        Self {
            isolation: Mutex::new(IsolationLevel::ReadCommitted),
            commit_calls: Mutex::new(0),
            abort_calls: Mutex::new(0),
            commit_error: Mutex::new(None),
            abort_error: Mutex::new(None),
            committed: Mutex::new(false),
            aborted: Mutex::new(false),
        }
    }

    pub fn set_isolation(&self, isolation: IsolationLevel) {
        *self.isolation.lock().expect("mock") = isolation;
    }

    pub fn set_commit_error(&self, message: impl Into<String>) {
        *self.commit_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_abort_error(&self, message: impl Into<String>) {
        *self.abort_error.lock().expect("mock") = Some(message.into());
    }

    pub fn commit_calls(&self) -> usize {
        *self.commit_calls.lock().expect("mock")
    }

    pub fn abort_calls(&self) -> usize {
        *self.abort_calls.lock().expect("mock")
    }

    pub fn committed(&self) -> bool {
        *self.committed.lock().expect("mock")
    }

    pub fn aborted(&self) -> bool {
        *self.aborted.lock().expect("mock")
    }
}

#[async_trait]
impl Transaction for MockTransaction {
    fn isolation(&self) -> IsolationLevel {
        *self.isolation.lock().expect("mock")
    }

    async fn commit(&self) -> Result<()> {
        *self.commit_calls.lock().expect("mock") += 1;
        if let Some(error) = &*self.commit_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        *self.committed.lock().expect("mock") = true;
        Ok(())
    }

    async fn abort(&self) -> Result<()> {
        *self.abort_calls.lock().expect("mock") += 1;
        if let Some(error) = &*self.abort_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        *self.aborted.lock().expect("mock") = true;
        Ok(())
    }
}
