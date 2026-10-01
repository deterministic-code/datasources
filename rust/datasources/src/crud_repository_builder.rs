use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::crud_repository::CRUDRepository;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::crud_repository_statements::CRUDRepositoryStatements;
use crate::readonly_repository_builder::assemble;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::result::Result;
use std::sync::Arc;

pub struct CRUDRepositoryBuilder;

pub(crate) fn write_statements(read: &ReadonlyRepositoryStatements) -> CRUDRepositoryStatements {
    CRUDRepositoryStatements::new(
        read.table().to_string(),
        read.read_columns().to_vec(),
        read.mappings(),
    )
}

impl CRUDRepositoryBuilder {
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
        CRUDRepository::new(connection, options)
    }
}
