use crate::sql_generate_driver::SqlGenerateDriver;
use datasources::{DatasourceError, ProviderCatalog, Result, SpecField, Statement};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

fn store() -> &'static Mutex<HashMap<String, &'static SqlGenerateDriver>> {
    static DRIVERS: OnceLock<Mutex<HashMap<String, &'static SqlGenerateDriver>>> = OnceLock::new();
    DRIVERS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_dialect(dialect: impl Into<String>, driver: SqlGenerateDriver) -> Result<()> {
    let dialect = dialect.into();
    require_name(&dialect)?;
    let leaked: &'static SqlGenerateDriver = Box::leak(Box::new(driver));
    store().lock().expect("drivers").insert(dialect, leaked);
    Ok(())
}

pub fn driver_of(dialect: &str) -> Result<&'static SqlGenerateDriver> {
    require_name(dialect)?;
    store()
        .lock()
        .expect("drivers")
        .get(dialect)
        .copied()
        .ok_or_else(|| DatasourceError::not_installed(dialect))
}

fn require_name(dialect: &str) -> Result<()> {
    if ProviderCatalog::default().contains_key(dialect) {
        Ok(())
    } else {
        Err(DatasourceError::message(format!(
            "Unknown SQL dialect \"{dialect}\"."
        )))
    }
}

pub fn require(language: &str) -> Result<String> {
    driver_of(language)?;
    Ok(language.to_string())
}

pub fn render_sql(dialect: &str, statement: &dyn Statement) -> Result<String> {
    let sql = driver_of(dialect)?
        .statement_renderer
        .render(statement)
        .trim_end()
        .to_string();
    Ok(if sql.is_empty() {
        String::new()
    } else if sql.ends_with(';') || sql.ends_with('/') {
        sql
    } else {
        format!("{sql};")
    })
}

pub fn q(dialect: &str, ident: &str) -> Result<String> {
    let driver = driver_of(dialect)?;
    Ok(format!("{}{ident}{}", driver.quote_left, driver.quote_right))
}

pub fn map_column_type(dialect: &str, field: &SpecField) -> Result<String> {
    driver_of(dialect)?.type_converter.convert(field)
}

pub fn sql_default(dialect: &str, field: &SpecField) -> Result<Option<String>> {
    let Some(default) = &field.default_value else {
        return Ok(None);
    };
    driver_of(dialect)?
        .default_value_converter
        .convert(&field.type_name, default)
}
