use crate::connection::SqliteConnection;
use crate::converters;
use crate::statement_renderer::SqliteStatementRenderer;
use crate::transaction::SqliteTransaction;
use crate::sql_generate;
use datasources::{
    downcast_connection, Command, Connection, DatasourceError, DefaultValueConverter, Driver,
    Information, IsolationLevel, Mapping, Result, StatementTextRenderer, Transaction, TypeConverter,
    Value,
};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

pub struct SqliteDriver;

impl SqliteDriver {
    pub const QUOTE_LEFT: &'static str = sql_generate::QUOTE_LEFT;
    pub const QUOTE_RIGHT: &'static str = sql_generate::QUOTE_RIGHT;

    pub fn statement_text_renderer() -> &'static StatementTextRenderer {
        static DRIVER: SqliteDriver = SqliteDriver;
        Driver::statement_renderer(&DRIVER).expect("sqlite")
    }
}

static STATEMENTS: LazyLock<StatementTextRenderer> =
    LazyLock::new(|| StatementTextRenderer::new(Some(Box::new(SqliteStatementRenderer)), None));

static INFORMATION: LazyLock<Information> = LazyLock::new(|| {
    Information::new(
        [
            "Embedded SQL database for local development and tests.",
            "Full connection, command, transaction, and repository support.",
        ],
        [
            "Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}",
            "Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)",
        ],
        [] as [&str; 0],
    )
});

pub(crate) fn column_sql(name: &str) -> Option<&'static str> {
    Some(match name {
        "int" => "INTEGER",
        "bool" => "BOOLEAN",
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

    fn type_converter(&self) -> Option<&dyn Mapping<String, String>> {
        Some(&*sql_generate::TYPES)
    }

    fn default_value_converter(&self) -> Option<&DefaultValueConverter> {
        Some(&sql_generate::DEFAULTS)
    }

    fn statement_renderer(&self) -> Option<&StatementTextRenderer> {
        Some(&STATEMENTS)
    }

    fn information(&self) -> Option<&Information> {
        Some(&INFORMATION)
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
        assert_eq!(driver.column_type("bool").unwrap(), "BOOLEAN");
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

    #[test]
    fn maps_property_targets() {
        use datasources::converters::RowTypeConverter;
        use datasources::Value;
        use std::collections::HashMap;

        let driver = SqliteDriver;
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
            Some(&Value::from("2020-01-02T03:04:05.000Z"))
        );
    }
}
