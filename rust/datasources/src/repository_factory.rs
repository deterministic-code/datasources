use crate::connection::Connection;
use crate::datasource_error::DatasourceError;
use crate::repository::{as_readonly, ReadonlyRepository, CRUDRepository};
use crate::result::Result;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type CRUDFn = Arc<dyn Fn(Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> + Send + Sync>;
type ReadonlyFn =
    Arc<dyn Fn(Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository> + Send + Sync>;

struct Maps {
    readonly_by_name: HashMap<String, ReadonlyFn>,
    readonly_by_type: HashMap<TypeId, ReadonlyFn>,
    crud_by_name: HashMap<String, CRUDFn>,
    crud_by_type: HashMap<TypeId, CRUDFn>,
}

pub trait ReadonlyRepositoryFactory: Send + Sync {
    fn register(&self, name: &str, factory: ReadonlyFn);
    fn register_type<T: 'static>(&self, factory: ReadonlyFn);
    fn create(
        &self,
        name: &str,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn ReadonlyRepository>>;
    fn create_type<T: 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn ReadonlyRepository>>;
}

pub trait CRUDRepositoryFactory: Send + Sync {
    fn register(&self, name: &str, factory: CRUDFn);
    fn register_type<T: 'static>(&self, factory: CRUDFn);
    fn create(
        &self,
        name: &str,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn CRUDRepository>>;
    fn create_type<T: 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn CRUDRepository>>;
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
                readonly_by_name: HashMap::new(),
                readonly_by_type: HashMap::new(),
                crud_by_name: HashMap::new(),
                crud_by_type: HashMap::new(),
            }),
        }
    }

    pub fn register(
        &self,
        name: &str,
        factory: impl Fn(Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static,
    ) {
        self.put_crud(name, None, Arc::new(factory));
    }

    pub fn register_type<T: 'static>(
        &self,
        factory: impl Fn(Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static,
    ) {
        self.put_crud(
            &short_name::<T>(),
            Some(TypeId::of::<T>()),
            Arc::new(factory),
        );
    }

    pub fn register_readonly(
        &self,
        name: &str,
        factory: impl Fn(Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository>
            + Send
            + Sync
            + 'static,
    ) {
        self.put_readonly(name, None, Arc::new(factory));
    }

    pub fn register_readonly_type<T: 'static>(
        &self,
        factory: impl Fn(Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository>
            + Send
            + Sync
            + 'static,
    ) {
        self.put_readonly(
            &short_name::<T>(),
            Some(TypeId::of::<T>()),
            Arc::new(factory),
        );
    }

    pub fn create(
        &self,
        name: &str,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn CRUDRepository>> {
        let factory = {
            let maps = self.lock();
            maps.crud_by_name.get(name).cloned()
        };
        match factory {
            Some(factory) => Ok(factory(connection)),
            None => Err(missing(name)),
        }
    }

    pub fn create_type<T: 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn CRUDRepository>> {
        let factory = {
            let maps = self.lock();
            maps.crud_by_type
                .get(&TypeId::of::<T>())
                .cloned()
                .or_else(|| maps.crud_by_name.get(&short_name::<T>()).cloned())
        };
        match factory {
            Some(factory) => Ok(factory(connection)),
            None => Err(missing(&short_name::<T>())),
        }
    }

    pub fn create_readonly(
        &self,
        name: &str,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn ReadonlyRepository>> {
        let factory = {
            let maps = self.lock();
            maps.readonly_by_name.get(name).cloned()
        };
        match factory {
            Some(factory) => Ok(factory(connection)),
            None => Err(missing(name)),
        }
    }

    pub fn create_readonly_type<T: 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn ReadonlyRepository>> {
        let factory = {
            let maps = self.lock();
            maps.readonly_by_type
                .get(&TypeId::of::<T>())
                .cloned()
                .or_else(|| maps.readonly_by_name.get(&short_name::<T>()).cloned())
        };
        match factory {
            Some(factory) => Ok(factory(connection)),
            None => Err(missing(&short_name::<T>())),
        }
    }

    fn put_crud(&self, name: &str, type_id: Option<TypeId>, factory: CRUDFn) {
        let crud = factory.clone();
        let readonly: ReadonlyFn = Arc::new(move |connection| as_readonly(crud(connection)));
        let mut maps = self.lock();
        maps.crud_by_name.insert(name.to_string(), factory.clone());
        maps.readonly_by_name
            .insert(name.to_string(), readonly.clone());
        if let Some(type_id) = type_id {
            maps.crud_by_type.insert(type_id, factory);
            maps.readonly_by_type.insert(type_id, readonly);
        }
    }

    fn put_readonly(&self, name: &str, type_id: Option<TypeId>, factory: ReadonlyFn) {
        let mut maps = self.lock();
        maps.readonly_by_name
            .insert(name.to_string(), factory.clone());
        if let Some(type_id) = type_id {
            maps.readonly_by_type.insert(type_id, factory);
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Maps> {
        self.maps.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl ReadonlyRepositoryFactory for RepositoryFactory {
    fn register(&self, name: &str, factory: ReadonlyFn) {
        self.put_readonly(name, None, factory);
    }

    fn register_type<T: 'static>(&self, factory: ReadonlyFn) {
        self.put_readonly(&short_name::<T>(), Some(TypeId::of::<T>()), factory);
    }

    fn create(
        &self,
        name: &str,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn ReadonlyRepository>> {
        self.create_readonly(name, connection)
    }

    fn create_type<T: 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn ReadonlyRepository>> {
        self.create_readonly_type::<T>(connection)
    }
}

impl CRUDRepositoryFactory for RepositoryFactory {
    fn register(&self, name: &str, factory: CRUDFn) {
        self.put_crud(name, None, factory);
    }

    fn register_type<T: 'static>(&self, factory: CRUDFn) {
        self.put_crud(&short_name::<T>(), Some(TypeId::of::<T>()), factory);
    }

    fn create(
        &self,
        name: &str,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn CRUDRepository>> {
        RepositoryFactory::create(self, name, connection)
    }

    fn create_type<T: 'static>(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Result<Arc<dyn CRUDRepository>> {
        RepositoryFactory::create_type::<T>(self, connection)
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
    use crate::stubs::{StubConnection, StubRepository};
    use std::sync::Mutex;

    struct Item;

    #[test]
    fn registers_and_creates_by_type_and_name() {
        let factory = RepositoryFactory::new();
        factory.register_type::<Item>(|_| Arc::new(StubRepository::new("memory")));

        assert!(factory.create_type::<Item>(None).is_ok());
        assert!(factory.create("Item", None).is_ok());
        assert!(factory.create_readonly("Item", None).is_ok());
        assert!(factory.create_readonly_type::<Item>(None).is_ok());
    }

    #[test]
    fn registers_by_name_and_type() {
        let factory = RepositoryFactory::new();
        factory.register("people", |_| Arc::new(StubRepository::new("memory")));
        factory.register_type::<Item>(|_| Arc::new(StubRepository::new("json")));

        assert!(factory.create("people", None).is_ok());
        assert!(factory.create_type::<Item>(None).is_ok());
    }

    #[test]
    fn forwards_connection() {
        let factory = RepositoryFactory::new();
        let seen = Arc::new(Mutex::new(false));
        let seen_flag = seen.clone();
        factory.register_type::<Item>(move |connection| {
            *seen_flag.lock().unwrap() = connection.is_some();
            Arc::new(StubRepository::new("memory"))
        });
        let connection: Arc<dyn Connection> = Arc::new(StubConnection::new("memory"));
        factory.create_type::<Item>(Some(connection)).unwrap();
        assert!(*seen.lock().unwrap());
    }

    #[test]
    fn readonly_registration_is_not_crud() {
        let factory = RepositoryFactory::new();
        factory.register_readonly_type::<Item>(|_| {
            as_readonly(Arc::new(StubRepository::new("memory")))
        });

        assert!(factory.create_readonly_type::<Item>(None).is_ok());
        assert!(factory.create_type::<Item>(None).is_err());
        match factory.create("Item", None) {
            Ok(_) => panic!("expected missing crud"),
            Err(err) => assert!(err.to_string().contains("Item")),
        }
    }

    #[test]
    fn missing_throws() {
        let factory = RepositoryFactory::new();
        match factory.create_type::<Item>(None) {
            Ok(_) => panic!("expected missing type"),
            Err(err) => assert_eq!(err.to_string(), "CRUDRepository \"Item\" is not registered"),
        }
        assert!(factory.create("missing", None).is_err());
    }
}
