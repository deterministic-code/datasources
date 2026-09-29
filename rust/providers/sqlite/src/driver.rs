use crate::connection::SqliteConnection;
use crate::converters;
use crate::transaction::SqliteTransaction;
use datasources::{
    downcast_connection, Command, Connection, DatasourceError, Driver, IsolationLevel, Result,
    Transaction, TypeConverter, Value,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SqliteDriver;

pub(crate) fn column_sql(name: &str) -> Option<&'static str> {
    Some(match name {
        "int" | "bool" => "INTEGER",
        "uuid" | "string" | "datetime" => "TEXT",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "binary" => "BLOB",
        _ => return None,
    })
}

impl Driver for SqliteDriver {
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
        Arc::new(SqliteTransaction::new(conn.share(), isolation))
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        converters::row_type_converters()
    }

    fn column_type(&self, name: &str) -> Result<String> {
        column_sql(name)
            .map(str::to_string)
            .ok_or_else(|| DatasourceError::message(format!("Unknown sqlite type \"{name}\"")))
    }
}

datasources::export_driver!(datasources_sqlite_driver, SqliteDriver);

#[cfg(test)]
mod tests {
    use super::SqliteDriver;
    use datasources::Driver;

    #[test]
    fn maps_column_types() {
        let driver = SqliteDriver;
        assert_eq!(driver.column_type("int").unwrap(), "INTEGER");
        assert_eq!(driver.column_type("uuid").unwrap(), "TEXT");
        assert_eq!(driver.column_type("string").unwrap(), "TEXT");
        assert_eq!(driver.column_type("bool").unwrap(), "INTEGER");
        assert_eq!(driver.column_type("decimal").unwrap(), "NUMERIC");
        assert_eq!(driver.column_type("float").unwrap(), "REAL");
        assert_eq!(driver.column_type("datetime").unwrap(), "TEXT");
        assert_eq!(driver.column_type("binary").unwrap(), "BLOB");
        assert!(driver
            .column_type("xml")
            .unwrap_err()
            .to_string()
            .contains("Unknown sqlite type \"xml\""));
    }
}
