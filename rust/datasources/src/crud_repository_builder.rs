use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::crud_repository::CRUDRepository;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::readonly_repository_builder::assemble;
use crate::result::Result;
use std::sync::Arc;

pub struct CRUDRepositoryBuilder;

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
