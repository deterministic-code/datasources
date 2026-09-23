use crate::connection::SqliteConnection;
use crate::converters;
use crate::transaction::SqliteTransaction;
use datasources::{
    downcast_connection, Command, Connection, Driver, IsolationLevel, Transaction, TypeConverter,
    Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SqliteProvider;

impl Driver for SqliteProvider {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        SqliteConnection::from_options(options)
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        let conn = downcast_connection::<SqliteConnection>(connection).expect("sqlite");
        Arc::new(SqliteTransaction::new(conn.driver.clone(), isolation))
    }

    fn row_type_converters(&self) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        converters::row_type_converters()
    }
}
