use crate::connection::PostgresConnection;
use async_trait::async_trait;
use datasources::{IsolationLevel, Result, Transaction};

pub struct PostgresTransaction {
    connection: PostgresConnection,
    pub(crate) isolation: IsolationLevel,
}

impl PostgresTransaction {
    pub fn new(connection: PostgresConnection, isolation: IsolationLevel) -> Self {
        Self {
            connection,
            isolation,
        }
    }
}

#[async_trait]
impl Transaction for PostgresTransaction {
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
