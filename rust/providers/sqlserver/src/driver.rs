use crate::connection::SqlServerConnection;
use crate::sql_generate;
use crate::statement_renderer::SqlServerStatementRenderer;
use datasources::{
    Command, Connection, DefaultValueConverter, DeterministicTypeConverter, Driver, Information,
    IsolationLevel, Mapping, StatementTextRenderer, StubTransaction, Transaction, TypeConverter,
    Value,
};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

pub struct SqlServerDriver;

impl SqlServerDriver {
    pub const QUOTE_LEFT: &'static str = sql_generate::QUOTE_LEFT;
    pub const QUOTE_RIGHT: &'static str = sql_generate::QUOTE_RIGHT;

    pub fn statement_text_renderer() -> &'static StatementTextRenderer {
        static DRIVER: SqlServerDriver = SqlServerDriver;
        Driver::statement_renderer(&DRIVER).expect("sqlserver")
    }

    pub fn types() -> &'static DeterministicTypeConverter {
        &*sql_generate::TYPES
    }

    pub fn defaults() -> &'static DefaultValueConverter {
        &sql_generate::DEFAULTS
    }
}

static STATEMENTS: LazyLock<StatementTextRenderer> =
    LazyLock::new(|| StatementTextRenderer::new(Some(Box::new(SqlServerStatementRenderer)), None));

static INFORMATION: LazyLock<Information> = LazyLock::new(|| {
    Information::new(
        ["SQL Server driver."],
        [
            "Overrides CreateCreateStoredProcedureStatement to PROCEDURE Create{Plural}",
            "Overrides CreateReadStoredProcedureStatement to PROCEDURE Read{Plural}By{Properties}",
            "Overrides CreateUpdateStoredProcedureStatement to PROCEDURE Update{Plural}By{Properties}",
            "Overrides CreateDeleteStoredProcedureStatement to PROCEDURE Delete{Plural}By{Properties}",
            "Overrides CreateBulkCreateStoredProcedureStatement to PROCEDURE BulkCreate{Plural}",
            "Overrides CreateBulkUpdateStoredProcedureStatement to PROCEDURE BulkUpdate{Plural}",
            "Overrides CreateBulkDeleteStoredProcedureStatement to PROCEDURE BulkDelete{Plural}",
        ],
        [] as [&str; 0],
    )
});

impl Driver for SqlServerDriver {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        SqlServerConnection::from_options(options)
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        _isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        Arc::new(StubTransaction::new("sqlserver", connection))
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        HashMap::new()
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
