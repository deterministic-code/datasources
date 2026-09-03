use crate::command_driver::PostgresCommandDriver;
use crate::connection_driver::PostgresConnectionDriver;
use crate::driver::isolation_sql;
use datasources::{IsolationLevel, Result};

pub struct PostgresTransactionDriver<'a> {
    commands: PostgresCommandDriver<'a>,
}

impl<'a> PostgresTransactionDriver<'a> {
    pub fn new(connection: &'a PostgresConnectionDriver) -> Self {
        Self {
            commands: PostgresCommandDriver::new(connection),
        }
    }

    pub async fn begin(&self, isolation: IsolationLevel) -> Result<()> {
        self.commands
            .exec(&format!(
                "BEGIN ISOLATION LEVEL {}",
                isolation_sql(isolation)
            ))
            .await
    }

    pub async fn commit(&self) -> Result<()> {
        self.commands.exec("COMMIT").await
    }

    pub async fn rollback(&self) -> Result<()> {
        self.commands.exec("ROLLBACK").await
    }
}
