use crate::connection::Connection;
use crate::datasource_error::DatasourceError;
use crate::i_unit_of_work::IUnitOfWork;
use crate::result::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::{Arc, LazyLock, Mutex};

static OPEN: LazyLock<Mutex<HashSet<usize>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

enum Phase {
    New,
    Joined,
    Owner(Arc<dyn Transaction>),
}

pub struct UnitOfWorkTransaction {
    connection: Arc<dyn Connection>,
    phase: Phase,
}

impl UnitOfWorkTransaction {
    pub fn new(connection: Arc<dyn Connection>) -> Self {
        Self {
            connection,
            phase: Phase::New,
        }
    }

    fn release(&self) {
        OPEN.lock()
            .expect("transactions")
            .remove(&data_ptr(&self.connection));
    }
}

#[async_trait]
impl IUnitOfWork for UnitOfWorkTransaction {
    async fn begin(&mut self) -> Result<()> {
        if !matches!(self.phase, Phase::New) {
            return Err(DatasourceError::message("Unit of work already begun"));
        }
        let key = data_ptr(&self.connection);
        let owner = {
            let mut open = OPEN.lock().expect("transactions");
            open.insert(key)
        };
        if !owner {
            self.phase = Phase::Joined;
            return Ok(());
        }
        match self.connection.start_transaction(None).await {
            Ok(txn) => {
                self.phase = Phase::Owner(txn);
                Ok(())
            }
            Err(error) => {
                OPEN.lock().expect("transactions").remove(&key);
                Err(error)
            }
        }
    }

    async fn commit(&mut self) -> Result<()> {
        match std::mem::replace(&mut self.phase, Phase::Joined) {
            Phase::New => {
                self.phase = Phase::New;
                Err(DatasourceError::message("Unit of work has not begun"))
            }
            Phase::Joined => Ok(()),
            Phase::Owner(txn) => match txn.commit().await {
                Ok(()) => {
                    self.release();
                    Ok(())
                }
                Err(error) => {
                    self.phase = Phase::Owner(txn);
                    Err(error)
                }
            },
        }
    }

    async fn abort(&mut self) -> Result<()> {
        match std::mem::replace(&mut self.phase, Phase::Joined) {
            Phase::New => {
                self.phase = Phase::New;
                Err(DatasourceError::message("Unit of work has not begun"))
            }
            Phase::Joined => Ok(()),
            Phase::Owner(txn) => {
                let outcome = txn.abort().await;
                self.release();
                outcome
            }
        }
    }
}

fn data_ptr(connection: &Arc<dyn Connection>) -> usize {
    &**connection as *const dyn Connection as *const () as usize
}
