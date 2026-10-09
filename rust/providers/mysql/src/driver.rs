use crate::connection::MysqlConnection;
use crate::converters;
use crate::statement_renderer::MysqlStatementRenderer;
use crate::transaction::MysqlTransaction;
use datasources::{
    downcast_connection, Command, Connection, DatasourceError, Driver, Information, IsolationLevel,
    Result, StatementTextRenderer, Transaction, TypeConverter, Value,
};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

pub struct MysqlDriver;

static STATEMENTS: LazyLock<StatementTextRenderer> =
    LazyLock::new(|| StatementTextRenderer::new(Some(Box::new(MysqlStatementRenderer)), None));

static INFORMATION: LazyLock<Information> = LazyLock::new(|| {
    Information::new(
        [
            "MySQL / MariaDB server driver.",
            "Full connection, command, transaction, and repository support.",
        ],
        [
            "Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}",
            "Overrides CreateTableStatement identity to AUTO_INCREMENT",
            "Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)",
        ],
        [] as [&str; 0],
    )
});

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

    fn statement_renderer(&self) -> Option<&StatementTextRenderer> {
        Some(&STATEMENTS)
    }

    fn information(&self) -> Option<&Information> {
        Some(&INFORMATION)
    }
}

datasources::export_driver!(datasources_mysql_driver, MysqlDriver);

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

    #[test]
    fn maps_property_targets() {
        use datasources::converters::RowTypeConverter;
        use datasources::Value;
        use std::collections::HashMap;

        let driver = MysqlDriver;
        let columns = HashMap::from([
            ("Key".into(), "uuid".into()),
            ("Active".into(), "bool".into()),
            ("Login".into(), "string".into()),
            ("Created".into(), "datetime".into()),
        ]);
        let converter = RowTypeConverter::from(|| driver.row_type_converters(), &columns);
        let row = HashMap::from([
            (
                "Key".into(),
                Value::from("11111111-1111-1111-1111-111111111111"),
            ),
            ("Active".into(), Value::Int(1)),
            ("Login".into(), Value::from("ada")),
            ("Created".into(), Value::from("2020-01-02T03:04:05.000Z")),
        ]);
        let converted = converter.convert(&row).unwrap();
        assert!(matches!(converted.get("Key"), Some(Value::Uuid(_))));
        assert_eq!(converted.get("Active"), Some(&Value::Bool(true)));
        assert_eq!(converted.get("Login"), Some(&Value::from("ada")));
        assert!(matches!(converted.get("Created"), Some(Value::DateTime(_))));
        let stored = converter.reverse_convert(&converted).unwrap();
        assert_eq!(
            stored.get("Key"),
            Some(&Value::from("11111111-1111-1111-1111-111111111111"))
        );
        assert_eq!(stored.get("Active"), Some(&Value::Int(1)));
        assert_eq!(stored.get("Login"), Some(&Value::from("ada")));
        assert_eq!(
            stored.get("Created"),
            Some(&Value::from("2020-01-02 03:04:05.000"))
        );
    }
}
