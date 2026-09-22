use crate::connection::MemoryConnection;
use crate::transaction::MemoryTransaction;
use datasources::{
    downcast_connection, Command, Connection, Driver, IsolationLevel, Transaction, TypeConverter,
    Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MemoryProvider;

impl Driver for MemoryProvider {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        MemoryConnection::from_options(options)
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        let conn = downcast_connection::<MemoryConnection>(connection).expect("memory");
        Arc::new(MemoryTransaction::new(conn.state.clone(), isolation))
    }

    fn types(&self) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        HashMap::new()
    }
}
