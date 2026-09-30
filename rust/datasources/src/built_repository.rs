use crate::connection::Connection;
use crate::converters::{RowTypeConverter, TypeConverter};
use crate::mappers::Mapper;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::row::Row;
use std::sync::Arc;

pub struct BuiltRepository {
    pub connection: Arc<dyn Connection>,
    pub row_converter: Option<RowTypeConverter>,
    pub object_converter: Box<dyn TypeConverter<Row, Row> + Send + Sync>,
    pub property_mappings: Box<dyn Mapper<String, String> + Send + Sync>,
    pub entity_mapper: Box<dyn Mapper<String, String> + Send + Sync>,
    pub statements: ReadonlyRepositoryStatements,
    pub ids: Vec<String>,
}
