use crate::datasource::Datasource;
use crate::datasource_error::DatasourceError;
use crate::registry_state::registry;
use crate::result::Result;
use std::sync::Arc;

pub fn create(provider: &str, options: Option<&str>) -> Result<Arc<dyn Datasource>> {
    let guard = registry();
    match guard.as_ref().and_then(|map| map.get(provider)).cloned() {
        Some(factory) => Ok(factory.create(options)),
        None => Err(DatasourceError::not_installed(provider)),
    }
}
