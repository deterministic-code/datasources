use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::crud_repository::CRUDRepository;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::crud_repository_statements::CRUDRepositoryStatements;
use crate::readonly_repository_builder::assemble;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::repository_builder_options_factory::RepositoryBuilderOptionsFactory;
use crate::result::Result;
use crate::row::Row;
use std::sync::Arc;

#[derive(Default)]
pub struct CRUDRepositoryBuilder {
    factory: RepositoryBuilderOptionsFactory,
}

pub(crate) fn write_statements(read: &ReadonlyRepositoryStatements) -> CRUDRepositoryStatements {
    CRUDRepositoryStatements::new(
        read.table().to_string(),
        read.read_columns().to_vec(),
        read.mappings(),
    )
}

impl CRUDRepositoryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(mut self, options: CRUDRepositoryOptions) -> Self {
        self.factory.add(options);
        self
    }

    pub fn add_for(mut self, dialect: impl Into<String>, options: CRUDRepositoryOptions) -> Self {
        self.factory.add_for(dialect, options);
        self
    }

    pub fn for_connection(&self, connection: &dyn Connection) -> Result<CRUDRepositoryOptions> {
        self.factory.for_provider(connection.provider())
    }

    pub fn parts(
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
    ) -> Result<BuiltRepository> {
        let ids = options.ids;
        assemble(connection, options.read, ids)
    }

    pub fn build(
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
    ) -> Result<CRUDRepository> {
        CRUDRepository::<Row>::new(connection, options)
    }
}
