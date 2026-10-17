use crate::connection::SqliteConnection;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Transaction};
use std::sync::Arc;

pub struct SqliteTransaction {
    connection: SqliteConnection,
    pub(crate) isolation: IsolationLevel,
}

impl SqliteTransaction {
    pub fn new(connection: SqliteConnection, isolation: IsolationLevel) -> Self {
        Self {
            connection,
            isolation,
        }
    }
}

#[async_trait]
impl Transaction for SqliteTransaction {
    fn connection(&self) -> Arc<dyn datasources::Connection> {
        Arc::new(self.connection.share())
    }

    fn isolation(&self) -> IsolationLevel {
        self.isolation
    }

    async fn commit(&self) -> Result<()> {
        self.connection.exec("COMMIT")
    }

    async fn abort(&self) -> Result<()> {
        self.connection.exec("ROLLBACK")
    }
}
