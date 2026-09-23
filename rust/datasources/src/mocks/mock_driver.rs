use crate::command::Command;
use crate::connection::Connection;
use crate::converters::TypeConverter;
use crate::driver::Driver;
use crate::isolation_level::IsolationLevel;
use crate::transaction::Transaction;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

use super::mock_connection::MockConnection;
use super::mock_transaction::MockTransaction;

pub struct MockDriver;

impl MockDriver {
    pub fn arc() -> Arc<dyn Driver> {
        Arc::new(Self)
    }
}

impl Driver for MockDriver {
    fn connection(&self, _: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        MockConnection::new()
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        _: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        let txn = MockTransaction::new_unbound();
        txn.set_isolation(isolation);
        Arc::new(txn)
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        HashMap::new()
    }
}
