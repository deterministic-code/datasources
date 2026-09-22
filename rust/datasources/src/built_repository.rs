use crate::connection::Connection;
use crate::converters::RowTypeConverter;
use crate::crud_mapper::CRUDMapper;
use crate::table_repository_statements::RepositoryStatements;
use std::sync::Arc;

pub struct BuiltRepository {
    pub connection: Arc<dyn Connection>,
    pub row_converter: RowTypeConverter,
    pub crud_mapper: Arc<CRUDMapper>,
    pub statements: RepositoryStatements,
    pub ids: Vec<String>,
}
