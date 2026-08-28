use crate::driver::SqliteDriver;
use async_trait::async_trait;
use datasources::{
    IsolationLevel, MiddlewareAware, MiddlewareHook, MiddlewareHost, Result, Transaction,
};
use std::sync::Arc;

pub struct SqliteTransaction {
    pub(crate) host: MiddlewareHost,
    pub(crate) driver: Arc<dyn SqliteDriver>,
    pub(crate) isolation: IsolationLevel,
}

impl MiddlewareAware for SqliteTransaction {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Transaction for SqliteTransaction {
    fn isolation(&self) -> IsolationLevel {
        self.isolation
    }

    async fn commit(&self) -> Result<()> {
        let driver = self.driver.clone();
        self.host
            .run("commit", None, move || async move { driver.commit().await })
            .await
    }

    async fn abort(&self) -> Result<()> {
        let driver = self.driver.clone();
        self.host
            .run("abort", None, move || async move { driver.rollback().await })
            .await
    }
}
