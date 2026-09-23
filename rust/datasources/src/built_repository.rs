use crate::connection::Connection;
use crate::converters::RowTypeConverter;
use crate::mappers::{Mapper, TransformMapper};
use crate::row::Row;
use crate::table_repository_statements::RepositoryStatements;
use crate::value::Value;
use std::sync::Arc;

pub struct BuiltRepository {
    pub connection: Arc<dyn Connection>,
    pub row_converter: RowTypeConverter,
    pub rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>,
    pub statements: RepositoryStatements,
    pub ids: Vec<String>,
    pub create: Option<TransformMapper<Value, Row>>,
}
