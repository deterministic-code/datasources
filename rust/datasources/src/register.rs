use crate::driver::Driver;
use crate::registry_state::{registry, FactoryPair};
use std::collections::HashMap;
use std::sync::Arc;

pub fn register(provider: impl Into<String>, driver: Arc<dyn Driver>) {
    let mut guard = registry();
    let map = guard.get_or_insert_with(HashMap::new);
    map.insert(provider.into(), FactoryPair { driver });
}
