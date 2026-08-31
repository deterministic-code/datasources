use crate::connection::Connection;
use crate::converters::RowTypeConverter;
use crate::mappers::NameSourceDestinationMapper;
use crate::table_repository_statements::TableRepositoryStatements;
use std::sync::Arc;

pub struct BuiltRepository {
    pub connection: Arc<dyn Connection>,
    pub converters: RowTypeConverter,
    pub mapping: NameSourceDestinationMapper,
    pub statements: TableRepositoryStatements,
    pub ids: Vec<String>,
    pub assign_identity_on_add: bool,
}
