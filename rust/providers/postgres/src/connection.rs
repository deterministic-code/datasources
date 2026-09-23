use crate::command::PostgresCommand;
use crate::driver::PostgresDriver;
use crate::tokio_postgres_driver::TokioPostgresDriver;
use crate::transaction::PostgresTransaction;
use async_trait::async_trait;
use datasources::{
    Command, Connection, DatasourceError, IsolationLevel, Result, Transaction,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct PostgresConnection {
    pub url: String,
    pub schema: Option<String>,
    pub driver: Arc<dyn PostgresDriver>,
    opened: Arc<Mutex<bool>>,
}

impl PostgresConnection {
    pub fn new(
        url: Option<String>,
        schema: Option<String>,
        driver: Arc<dyn PostgresDriver>,
    ) -> Self {
        Self {
            url: url.unwrap_or_default(),
            schema,
            driver,
            opened: Arc::new(Mutex::new(false)),
        }
    }

    pub fn from_options(options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        let url = options.and_then(|o| o.get("url").or_else(|| o.get("database_url")).cloned());
        let schema = options.and_then(|o| {
            o.get("schema")
                .or_else(|| o.get("database_schema"))
                .cloned()
        });
        Arc::new(PostgresConnection::new(
            url,
            schema.clone(),
            Arc::new(TokioPostgresDriver::new(schema)),
        ))
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }
}

#[async_trait]
impl Connection for PostgresConnection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn provider(&self) -> &str {
        "postgres"
    }

    fn is_open(&self) -> bool {
        *self.opened.lock().unwrap()
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(PostgresCommand::new(Arc::new(PostgresConnection {
            url: self.url.clone(),
            schema: self.schema.clone(),
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
        Ok(Arc::new(PostgresTransaction {
            driver: self.driver.clone(),
            isolation,
        }) as Arc<dyn Transaction>)
    }
}
