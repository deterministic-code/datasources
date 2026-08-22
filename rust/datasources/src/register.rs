use crate::datasource_factory::DatasourceFactory;
use crate::registry_state::registry;
use std::collections::HashMap;
use std::sync::Arc;

pub fn register(factory: Arc<dyn DatasourceFactory>) {
    let mut guard = registry();
    let map = guard.get_or_insert_with(HashMap::new);
    map.insert(factory.provider().to_string(), factory);
}
