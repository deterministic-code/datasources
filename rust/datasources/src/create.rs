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
    let name = get(&listed, "provider")
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

pub fn create_default() -> Result<Arc<dyn Datasource>> {
    default()
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
    for key in [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "DATABASE_SCHEMA",
        "PROVIDER",
        "provider",
    ] {
        if let Some(value) = env(key) {
            options.insert(key.into(), value);
        }
    }
    let url = env("DEFAULT_DATASOURCE_URL")
        .or_else(|| get(&options, "DATASOURCE_URL").map(str::to_string));
    let provider = env("DEFAULT_PROVIDER")
        .or_else(|| get(&options, "PROVIDER").map(str::to_string))
        .or_else(|| get(&options, "provider").map(str::to_string));
    if let Some(url) = url {
        options.insert("DATASOURCE_URL".into(), url.clone());
        options.insert("database_url".into(), url);
    }
    if let Some(provider) = provider {
        options.insert("provider".into(), provider);
    }
    if let Some(schema) = get(&options, "DATABASE_SCHEMA").map(str::to_string) {
        options.insert("database_schema".into(), schema);
    }
    options
}
