use crate::connection::PostgresConnection;
use crate::converters;
use crate::transaction::PostgresTransaction;
use datasources::{
    downcast_connection, Command, Connection, Driver, IsolationLevel, Transaction, TypeConverter,
    Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct PostgresProvider;

impl Driver for PostgresProvider {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        PostgresConnection::from_options(options)
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        let conn = downcast_connection::<PostgresConnection>(connection).expect("postgres");
        Arc::new(PostgresTransaction::new(conn.driver.clone(), isolation))
    }

    fn types(&self) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        converters::types()
    }
}
