use crate::connection_driver::{map_err, MysqlConnectionDriver};
use crate::driver::isolation_sql;
use datasources::{DatasourceError, IsolationLevel, Result};
use mysql_async::prelude::Queryable;

pub struct MysqlTransactionDriver<'a> {
    connection: &'a MysqlConnectionDriver,
}

impl<'a> MysqlTransactionDriver<'a> {
    pub fn new(connection: &'a MysqlConnectionDriver) -> Self {
        Self { connection }
    }

    pub async fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        let sql = format!(
            "SET TRANSACTION ISOLATION LEVEL {}",
            isolation_sql(isolation)
        );
        let mut slot = self.connection.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        conn.query_drop(sql).await.map_err(map_err)?;
        conn.query_drop("START TRANSACTION").await.map_err(map_err)
    }

    pub async fn commit(&self) -> Result<()> {
        let mut slot = self.connection.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        conn.query_drop("COMMIT").await.map_err(map_err)
    }

    pub async fn rollback(&self) -> Result<()> {
        let mut slot = self.connection.conn.lock().await;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("MySQL driver is not open"))?;
        conn.query_drop("ROLLBACK").await.map_err(map_err)
    }
}
