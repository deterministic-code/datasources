use crate::connection::MysqlConnection;
use crate::converters;
use crate::transaction::MysqlTransaction;
use datasources::{
    downcast_connection, Command, Connection, DatasourceError, Driver, IsolationLevel, Result,
    Transaction, TypeConverter, Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MysqlDriver;

pub(crate) fn column_sql(name: &str) -> Option<&'static str> {
    Some(match name {
        "int" => "INT",
        "uuid" => "CHAR(36)",
        "string" | "datetime" => "TEXT",
        "bool" => "TINYINT",
        "decimal" => "DECIMAL(18,4)",
        "float" => "DOUBLE",
        "binary" => "BLOB",
        _ => return None,
    })
}

impl Driver for MysqlDriver {
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
        Arc::new(MysqlTransaction::new(conn.share(), isolation))
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        converters::row_type_converters()
    }

    fn column_type(&self, name: &str) -> Result<String> {
        column_sql(name)
            .map(str::to_string)
            .ok_or_else(|| DatasourceError::message(format!("Unknown mysql type \"{name}\"")))
    }
}

#[cfg(test)]
mod tests {
    use super::MysqlDriver;
    use datasources::Driver;

    #[test]
    fn maps_column_types() {
        let driver = MysqlDriver;
        assert_eq!(driver.column_type("int").unwrap(), "INT");
        assert_eq!(driver.column_type("uuid").unwrap(), "CHAR(36)");
        assert_eq!(driver.column_type("string").unwrap(), "TEXT");
        assert_eq!(driver.column_type("datetime").unwrap(), "TEXT");
        assert_eq!(driver.column_type("bool").unwrap(), "TINYINT");
        assert_eq!(driver.column_type("decimal").unwrap(), "DECIMAL(18,4)");
        assert_eq!(driver.column_type("float").unwrap(), "DOUBLE");
        assert_eq!(driver.column_type("binary").unwrap(), "BLOB");
        assert!(driver
            .column_type("xml")
            .unwrap_err()
            .to_string()
            .contains("Unknown mysql type \"xml\""));
    }
}
