use crate::connection::Connection;
use crate::repository::ICRUDRepository;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

pub type ConnectionFn =
    Arc<dyn Fn(Option<&HashMap<String, String>>) -> Arc<dyn Connection> + Send + Sync>;
pub type RepositoryFn = Arc<dyn Fn(Arc<dyn Connection>) -> Arc<dyn ICRUDRepository> + Send + Sync>;

pub struct FactoryPair {
    pub connection: ConnectionFn,
    pub repository: RepositoryFn,
}

static REGISTRY: Mutex<Option<HashMap<String, FactoryPair>>> = Mutex::new(None);

pub(crate) fn registry() -> MutexGuard<'static, Option<HashMap<String, FactoryPair>>> {
    REGISTRY.lock().expect("registry")
}
