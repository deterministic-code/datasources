use datasources::{DatasourceError, Driver, ProviderCatalog, Result, Statement, StatementNaming};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

fn store() -> &'static Mutex<HashMap<String, Arc<dyn Driver>>> {
    static DRIVERS: OnceLock<Mutex<HashMap<String, Arc<dyn Driver>>>> = OnceLock::new();
    DRIVERS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_dialect(dialect: impl Into<String>, driver: Arc<dyn Driver>) -> Result<()> {
    let dialect = dialect.into();
    require_name(&dialect)?;
    if driver.statement_renderer().is_none() {
        return Err(DatasourceError::message(format!("\"{dialect}\" is not a SQL dialect.")));
    }
    store().lock().expect("drivers").insert(dialect, driver);
    Ok(())
}

pub fn driver_of(dialect: &str) -> Result<Arc<dyn Driver>> {
    require_name(dialect)?;
    store()
        .lock()
        .expect("drivers")
        .get(dialect)
        .cloned()
        .ok_or_else(|| DatasourceError::not_installed(dialect))
}

pub fn schema_renderer(dialect: &str, naming: StatementNaming) -> Result<datasources::StatementTextRenderer> {
    Ok(driver_of(dialect)?
        .statement_renderer()
        .expect("sql")
        .with_naming(naming))
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
        .statement_renderer()
        .expect("sql")
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
