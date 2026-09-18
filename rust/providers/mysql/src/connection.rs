use crate::command::MysqlCommand;
use crate::driver::MysqlDriver;
use crate::transaction::MysqlTransaction;
use async_trait::async_trait;
use datasources::{Command, Connection, DatasourceError, IsolationLevel, Result, Transaction};
use std::sync::{Arc, Mutex};

pub struct MysqlConnection {
    pub url: String,
    pub driver: Arc<dyn MysqlDriver>,
    opened: Arc<Mutex<bool>>,
}

impl MysqlConnection {
    pub fn new(url: Option<String>, driver: Arc<dyn MysqlDriver>) -> Self {
        Self {
            url: url.unwrap_or_default(),
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
impl Connection for MysqlConnection {
    fn provider(&self) -> &str {
        "mysql"
    }

    fn is_open(&self) -> bool {
        *self.opened.lock().unwrap()
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(MysqlCommand::new(Arc::new(MysqlConnection {
            url: self.url.clone(),
            driver: self.driver.clone(),
            opened: self.opened.clone(),
        })))
    }

    async fn open(&self) -> Result<()> {
        if *self.opened.lock().unwrap() {
            return Ok(());
        }
        self.driver.open(&self.url).await?;
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
        Ok(Arc::new(MysqlTransaction {
            driver: self.driver.clone(),
            isolation,
        }) as Arc<dyn Transaction>)
    }
}
