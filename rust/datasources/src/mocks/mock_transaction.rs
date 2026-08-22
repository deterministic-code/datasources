use crate::datasource_error::DatasourceError;
use crate::middleware_aware::MiddlewareAware;
use crate::middleware_hook::MiddlewareHook;
use crate::middleware_host::MiddlewareHost;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct MockTransaction {
    host: MiddlewareHost,
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
            host: MiddlewareHost::new(),
            commit_calls: Mutex::new(0),
            abort_calls: Mutex::new(0),
            commit_error: Mutex::new(None),
            abort_error: Mutex::new(None),
            committed: Mutex::new(false),
            aborted: Mutex::new(false),
        }
    }

    pub fn set_commit_error(&self, message: impl Into<String>) {
        *self.commit_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_abort_error(&self, message: impl Into<String>) {
        *self.abort_error.lock().expect("mock") = Some(message.into());
    }

    pub fn committed(&self) -> bool {
        *self.committed.lock().expect("mock")
    }

    pub fn aborted(&self) -> bool {
        *self.aborted.lock().expect("mock")
    }
}

impl MiddlewareAware for MockTransaction {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Transaction for MockTransaction {
    async fn commit(&self) -> Result<()> {
        self.host
            .run("commit", None, || async {
                *self.commit_calls.lock().expect("mock") += 1;
                if let Some(error) = &*self.commit_error.lock().expect("mock") {
                    return Err(DatasourceError::message(error.clone()));
                }
                *self.committed.lock().expect("mock") = true;
                Ok(())
            })
            .await
    }

    async fn abort(&self) -> Result<()> {
        self.host
            .run("abort", None, || async {
                *self.abort_calls.lock().expect("mock") += 1;
                if let Some(error) = &*self.abort_error.lock().expect("mock") {
                    return Err(DatasourceError::message(error.clone()));
                }
                *self.aborted.lock().expect("mock") = true;
                Ok(())
            })
            .await
    }
}
