use crate::connection::OracleConnection;
use crate::sql_generate;
use crate::statement_renderer::OracleStatementRenderer;
use datasources::{
    Column, Command, Connection, Driver, Information, IsolationLevel, Mapping, Result,
    RoutineParameter,
    StatementTextRenderer, StubTransaction, Transaction, TypeConverter, Value,
};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

pub struct OracleDriver;

impl OracleDriver {
    pub const QUOTE_LEFT: &'static str = sql_generate::QUOTE_LEFT;
    pub const QUOTE_RIGHT: &'static str = sql_generate::QUOTE_RIGHT;
    pub const PROCEDURE_PAYLOAD_TYPE: &'static str = sql_generate::PROCEDURE_PAYLOAD_TYPE;

    pub fn procedure_specs(
        entity_name: &str,
        by_groups: &[Vec<String>],
        routine_name: &dyn Fn(&str) -> String,
        plural_stem: &dyn Fn(&str) -> String,
    ) -> Vec<(String, String, Vec<String>)> {
        sql_generate::procedure_specs(entity_name, by_groups, routine_name, plural_stem)
    }

    pub fn apply_keywords(sql: &str, keyword: &dyn Fn(&str) -> String) -> String {
        sql_generate::apply_keywords(sql, keyword)
    }

    pub fn statement_text_renderer() -> &'static StatementTextRenderer {
        static DRIVER: OracleDriver = OracleDriver;
        Driver::statement_renderer(&DRIVER).expect("oracle")
    }

    pub fn types() -> &'static crate::type_map::TypeMap {
        &*sql_generate::TYPES
    }

    pub fn defaults() -> &'static crate::default_value_converter::DefaultValueConverter {
        &sql_generate::DEFAULTS
    }

    pub fn convert_type(field: &datasources::SpecField) -> Result<String> {
        sql_generate::convert_type(field)
    }

    pub fn convert_default(spec_type: &str, text: &str) -> Result<Option<String>> {
        sql_generate::convert_default(spec_type, text)
    }

    pub fn quote(name: &str) -> String {
        format!("\"{name}\"")
    }

    pub fn native_type(column: &Column) -> String {
        column
            .field
            .as_ref()
            .and_then(|field| sql_generate::TYPES.convert_field(field).ok())
            .unwrap_or_else(|| column.type_name.clone())
    }

    pub fn native_default(column: &Column) -> String {
        let Some(default) = &column.default else {
            return String::new();
        };
        let Some(field) = &column.field else {
            return default.value.clone();
        };
        sql_generate::DEFAULTS
            .convert(&field.type_name, &default.value)
            .ok()
            .flatten()
            .unwrap_or_else(|| default.value.clone())
    }

    pub fn utc_now() -> String {
        sql_generate::DEFAULTS
            .convert("datetime", "UtcNow")
            .ok()
            .flatten()
            .unwrap_or_default()
    }

    pub fn native_param(parameter: &RoutineParameter) -> String {
        if parameter.r#type == "payload" {
            return Self::PROCEDURE_PAYLOAD_TYPE.to_string();
        }
        parameter
            .field
            .as_ref()
            .and_then(|field| sql_generate::TYPES.convert_field(field).ok())
            .unwrap_or_else(|| parameter.r#type.clone())
    }
}

static STATEMENTS: LazyLock<StatementTextRenderer> =
    LazyLock::new(|| StatementTextRenderer::new(Some(Box::new(OracleStatementRenderer)), None));

static INFORMATION: LazyLock<Information> = LazyLock::new(|| {
    Information::new(
        ["Oracle driver."],
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

impl Driver for OracleDriver {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        OracleConnection::from_options(options)
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        _: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        Arc::new(StubTransaction::new("oracle", connection))
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        HashMap::new()
    }

    fn type_converter(&self) -> Option<&dyn Mapping<String, String>> {
        Some(&*sql_generate::TYPES)
    }

    fn statement_renderer(&self) -> Option<&StatementTextRenderer> {
        Some(&STATEMENTS)
    }

    fn information(&self) -> Option<&Information> {
        Some(&INFORMATION)
    }
}
