use crate::driver::SqliteDriver;
use crate::transaction::SqliteTransaction;
use async_trait::async_trait;
use datasources::{
    Connection, DatasourceError, MiddlewareAware, MiddlewareHook, MiddlewareHost, Result,
    Transaction,
};
use std::sync::{Arc, Mutex};

pub struct SqliteConnection {
    host: MiddlewareHost,
    pub filename: String,
    pub driver: Arc<dyn SqliteDriver>,
    opened: Mutex<bool>,
}

impl SqliteConnection {
    pub fn new(filename: Option<String>, driver: Arc<dyn SqliteDriver>) -> Self {
        Self {
            host: MiddlewareHost::new(),
            filename: filename.unwrap_or_else(|| ":memory:".to_string()),
            driver,
            opened: Mutex::new(false),
        }
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }
}

impl MiddlewareAware for SqliteConnection {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Connection for SqliteConnection {
    async fn open(&self) -> Result<()> {
        self.host
            .run("open", None, || async {
                if *self.opened.lock().unwrap() {
                    return Ok(());
                }
                self.driver.open(&self.filename).await?;
                *self.opened.lock().unwrap() = true;
                Ok(())
            })
            .await
    }

    async fn close(&self) -> Result<()> {
        self.host
            .run("close", None, || async {
                self.driver.close().await?;
                *self.opened.lock().unwrap() = false;
                Ok(())
            })
            .await
    }

    async fn start_transaction(&self) -> Result<Arc<dyn Transaction>> {
        self.host
            .run("startTransaction", None, || async {
                self.ensure_open()?;
                self.driver.begin().await?;
                Ok(Arc::new(SqliteTransaction {
                    host: MiddlewareHost::new(),
                    driver: self.driver.clone(),
                }) as Arc<dyn Transaction>)
            })
            .await
    }
}
