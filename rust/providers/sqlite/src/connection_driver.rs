use crate::connection_string;
use datasources::{DatasourceError, Result};
use rusqlite::Connection;
use std::sync::Mutex;

pub(crate) fn map_err(error: rusqlite::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

pub struct SqliteConnectionDriver {
    pub(crate) db: Mutex<Option<Connection>>,
}

impl SqliteConnectionDriver {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(None),
        }
    }

    pub fn open(&self, filename: &str) -> Result<()> {
        let mut db = self.db.lock().unwrap();
        if db.is_some() {
            return Ok(());
        }
        let conn = connection_string::open(filename).map_err(map_err)?;
        conn.busy_timeout(std::time::Duration::ZERO)
            .map_err(map_err)?;
        *db = Some(conn);
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        *self.db.lock().unwrap() = None;
        Ok(())
    }
}

impl Default for SqliteConnectionDriver {
    fn default() -> Self {
        Self::new()
    }
}
