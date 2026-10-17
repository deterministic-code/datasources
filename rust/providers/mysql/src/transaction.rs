use crate::connection::MysqlConnection;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Transaction};
use std::sync::Arc;

pub struct MysqlTransaction {
    connection: MysqlConnection,
    pub(crate) isolation: IsolationLevel,
}

impl MysqlTransaction {
    pub fn new(connection: MysqlConnection, isolation: IsolationLevel) -> Self {
        Self {
            connection,
            isolation,
        }
    }
}

#[async_trait]
impl Transaction for MysqlTransaction {
    fn connection(&self) -> Arc<dyn datasources::Connection> {
        Arc::new(self.connection.share())
    }

    fn isolation(&self) -> IsolationLevel {
        self.isolation
    }

    async fn commit(&self) -> Result<()> {
        self.connection.exec("COMMIT").await
    }

    async fn abort(&self) -> Result<()> {
        self.connection.exec("ROLLBACK").await
    }
}
