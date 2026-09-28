use crate::connection::Connection;
use crate::result::Result;
use std::collections::HashSet;
use std::future::Future;
use std::sync::{Arc, LazyLock, Mutex};

static OPEN: LazyLock<Mutex<HashSet<usize>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct UnitOfWorkTransaction {
    connection: Arc<dyn Connection>,
}

impl UnitOfWorkTransaction {
    pub fn new(connection: Arc<dyn Connection>) -> Self {
        Self { connection }
    }

    pub async fn do_work<T>(&self, work: impl Future<Output = Result<T>>) -> Result<T> {
        let key = data_ptr(&self.connection);
        let joined = {
            let mut open = OPEN.lock().expect("transactions");
            !open.insert(key)
        };
        if joined {
            return work.await;
        }
        let outcome = async {
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
        .await;
        OPEN.lock().expect("transactions").remove(&key);
        outcome
    }
}

fn data_ptr(connection: &Arc<dyn Connection>) -> usize {
    &**connection as *const dyn Connection as *const () as usize
}
