use crate::datasource_factory::DatasourceFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

static REGISTRY: Mutex<Option<HashMap<String, Arc<dyn DatasourceFactory>>>> = Mutex::new(None);

pub(crate) fn registry() -> MutexGuard<'static, Option<HashMap<String, Arc<dyn DatasourceFactory>>>>
{
    REGISTRY.lock().expect("registry")
}
