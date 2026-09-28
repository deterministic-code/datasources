use crate::connection::PostgresConnection;
use crate::converters;
use crate::transaction::PostgresTransaction;
use datasources::{
    downcast_connection, Command, Connection, DatasourceError, Driver, IsolationLevel, Result,
    Transaction, TypeConverter, Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct PostgresDriver;

impl Driver for PostgresDriver {
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
        Arc::new(PostgresTransaction::new(conn.share(), isolation))
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        converters::row_type_converters()
    }

    fn column_type(&self, name: &str) -> Result<String> {
        Ok(match name {
            "int" => "INTEGER",
            "uuid" => "UUID",
            "string" => "TEXT",
            "bool" => "BOOLEAN",
            "decimal" => "NUMERIC",
            "float" => "DOUBLE PRECISION",
            "datetime" => "TIMESTAMPTZ",
            "binary" => "BYTEA",
            _ => {
                return Err(DatasourceError::message(format!(
                    "Unknown postgres type \"{name}\""
                )))
            }
        }
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::PostgresDriver;
    use datasources::Driver;

    #[test]
    fn maps_column_types() {
        let driver = PostgresDriver;
        assert_eq!(driver.column_type("int").unwrap(), "INTEGER");
        assert_eq!(driver.column_type("uuid").unwrap(), "UUID");
        assert_eq!(driver.column_type("string").unwrap(), "TEXT");
        assert_eq!(driver.column_type("bool").unwrap(), "BOOLEAN");
        assert_eq!(driver.column_type("decimal").unwrap(), "NUMERIC");
        assert_eq!(driver.column_type("float").unwrap(), "DOUBLE PRECISION");
        assert_eq!(driver.column_type("datetime").unwrap(), "TIMESTAMPTZ");
        assert_eq!(driver.column_type("binary").unwrap(), "BYTEA");
        assert!(driver
            .column_type("xml")
            .unwrap_err()
            .to_string()
            .contains("Unknown postgres type \"xml\""));
    }
}
