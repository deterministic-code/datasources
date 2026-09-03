use datasources::{DatasourceError, Result};
use mysql_async::prelude::Queryable;
use mysql_async::Conn;
use tokio::sync::Mutex;

pub(crate) fn map_err(error: mysql_async::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

pub struct MysqlConnectionDriver {
    pub(crate) conn: Mutex<Option<Conn>>,
}

impl MysqlConnectionDriver {
    pub fn new() -> Self {
        Self {
            conn: Mutex::new(None),
        }
    }

    pub async fn open(&self, url: &str) -> Result<()> {
        let mut slot = self.conn.lock().await;
        if slot.is_some() {
            return Ok(());
        }
        let mut conn = Conn::from_url(url.to_string()).await.map_err(map_err)?;
        conn.query_drop("SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')")
            .await
            .map_err(map_err)?;
        *slot = Some(conn);
        Ok(())
    }

    pub async fn close(&self) -> Result<()> {
        *self.conn.lock().await = None;
        Ok(())
    }
}

impl Default for MysqlConnectionDriver {
    fn default() -> Self {
        Self::new()
    }
}
