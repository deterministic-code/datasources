use crate::connection::Connection;
use crate::converters::RowTypeConverter;
use crate::mappers::StatementMapper;
use crate::table_repository_statements::RepositoryStatements;
use std::sync::Arc;

pub struct BuiltRepository {
    pub connection: Arc<dyn Connection>,
    pub row_converter: RowTypeConverter,
    pub statement_mapper: Arc<StatementMapper>,
    pub statements: RepositoryStatements,
    pub ids: Vec<String>,
    pub assign_identity_on_create: bool,
}
