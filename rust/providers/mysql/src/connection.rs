use crate::command::MysqlCommand;
use crate::engine::isolation_sql;
use crate::transaction::MysqlTransaction;
use async_trait::async_trait;
use datasources::{Command, Connection, DatasourceError, IsolationLevel, Result, Transaction};
use mysql_async::prelude::Queryable;
use mysql_async::Conn;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) fn map_err(error: mysql_async::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

pub struct MysqlConnection {
    pub url: String,
    pub(crate) opened: Arc<Mutex<bool>>,
    pub(crate) conn: Arc<tokio::sync::Mutex<Option<Conn>>>,
    pub(crate) ready: Arc<tokio::sync::Notify>,
}

impl MysqlConnection {
    pub fn new(url: Option<String>) -> Self {
        Self {
            url: url.unwrap_or_default(),
            opened: Arc::new(Mutex::new(false)),
            conn: Arc::new(tokio::sync::Mutex::new(None)),
            ready: Arc::new(tokio::sync::Notify::new()),
        }
    }

    pub fn from_options(options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        let url = options.and_then(|o| o.get("url").or_else(|| o.get("database_url")).cloned());
        Arc::new(MysqlConnection::new(url))
    }

    pub(crate) fn share(&self) -> Self {
        Self {
            url: self.url.clone(),
            opened: self.opened.clone(),
            conn: self.conn.clone(),
            ready: self.ready.clone(),
        }
    }

    pub(crate) async fn lock_conn(&self) -> Result<tokio::sync::MutexGuard<'_, Option<Conn>>> {
        loop {
            let notified = self.ready.notified();
            tokio::pin!(notified);
            let slot = self.conn.lock().await;
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
        let mut slot = self.lock_conn().await?;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        conn.query_drop(sql).await.map_err(map_err)
    }
}

#[async_trait]
impl Connection for MysqlConnection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn provider(&self) -> &str {
        "mysql"
    }

    fn is_open(&self) -> bool {
        *self.opened.lock().unwrap()
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(MysqlCommand::new(Arc::new(self.share())))
    }

    async fn open(&self) -> Result<()> {
        if *self.opened.lock().unwrap() {
            return Ok(());
        }
        let mut slot = self.conn.lock().await;
        if slot.is_some() {
            *self.opened.lock().unwrap() = true;
            return Ok(());
        }
        let mut conn = Conn::from_url(self.url.clone()).await.map_err(map_err)?;
        conn.query_drop("SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')")
            .await
            .map_err(map_err)?;
        *slot = Some(conn);
        drop(slot);
        *self.opened.lock().unwrap() = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.opened.lock().unwrap() = false;
        *self.conn.lock().await = None;
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
            "SET TRANSACTION ISOLATION LEVEL {}",
            isolation_sql(isolation)
        ))
        .await?;
        self.exec("START TRANSACTION").await?;
        Ok(Arc::new(MysqlTransaction::new(self.share(), isolation)))
    }
}
