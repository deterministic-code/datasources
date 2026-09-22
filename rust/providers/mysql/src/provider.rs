use crate::connection::MysqlConnection;
use crate::converters;
use crate::transaction::MysqlTransaction;
use datasources::{
    downcast_connection, Command, Connection, Driver, IsolationLevel, Transaction, TypeConverter,
    Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MysqlProvider;

impl Driver for MysqlProvider {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        MysqlConnection::from_options(options)
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        let conn = downcast_connection::<MysqlConnection>(connection).expect("mysql");
        Arc::new(MysqlTransaction::new(conn.driver.clone(), isolation))
    }

    fn types(&self) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        converters::types()
    }
}
