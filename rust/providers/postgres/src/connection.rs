use crate::bind::isolation_sql;
use crate::command::PostgresCommand;
use crate::transaction::PostgresTransaction;
use async_trait::async_trait;
use datasources::{Command, Connection, DatasourceError, IsolationLevel, Result, Transaction};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;

pub(crate) fn map_err(error: tokio_postgres::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

pub struct PostgresConnection {
    pub url: String,
    pub schema: Option<String>,
    opened: Arc<Mutex<bool>>,
    pub(crate) client: Arc<tokio::sync::Mutex<Option<tokio_postgres::Client>>>,
}

impl PostgresConnection {
    pub fn new(url: Option<String>, schema: Option<String>) -> Self {
        Self {
            url: url.unwrap_or_default(),
            schema,
            opened: Arc::new(Mutex::new(false)),
            client: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    pub fn from_options(options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        let url = options.and_then(|o| o.get("url").or_else(|| o.get("database_url")).cloned());
        let schema = options.and_then(|o| {
            o.get("schema")
                .or_else(|| o.get("database_schema"))
                .cloned()
        });
        Arc::new(PostgresConnection::new(url, schema))
    }

    pub(crate) fn share(&self) -> Self {
        Self {
            url: self.url.clone(),
            schema: self.schema.clone(),
            opened: self.opened.clone(),
            client: self.client.clone(),
        }
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }

    pub(crate) async fn exec(&self, sql: &str) -> Result<()> {
        let slot = self.client.lock().await;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        client.batch_execute(sql).await.map_err(map_err)
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
        Arc::new(PostgresCommand::new(Arc::new(self.share())))
    }

    async fn open(&self) -> Result<()> {
        if *self.opened.lock().unwrap() {
            return Ok(());
        }
        let mut slot = self.client.lock().await;
        if slot.is_some() {
            *self.opened.lock().unwrap() = true;
            return Ok(());
        }
        let (client, connection) = tokio_postgres::connect(&self.url, NoTls)
            .await
            .map_err(map_err)?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        if let Some(schema) = &self.schema {
            client
                .batch_execute(&format!("SET search_path TO {schema}"))
                .await
                .map_err(map_err)?;
        }
        *slot = Some(client);
        drop(slot);
        *self.opened.lock().unwrap() = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.client.lock().await = None;
        *self.opened.lock().unwrap() = false;
        Ok(())
    }

    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        self.ensure_open()?;
        let isolation = isolation.unwrap_or_default();
        self.exec(&format!(
            "BEGIN ISOLATION LEVEL {}",
            isolation_sql(isolation)
        ))
        .await?;
        Ok(Arc::new(PostgresTransaction::new(self.share(), isolation)))
    }
}
