use crate::connection::Connection;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::repository_factory::RepositoryFactory;
use crate::result::Result;
use std::sync::Arc;

pub struct RepositoryAccess {
    factory: RepositoryFactory,
    name: String,
    connection: Arc<dyn Connection>,
}

impl RepositoryAccess {
    pub fn new(
        factory: RepositoryFactory,
        name: impl Into<String>,
        connection: Arc<dyn Connection>,
    ) -> Self {
        Self {
            factory,
            name: name.into(),
            connection,
        }
    }

    pub fn create(&self) -> Result<Arc<dyn ICRUDRepository>> {
        self.factory
            .create_named(&self.name, Arc::clone(&self.connection))
    }

    pub fn create_readonly(&self) -> Result<Arc<dyn IReadonlyRepository>> {
        self.factory
            .create_readonly_named(&self.name, Arc::clone(&self.connection))
    }
}
