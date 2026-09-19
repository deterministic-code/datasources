use crate::connection::Connection;
use crate::registry_state::{registry, FactoryPair};
use crate::repository::CRUDRepository;
use std::collections::HashMap;
use std::sync::Arc;

pub fn register(
    provider: impl Into<String>,
    connection: impl Fn(Option<&HashMap<String, String>>) -> Arc<dyn Connection> + Send + Sync + 'static,
    repository: impl Fn(Arc<dyn Connection>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static,
) {
    let mut guard = registry();
    let map = guard.get_or_insert_with(HashMap::new);
    map.insert(
        provider.into(),
        FactoryPair {
            connection: Arc::new(connection),
            repository: Arc::new(repository),
        },
    );
}
