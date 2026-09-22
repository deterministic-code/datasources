use crate::datasource_error::DatasourceError;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Maps {
    readonly: HashMap<TypeId, Arc<dyn IReadonlyRepository>>,
    crud: HashMap<TypeId, Arc<dyn ICRUDRepository>>,
}

pub trait ReadonlyRepositoryFactory: Send + Sync {
    fn register(&self, repository: Arc<dyn IReadonlyRepository>);
    fn create<T: 'static>(&self) -> Result<Arc<dyn IReadonlyRepository>>;
}

pub trait CRUDRepositoryFactory: Send + Sync {
    fn register(&self, repository: Arc<dyn ICRUDRepository>);
    fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>>;
}

pub struct RepositoryFactory {
    maps: Mutex<Maps>,
}

impl Default for RepositoryFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl RepositoryFactory {
    pub fn new() -> Self {
        Self {
            maps: Mutex::new(Maps {
                readonly: HashMap::new(),
                crud: HashMap::new(),
            }),
        }
    }

    pub fn register(&self, repository: Arc<dyn ICRUDRepository>) {
        let type_id = repository.entity_type();
        let readonly = as_readonly(repository.clone());
        let mut maps = self.lock();
        maps.readonly.insert(type_id, readonly);
        maps.crud.insert(type_id, repository);
    }

    pub fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>> {
        let type_id = TypeId::of::<T>();
        let repository = {
            let maps = self.lock();
            maps.crud.get(&type_id).cloned()
        };
        repository.ok_or_else(|| missing(&short_name::<T>()))
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Maps> {
        self.maps.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl ReadonlyRepositoryFactory for RepositoryFactory {
    fn register(&self, repository: Arc<dyn IReadonlyRepository>) {
        let type_id = repository.entity_type();
        self.lock().readonly.insert(type_id, repository);
    }

    fn create<T: 'static>(&self) -> Result<Arc<dyn IReadonlyRepository>> {
        let type_id = TypeId::of::<T>();
        let repository = {
            let maps = self.lock();
            maps.readonly.get(&type_id).cloned()
        };
        repository.ok_or_else(|| missing(&short_name::<T>()))
    }
}

impl CRUDRepositoryFactory for RepositoryFactory {
    fn register(&self, repository: Arc<dyn ICRUDRepository>) {
        RepositoryFactory::register(self, repository);
    }

    fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>> {
        RepositoryFactory::create::<T>(self)
    }
}

fn short_name<T: ?Sized>() -> String {
    std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or(std::any::type_name::<T>())
        .to_string()
}

fn missing(name: &str) -> DatasourceError {
    DatasourceError::message(format!("CRUDRepository \"{name}\" is not registered"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stubs::StubRepository;

    struct Item;

    #[test]
    fn registers_and_creates() {
        let factory = RepositoryFactory::new();
        factory.register(Arc::new(StubRepository::typed::<Item>("memory")));
        let created = factory.create::<Item>().unwrap();

        assert_eq!(created.entity_type(), TypeId::of::<Item>());
        assert_eq!(
            ReadonlyRepositoryFactory::create::<Item>(&factory)
                .unwrap()
                .entity_type(),
            TypeId::of::<Item>()
        );
    }

    #[test]
    fn readonly_registration_is_not_crud() {
        let factory = RepositoryFactory::new();
        ReadonlyRepositoryFactory::register(
            &factory,
            as_readonly(Arc::new(StubRepository::typed::<Item>("memory"))),
        );

        assert!(ReadonlyRepositoryFactory::create::<Item>(&factory).is_ok());
        match factory.create::<Item>() {
            Ok(_) => panic!("expected missing crud"),
            Err(err) => assert!(err.to_string().contains("Item")),
        }
    }

    #[test]
    fn missing_throws() {
        let factory = RepositoryFactory::new();
        match factory.create::<Item>() {
            Ok(_) => panic!("expected missing type"),
            Err(err) => assert_eq!(err.to_string(), "CRUDRepository \"Item\" is not registered"),
        }
    }
}
