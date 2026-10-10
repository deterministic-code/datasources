use crate::bind::isolation_sql;
use crate::command::PostgresCommand;
use crate::transaction::PostgresTransaction;
use async_trait::async_trait;
use datasources::{
    Command, Connection, DatasourceError, DatasourceUrlParser, IsolationLevel, Result, Transaction,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;

pub(crate) fn map_err(error: tokio_postgres::Error) -> DatasourceError {
    let mut text = error.to_string();
    let mut current = std::error::Error::source(&error);
    while let Some(next) = current {
        text.push_str(": ");
        text.push_str(&next.to_string());
        current = next.source();
    }
    DatasourceError::message(text)
}

pub struct PostgresConnection {
    pub url: String,
    pub schema: Option<String>,
    pub(crate) opened: Arc<Mutex<bool>>,
    pub(crate) client: Arc<tokio::sync::Mutex<Option<tokio_postgres::Client>>>,
    pub(crate) ready: Arc<tokio::sync::Notify>,
}

impl PostgresConnection {
    pub fn new(url: Option<String>, schema: Option<String>) -> Self {
        Self {
            url: url.unwrap_or_default(),
            schema,
            opened: Arc::new(Mutex::new(false)),
            client: Arc::new(tokio::sync::Mutex::new(None)),
            ready: Arc::new(tokio::sync::Notify::new()),
        }
    }

    pub fn from_options(options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        let url = options.and_then(|o| o.get("DATASOURCE_URL").cloned());
        let schema = url
            .as_deref()
            .and_then(DatasourceUrlParser::parse)
            .and_then(|parsed| parsed.schema);
        Arc::new(PostgresConnection::new(url, schema))
    }

    pub(crate) fn share(&self) -> Self {
        Self {
            url: self.url.clone(),
            schema: self.schema.clone(),
            opened: self.opened.clone(),
            client: self.client.clone(),
            ready: self.ready.clone(),
        }
    }

    pub(crate) async fn lock_client(
        &self,
    ) -> Result<tokio::sync::MutexGuard<'_, Option<tokio_postgres::Client>>> {
        loop {
            let notified = self.ready.notified();
            tokio::pin!(notified);
            let slot = self.client.lock().await;
            if slot.is_some() {
                return Ok(slot);
            }
            let open = *self.opened.lock().unwrap();
            drop(slot);
            if !open {
                return Err(DatasourceError::message("Connection is not open"));
            }
            notified.await;
        }
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }

    pub(crate) async fn exec(&self, sql: &str) -> Result<()> {
        let slot = self.lock_client().await?;
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
            return Err(DatasourceError::message("Connection is already open"));
        }
        let mut slot = self.client.lock().await;
        if slot.is_some() {
            *self.opened.lock().unwrap() = true;
            return Ok(());
        }
        let (client, connection) = tokio_postgres::connect(&connect_url(&self.url), NoTls)
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
        *self.opened.lock().unwrap() = false;
        *self.client.lock().await = None;
        self.ready.notify_waiters();
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

pub(crate) fn connect_url(url: &str) -> String {
    let Some(parsed) = DatasourceUrlParser::parse(url) else {
        return url.to_string();
    };
    if parsed.schema.is_none() {
        return url.to_string();
    }
    let mut out = format!("{}://", parsed.provider);
    match (&parsed.username, &parsed.password) {
        (Some(user), Some(password)) => out.push_str(&format!("{user}:{password}@")),
        (Some(user), None) => out.push_str(&format!("{user}@")),
        _ => {}
    }
    if let Some(host) = &parsed.host {
        out.push_str(host);
    }
    if let Some(port) = parsed.port {
        out.push_str(&format!(":{port}"));
    }
    out.push_str(&parsed.path);
    if !parsed.options.is_empty() {
        let query = parsed
            .options
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");
        out.push('?');
        out.push_str(&query);
    }
    out
}
