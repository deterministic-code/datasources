use crate::command::SqliteCommand;
use crate::connection_string;
use crate::pool::{configure, SqlitePool};
use crate::sqlite_path::{sqlite_path, SqliteOptions};
use crate::transaction::SqliteTransaction;
use async_trait::async_trait;
use datasources::{Command, Connection, DatasourceError, IsolationLevel, Result, Transaction};
use rusqlite::Connection as SqliteDb;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) fn map_err(error: rusqlite::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

pub struct SqliteConnection {
    pub filename: String,
    pub(crate) opened: Arc<Mutex<bool>>,
    pub(crate) db: Arc<Mutex<Option<SqliteDb>>>,
    pub(crate) pool: Arc<Mutex<Option<Arc<SqlitePool>>>>,
    pub(crate) pinned: bool,
    options: SqliteOptions,
    open_error: Option<String>,
}

impl SqliteConnection {
    pub fn new(filename: Option<String>) -> Self {
        Self::with_options(filename, SqliteOptions::default())
    }

    pub fn with_options(filename: Option<String>, options: SqliteOptions) -> Self {
        Self {
            filename: filename.unwrap_or_else(|| ":memory:".to_string()),
            opened: Arc::new(Mutex::new(false)),
            db: Arc::new(Mutex::new(None)),
            pool: Arc::new(Mutex::new(None)),
            pinned: false,
            options,
            open_error: None,
        }
    }

    pub fn from_options(options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        let url = options.and_then(|o| o.get("DATASOURCE_URL").map(String::as_str));
        Arc::new(match url.and_then(sqlite_path) {
            None => SqliteConnection::new(None),
            Some((filename, Ok(parsed))) => SqliteConnection::with_options(Some(filename), parsed),
            Some((filename, Err(error))) => SqliteConnection {
                open_error: Some(error),
                ..SqliteConnection::with_options(Some(filename), SqliteOptions::default())
            },
        })
    }

    pub(crate) fn share(&self) -> Self {
        Self {
            filename: self.filename.clone(),
            opened: self.opened.clone(),
            db: self.db.clone(),
            pool: self.pool.clone(),
            pinned: false,
            options: self.options.clone(),
            open_error: self.open_error.clone(),
        }
    }

    pub(crate) fn share_pinned(&self) -> Self {
        Self {
            pinned: true,
            ..self.share()
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

    pub(crate) fn pool(&self) -> Option<Arc<SqlitePool>> {
        self.pool.lock().unwrap().clone()
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
        let connection = if self.pinned {
            self.share_pinned()
        } else {
            self.share()
        };
        Arc::new(SqliteCommand::new(Arc::new(connection)))
    }

    async fn open(&self) -> Result<()> {
        if let Some(error) = &self.open_error {
            return Err(DatasourceError::message(error.clone()));
        }
        if *self.opened.lock().unwrap() {
            return Err(DatasourceError::message("Connection is already open"));
        }
        let mut db = self.db.lock().unwrap();
        if db.is_some() {
            *self.opened.lock().unwrap() = true;
            return Ok(());
        }
        let resolved = self.options.resolved(&self.filename);
        let conn = connection_string::open(&self.filename).map_err(map_err)?;
        configure(&conn, &resolved)?;
        *db = Some(conn);
        drop(db);
        if resolved.pool_size > 0 {
            *self.pool.lock().unwrap() = Some(Arc::new(SqlitePool::new(
                self.filename.clone(),
                resolved,
            )));
        }
        *self.opened.lock().unwrap() = true;
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.opened.lock().unwrap() = false;
        if let Some(pool) = self.pool.lock().unwrap().take() {
            pool.close();
        }
        *self.db.lock().unwrap() = None;
        Ok(())
    }

    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        self.ensure_open()?;
        let isolation = isolation.unwrap_or(IsolationLevel::ReadCommitted);
        self.begin(isolation)?;
        Ok(Arc::new(SqliteTransaction::new(
            self.share_pinned(),
            isolation,
        )))
    }
}
