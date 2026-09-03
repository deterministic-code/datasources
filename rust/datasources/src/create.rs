use crate::datasource::Datasource;
use crate::datasource_error::DatasourceError;
use crate::defaults;
use crate::registry_state::registry;
use crate::result::Result;
use std::collections::HashMap;
use std::sync::Arc;

pub fn create(
    provider: &str,
    options: Option<&HashMap<String, String>>,
) -> Result<Arc<dyn Datasource>> {
    let mut merged = defaults::snapshot().unwrap_or_default();
    if let Some(extra) = options {
        merged.extend(extra.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
    let opts = (!merged.is_empty()).then_some(&merged);
    let guard = registry();
    match guard.as_ref().and_then(|map| map.get(provider)).cloned() {
        Some(factory) => Ok(factory.create(opts)),
        None => Err(DatasourceError::not_installed(provider)),
    }
}

pub fn create_default() -> Result<Arc<dyn Datasource>> {
    let merged = defaults::snapshot()
        .ok_or_else(|| DatasourceError::message("Datasource defaults are not specified"))?;
    let provider = merged
        .get("provider")
        .cloned()
        .ok_or_else(|| DatasourceError::message("Datasource defaults are not specified"))?;
    create(&provider, Some(&merged))
}
