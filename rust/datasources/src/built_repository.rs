use crate::connection::Connection;
use crate::converters::RowTypeConverter;
use crate::mappers::{NameSourceDestinationMapper, TransformMapper};
use crate::row::Row;
use crate::table_repository_statements::TableRepositoryStatements;
use std::sync::Arc;

pub struct BuiltRepository {
    pub connection: Arc<dyn Connection>,
    pub row_converter: RowTypeConverter,
    pub mapper: TransformMapper<Row, Row>,
    pub mapping: NameSourceDestinationMapper,
    pub statements: TableRepositoryStatements,
    pub ids: Vec<String>,
    pub assign_identity_on_add: bool,
}
