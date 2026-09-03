use crate::command_driver::SqliteCommandDriver;
use crate::connection_driver::{map_err, SqliteConnectionDriver};
use datasources::{DatasourceError, IsolationLevel, Result};
use std::collections::HashMap;

pub struct SqliteTransactionDriver<'a> {
    connection: &'a SqliteConnectionDriver,
}

impl<'a> SqliteTransactionDriver<'a> {
    pub fn new(connection: &'a SqliteConnectionDriver) -> Self {
        Self { connection }
    }

    pub fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        let db = self.connection.db.lock().unwrap();
        let conn = db
            .as_ref()
            .ok_or_else(|| DatasourceError::message("SQLite driver is not open"))?;
        let flag = matches!(isolation, IsolationLevel::ReadUncommitted) as i64;
        conn.pragma_update(None, "read_uncommitted", flag)
            .map_err(map_err)?;
        conn.execute("BEGIN", []).map_err(map_err).map(|_| ())
    }

    pub fn commit(&self) -> Result<()> {
        SqliteCommandDriver::new(self.connection)
            .execute("COMMIT", &HashMap::new())
            .map(|_| ())
    }

    pub fn rollback(&self) -> Result<()> {
        SqliteCommandDriver::new(self.connection)
            .execute("ROLLBACK", &HashMap::new())
            .map(|_| ())
    }
}
