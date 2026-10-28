use crate::connection::Connection;
use crate::datasource_error::DatasourceError;
use crate::entity::Entity;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::repository_access::RepositoryAccess;
use crate::result::Result;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Binder = Arc<dyn Fn(&RepositoryFactory) + Send + Sync>;

fn binders() -> &'static Mutex<Vec<Binder>> {
    static BINDERS: Mutex<Vec<Binder>> = Mutex::new(Vec::new());
    &BINDERS
}

struct Named {
    create: Arc<dyn Fn(Arc<dyn Connection>) -> Arc<dyn Any + Send + Sync> + Send + Sync>,
    create_readonly: Arc<dyn Fn(Arc<dyn Connection>) -> Arc<dyn Any + Send + Sync> + Send + Sync>,
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
        let factory = Self {
            maps: Arc::new(Mutex::new(Maps {
                readonly: HashMap::new(),
                crud: HashMap::new(),
                named: HashMap::new(),
            })),
        };
        factory.apply_bound();
        factory
    }

    pub fn bind(register: impl Fn(&RepositoryFactory) + Send + Sync + 'static) {
        binders()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(Arc::new(register));
    }

    pub fn reset_binders() {
        binders().lock().unwrap_or_else(|e| e.into_inner()).clear();
    }

    fn apply_bound(&self) {
        let snapshot = binders().lock().unwrap_or_else(|e| e.into_inner()).clone();
        for register in snapshot {
            register(self);
        }
    }

    pub fn register(&self, repository: Arc<dyn ICRUDRepository>) {
        let type_id = repository.entity_type();
        let readonly = as_readonly(repository.clone());
        let mut maps = self.lock();
        maps.readonly.insert(type_id, readonly);
        maps.crud.insert(type_id, repository);
    }

    pub fn register_named<T: Entity>(
        &self,
        type_name: impl Into<String>,
        plural_name: impl Into<String>,
        create: impl Fn(Arc<dyn Connection>) -> Arc<dyn ICRUDRepository<T>> + Send + Sync + 'static,
        create_readonly: impl Fn(Arc<dyn Connection>) -> Arc<dyn IReadonlyRepository<T>>
            + Send
            + Sync
            + 'static,
    ) {
        let named = Named {
            create: Arc::new(move |connection| Arc::new(create(connection))),
            create_readonly: Arc::new(move |connection| Arc::new(create_readonly(connection))),
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

    pub fn create_named<T: Entity>(
        &self,
        name: &str,
        connection: Arc<dyn Connection>,
    ) -> Result<Arc<dyn ICRUDRepository<T>>> {
        self.apply_bound();
        let create = {
            let maps = self.lock();
            maps.named.get(name).map(|named| Arc::clone(&named.create))
        };
        let stored = (create.ok_or_else(|| missing_named(name))?)(connection);
        stored
            .downcast::<Arc<dyn ICRUDRepository<T>>>()
            .map(|repo| (*repo).clone())
            .map_err(|_| missing_named(name))
    }

    pub fn create_readonly_named<T: Entity>(
        &self,
        name: &str,
        connection: Arc<dyn Connection>,
    ) -> Result<Arc<dyn IReadonlyRepository<T>>> {
        self.apply_bound();
        let create = {
            let maps = self.lock();
            maps.named
                .get(name)
                .map(|named| Arc::clone(&named.create_readonly))
        };
        let stored = (create.ok_or_else(|| missing_named(name))?)(connection);
        stored
            .downcast::<Arc<dyn IReadonlyRepository<T>>>()
            .map(|repo| (*repo).clone())
            .map_err(|_| missing_named(name))
    }

    pub fn access(&self, name: &str, connection: Arc<dyn Connection>) -> RepositoryAccess {
        self.apply_bound();
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
    use crate::row::Row;
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
            .create_named::<Row>("contacts", Arc::clone(&connection))
            .unwrap();
        assert_eq!(created.entity_type(), TypeId::of::<Item>());
        let readonly = factory
            .create_readonly_named::<Row>("Contact", connection)
            .unwrap();
        assert_eq!(readonly.entity_type(), TypeId::of::<Item>());
    }

    #[test]
    fn bind_registers_on_construct() {
        RepositoryFactory::reset_binders();
        let crud = Arc::new(StubRepository::typed::<Item>("crud"));
        let read = as_readonly(Arc::new(StubRepository::typed::<Item>("read")));
        RepositoryFactory::bind(move |factory| {
            factory.register_named(
                "Contact",
                "contacts",
                {
                    let crud = Arc::clone(&crud);
                    move |_| Arc::clone(&crud) as Arc<dyn ICRUDRepository>
                },
                {
                    let read = Arc::clone(&read);
                    move |_| Arc::clone(&read)
                },
            );
        });
        let factory = RepositoryFactory::new();
        let connection: Arc<dyn Connection> = Arc::new(StubConnection::new("memory"));
        let created = factory
            .create_named::<Row>("contacts", Arc::clone(&connection))
            .unwrap();
        assert_eq!(created.entity_type(), TypeId::of::<Item>());
        RepositoryFactory::reset_binders();
    }

    #[test]
    fn bind_registers_on_access() {
        RepositoryFactory::reset_binders();
        let crud = Arc::new(StubRepository::typed::<Item>("crud"));
        let read = as_readonly(Arc::new(StubRepository::typed::<Item>("read")));
        RepositoryFactory::bind(move |factory| {
            factory.register_named(
                "Contact",
                "contacts",
                {
                    let crud = Arc::clone(&crud);
                    move |_| Arc::clone(&crud) as Arc<dyn ICRUDRepository>
                },
                {
                    let read = Arc::clone(&read);
                    move |_| Arc::clone(&read)
                },
            );
        });
        let factory = RepositoryFactory::new();
        let connection: Arc<dyn Connection> = Arc::new(StubConnection::new("memory"));
        let created = factory
            .access("contacts", Arc::clone(&connection))
            .create()
            .unwrap();
        assert_eq!(created.entity_type(), TypeId::of::<Item>());
        RepositoryFactory::reset_binders();
    }

    #[test]
    fn missing_name_throws() {
        let factory = RepositoryFactory::new();
        let connection: Arc<dyn Connection> = Arc::new(StubConnection::new("memory"));
        match factory.create_named::<Row>("Contacts", connection) {
            Ok(_) => panic!("expected missing name"),
            Err(err) => {
                assert_eq!(err.to_string(), "Repository \"Contacts\" is not registered")
            }
        }
    }
}
