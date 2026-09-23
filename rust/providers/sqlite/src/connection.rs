use crate::command::SqliteCommand;
use crate::connection_string;
use crate::sqlite_path::sqlite_filename;
use crate::transaction::SqliteTransaction;
use async_trait::async_trait;
use datasources::{Command, Connection, DatasourceError, IsolationLevel, Result, Transaction};
use rusqlite::Connection as SqliteDb;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub(crate) fn map_err(error: rusqlite::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

pub struct SqliteConnection {
    pub filename: String,
    opened: Arc<Mutex<bool>>,
    pub(crate) db: Arc<Mutex<Option<SqliteDb>>>,
}

impl SqliteConnection {
    pub fn new(filename: Option<String>) -> Self {
        Self {
            filename: filename.unwrap_or_else(|| ":memory:".to_string()),
            opened: Arc::new(Mutex::new(false)),
            db: Arc::new(Mutex::new(None)),
        }
    }

    pub fn from_options(options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        let filename = options.and_then(|o| {
            o.get("filename").cloned().or_else(|| {
                o.get("url")
                    .or_else(|| o.get("database_url"))
                    .and_then(|url| sqlite_filename(url))
            })
        });
        Arc::new(SqliteConnection::new(filename))
    }

    pub(crate) fn share(&self) -> Self {
        Self {
            filename: self.filename.clone(),
            opened: self.opened.clone(),
            db: self.db.clone(),
        }
    }

    pub fn ensure_open(&self) -> Result<()> {
        if !*self.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is not open"));
        }
        Ok(())
    }

    pub(crate) fn exec(&self, sql: &str) -> Result<()> {
        let db = self.db.lock().unwrap();
        let conn = db
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        conn.execute(sql, []).map_err(map_err).map(|_| ())
    }

    fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        let db = self.db.lock().unwrap();
        let conn = db
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let flag = matches!(isolation, IsolationLevel::ReadUncommitted) as i64;
        conn.pragma_update(None, "read_uncommitted", flag)
            .map_err(map_err)?;
        conn.execute("BEGIN", []).map_err(map_err).map(|_| ())
    }
}

#[async_trait]
impl Connection for SqliteConnection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn provider(&self) -> &str {
        "sqlite"
    }

    fn is_open(&self) -> bool {
        *self.opened.lock().unwrap()
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(SqliteCommand::new(Arc::new(self.share())))
    }

    async fn open(&self) -> Result<()> {
        if *self.opened.lock().unwrap() {
            return Ok(());
        }
        let mut db = self.db.lock().unwrap();
        if db.is_some() {
            *self.opened.lock().unwrap() = true;
            return Ok(());
        }
        let conn = connection_string::open(&self.filename).map_err(map_err)?;
        conn.busy_timeout(Duration::ZERO).map_err(map_err)?;
        *db = Some(conn);
        drop(db);
        *self.opened.lock().unwrap() = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.db.lock().unwrap() = None;
        *self.opened.lock().unwrap() = false;
        Ok(())
    }

    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        self.ensure_open()?;
        let isolation = isolation.unwrap_or_default();
        self.begin(isolation)?;
        Ok(Arc::new(SqliteTransaction::new(self.share(), isolation)))
    }
}
