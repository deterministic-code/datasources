use crate::command::SqliteCommand;
use crate::driver::SqliteDriver;
use crate::transaction::SqliteTransaction;
use async_trait::async_trait;
use datasources::{
    Command, Connection, DatasourceError, IsolationLevel, Result, Transaction,
};
use std::sync::{Arc, Mutex};

pub struct SqliteConnection {
    pub filename: String,
    pub driver: Arc<dyn SqliteDriver>,
    opened: Arc<Mutex<bool>>,
}

impl SqliteConnection {
    pub fn new(filename: Option<String>, driver: Arc<dyn SqliteDriver>) -> Self {
        Self {
            filename: filename.unwrap_or_else(|| ":memory:".to_string()),
            driver,
            opened: Arc::new(Mutex::new(false)),
        }
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }
}

#[async_trait]
impl Connection for SqliteConnection {
    fn provider(&self) -> &str {
        "sqlite"
    }

    fn opened(&self) -> bool {
        *self.opened.lock().unwrap()
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(SqliteCommand::new(Arc::new(SqliteConnection {
            filename: self.filename.clone(),
            driver: self.driver.clone(),
            opened: self.opened.clone(),
        })))
    }

    async fn open(&self) -> Result<()> {
        if *self.opened.lock().unwrap() {
            return Ok(());
        }
        self.driver.open(&self.filename).await?;
        *self.opened.lock().unwrap() = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        self.driver.close().await?;
        *self.opened.lock().unwrap() = false;
        Ok(())
    }

    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        self.ensure_open()?;
        let isolation = isolation.unwrap_or_default();
        self.driver.begin(isolation).await?;
        Ok(Arc::new(SqliteTransaction {
            driver: self.driver.clone(),
            isolation,
        }) as Arc<dyn Transaction>)
    }
}
