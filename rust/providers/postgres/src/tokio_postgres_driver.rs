use crate::command_driver::PostgresCommandDriver;
use crate::connection_driver::PostgresConnectionDriver;
use crate::driver::{PostgresDriver, PostgresExecuteResult};
use crate::transaction_driver::PostgresTransactionDriver;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Value};
use std::collections::HashMap;

pub struct TokioPostgresDriver {
    connection: PostgresConnectionDriver,
}

impl TokioPostgresDriver {
    pub fn new(schema: Option<String>) -> Self {
        Self {
            connection: PostgresConnectionDriver::new(schema),
        }
    }
}

#[async_trait]
impl PostgresDriver for TokioPostgresDriver {
    async fn open(&self, url: &str) -> Result<()> {
        self.connection.open(url).await
    }

    async fn close(&self) -> Result<()> {
        self.connection.close().await
    }

    async fn execute(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<PostgresExecuteResult> {
        PostgresCommandDriver::new(&self.connection)
            .execute(sql, params)
            .await
    }

    async fn query(
        &self,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<Vec<datasources::Row>> {
        PostgresCommandDriver::new(&self.connection)
            .query(sql, params)
            .await
    }

    async fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        PostgresTransactionDriver::new(&self.connection)
            .begin(isolation)
            .await
    }

    async fn commit(&self) -> Result<()> {
        PostgresTransactionDriver::new(&self.connection)
            .commit()
            .await
    }

    async fn rollback(&self) -> Result<()> {
        PostgresTransactionDriver::new(&self.connection)
            .rollback()
            .await
    }
}
