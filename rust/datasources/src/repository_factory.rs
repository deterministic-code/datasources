use crate::connection::Connection;
use crate::datasource_error::DatasourceError;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::repository_access::RepositoryAccess;
use crate::result::Result;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Named {
    create: Arc<dyn Fn(Arc<dyn Connection>) -> Arc<dyn ICRUDRepository> + Send + Sync>,
    create_readonly: Arc<dyn Fn(Arc<dyn Connection>) -> Arc<dyn IReadonlyRepository> + Send + Sync>,
}

struct Maps {
    readonly: HashMap<TypeId, Arc<dyn IReadonlyRepository>>,
    crud: HashMap<TypeId, Arc<dyn ICRUDRepository>>,
    named: HashMap<String, Named>,
}

pub trait ReadonlyRepositoryFactory: Send + Sync {
    fn register(&self, repository: Arc<dyn IReadonlyRepository>);
    fn create<T: 'static>(&self) -> Result<Arc<dyn IReadonlyRepository>>;
}

pub trait CRUDRepositoryFactory: Send + Sync {
    fn register(&self, repository: Arc<dyn ICRUDRepository>);
    fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>>;
}

#[derive(Clone)]
pub struct RepositoryFactory {
    maps: Arc<Mutex<Maps>>,
}

impl Default for RepositoryFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl RepositoryFactory {
    pub fn new() -> Self {
        Self {
            maps: Arc::new(Mutex::new(Maps {
                readonly: HashMap::new(),
                crud: HashMap::new(),
                named: HashMap::new(),
            })),
        }
    }

    pub fn register(&self, repository: Arc<dyn ICRUDRepository>) {
        let type_id = repository.entity_type();
        let readonly = as_readonly(repository.clone());
        let mut maps = self.lock();
        maps.readonly.insert(type_id, readonly);
        maps.crud.insert(type_id, repository);
    }

    pub fn register_named(
        &self,
        type_name: impl Into<String>,
        plural_name: impl Into<String>,
        create: impl Fn(Arc<dyn Connection>) -> Arc<dyn ICRUDRepository> + Send + Sync + 'static,
        create_readonly: impl Fn(Arc<dyn Connection>) -> Arc<dyn IReadonlyRepository>
            + Send
            + Sync
            + 'static,
    ) {
        let named = Named {
            create: Arc::new(create),
            create_readonly: Arc::new(create_readonly),
        };
        let mut maps = self.lock();
        maps.named.insert(type_name.into(), named.clone_entry());
        maps.named.insert(plural_name.into(), named);
    }

    pub fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>> {
        let type_id = TypeId::of::<T>();
        let repository = {
            let maps = self.lock();
            maps.crud.get(&type_id).cloned()
        };
        repository.ok_or_else(|| missing(&short_name::<T>()))
    }

    pub fn create_named(
        &self,
        name: &str,
        connection: Arc<dyn Connection>,
    ) -> Result<Arc<dyn ICRUDRepository>> {
        let create = {
            let maps = self.lock();
            maps.named.get(name).map(|named| Arc::clone(&named.create))
        };
        Ok((create.ok_or_else(|| missing_named(name))?)(connection))
    }

    pub fn create_readonly_named(
        &self,
        name: &str,
        connection: Arc<dyn Connection>,
    ) -> Result<Arc<dyn IReadonlyRepository>> {
        let create = {
            let maps = self.lock();
            maps.named
                .get(name)
                .map(|named| Arc::clone(&named.create_readonly))
        };
        Ok((create.ok_or_else(|| missing_named(name))?)(connection))
    }

    pub fn access(&self, name: &str, connection: Arc<dyn Connection>) -> RepositoryAccess {
        RepositoryAccess::new(self.clone(), name, connection)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Maps> {
        self.maps.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl Named {
    fn clone_entry(&self) -> Self {
        Self {
            create: Arc::clone(&self.create),
            create_readonly: Arc::clone(&self.create_readonly),
        }
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

fn missing_named(name: &str) -> DatasourceError {
    DatasourceError::message(format!("Repository \"{name}\" is not registered"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stubs::{StubConnection, StubRepository};

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

    #[test]
    fn registers_and_creates_by_name() {
        let factory = RepositoryFactory::new();
        factory.register_named(
            "Contact",
            "contacts",
            |_| Arc::new(StubRepository::typed::<Item>("crud")),
            |_| as_readonly(Arc::new(StubRepository::typed::<Item>("read"))),
        );
        let connection: Arc<dyn Connection> = Arc::new(StubConnection::new("memory"));
        let created = factory
            .create_named("contacts", Arc::clone(&connection))
            .unwrap();
        assert_eq!(created.entity_type(), TypeId::of::<Item>());
        let readonly = factory
            .create_readonly_named("Contact", connection)
            .unwrap();
        assert_eq!(readonly.entity_type(), TypeId::of::<Item>());
    }

    #[test]
    fn missing_name_throws() {
        let factory = RepositoryFactory::new();
        let connection: Arc<dyn Connection> = Arc::new(StubConnection::new("memory"));
        match factory.create_named("Contacts", connection) {
            Ok(_) => panic!("expected missing name"),
            Err(err) => {
                assert_eq!(err.to_string(), "Repository \"Contacts\" is not registered")
            }
        }
    }
}
