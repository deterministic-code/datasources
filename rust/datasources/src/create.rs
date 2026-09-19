use crate::datasource::{BuiltDatasource, Datasource};
use crate::datasource_error::DatasourceError;
use crate::defaults;
use crate::provider_from_url::provider_from_url;
use crate::registry_state::registry;
use crate::result::Result;
use std::collections::HashMap;
use std::sync::Arc;

pub fn create(options: Option<&HashMap<String, String>>) -> Result<Arc<dyn Datasource>> {
    if options.is_none() {
        defaults::register_listed(&install_from_environment())?;
    }
    let listed = options.cloned().unwrap_or_else(connection_from_environment);
    let name = get(&listed, "provider")
        .and_then(|value| (!value.is_empty()).then_some(value.to_string()))
        .or_else(|| get(&listed, "database_url").and_then(provider_from_url))
        .ok_or_else(|| DatasourceError::message("Datasource defaults are not specified"))?;
    let bound = for_factory(&listed);
    let guard = registry();
    let pair = guard
        .as_ref()
        .and_then(|map| map.get(&name))
        .ok_or_else(|| DatasourceError::not_installed(&name))?;
    let connection = pair.connection.clone();
    let repository = pair.repository.clone();
    drop(guard);
    Ok(Arc::new(BuiltDatasource::new(
        name,
        move || connection(Some(&bound)),
        move |conn| repository(conn),
    )))
}

pub fn create_default() -> Result<Arc<dyn Datasource>> {
    create(None)
}

fn get<'a>(options: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    options
        .get(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
}

fn for_factory(options: &HashMap<String, String>) -> HashMap<String, String> {
    let mut copy = options.clone();
    if !copy.contains_key("url") {
        if let Some(url) = get(options, "database_url") {
            copy.insert("url".into(), url.into());
        }
    }
    if !copy.contains_key("schema") {
        if let Some(schema) = get(options, "database_schema") {
            copy.insert("schema".into(), schema.into());
        }
    }
    copy
}

fn connection_from_environment() -> HashMap<String, String> {
    let mut options = HashMap::new();
    if let Ok(url) = std::env::var("DATABASE_URL") {
        if !url.trim().is_empty() {
            options.insert("database_url".into(), url);
        }
    }
    if let Ok(schema) = std::env::var("DATABASE_SCHEMA") {
        if !schema.trim().is_empty() {
            options.insert("database_schema".into(), schema);
        }
    }
    options
}

fn install_from_environment() -> HashMap<String, String> {
    let mut options = HashMap::new();
    for key in ["INSTALLED_PROVIDERS", "DATABASE_URL"] {
        if let Ok(value) = std::env::var(key) {
            if !value.trim().is_empty() {
                options.insert(key.into(), value);
            }
        }
    }
    options
}
