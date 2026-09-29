use crate::connection::JsonConnection;
use crate::transaction::JsonTransaction;
use datasources::{
    downcast_connection, Command, Connection, Driver, IsolationLevel, Transaction, TypeConverter,
    Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct JsonDriver;

impl Driver for JsonDriver {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        JsonConnection::from_options(options)
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        let conn = downcast_connection::<JsonConnection>(connection).expect("json");
        Arc::new(JsonTransaction::new(conn.state.clone(), isolation))
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        HashMap::new()
    }
}
