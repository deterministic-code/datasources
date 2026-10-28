use crate::driver::Driver;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct FactoryPair {
    pub driver: Arc<dyn Driver>,
}

static REGISTRY: Mutex<Option<HashMap<String, FactoryPair>>> = Mutex::new(None);

pub(crate) fn registry() -> MutexGuard<'static, Option<HashMap<String, FactoryPair>>> {
    REGISTRY.lock().expect("registry")
}

pub(crate) fn registered_driver(name: &str) -> Option<Arc<dyn Driver>> {
    registry().as_ref()?.get(name).map(|pair| pair.driver.clone())
}
