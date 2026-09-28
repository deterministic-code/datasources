use crate::connection::Connection;
use crate::result::Result;
use std::future::Future;
use std::sync::Arc;

pub struct UnitOfWork {
    connection: Arc<dyn Connection>,
}

impl UnitOfWork {
    pub fn new(connection: Arc<dyn Connection>) -> Self {
        Self { connection }
    }

    pub async fn in_transaction<T>(&self, work: impl Future<Output = Result<T>>) -> Result<T> {
        let txn = self.connection.start_transaction(None).await?;
        match work.await {
            Ok(value) => {
                txn.commit().await?;
                Ok(value)
            }
            Err(error) => {
                txn.abort().await?;
                Err(error)
            }
        }
    }
}
