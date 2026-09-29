use crate::datasource::{BuiltDatasource, Datasource};
use crate::datasource_error::DatasourceError;
use crate::defaults;
use crate::dot_env;
use crate::registry_state::registry;
use crate::result::Result;
use std::collections::HashMap;
use std::sync::Arc;

pub fn create(options: Option<&HashMap<String, String>>) -> Result<Arc<dyn Datasource>> {
    let listed = match options {
        Some(options) => options.clone(),
        None => {
            let listed = merge(&dot_env::load(".env"), env);
            defaults::register_listed(&listed)?;
            listed
        }
    };
    let name = get(&listed, "PROVIDER")
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .ok_or_else(|| DatasourceError::message("Datasource defaults are not specified"))?;
    let guard = registry();
    let pair = guard
        .as_ref()
        .and_then(|map| map.get(&name))
        .ok_or_else(|| DatasourceError::not_installed(&name))?;
    let driver = pair.driver.clone();
    drop(guard);
    Ok(Arc::new(BuiltDatasource::new(name, driver, Some(listed))))
}

pub fn default() -> Result<Arc<dyn Datasource>> {
    create(None)
}

fn get<'a>(options: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    options
        .get(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
}

fn env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn merge(
    file: &HashMap<String, String>,
    env: fn(&str) -> Option<String>,
) -> HashMap<String, String> {
    let mut options = file.clone();
    for key in ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"] {
        if let Some(value) = env(key) {
            options.insert(key.into(), value);
        }
    }
    options
}
