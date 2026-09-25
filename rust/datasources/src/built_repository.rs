use crate::connection::Connection;
use crate::converters::RowTypeConverter;
use crate::mappers::{Mapper, TransformMapper};
use crate::row::Row;
use crate::table_repository_statements::RepositoryStatements;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use std::sync::Arc;

pub struct BuiltRepository {
    pub connection: Arc<dyn Connection>,
    pub row_converter: RowTypeConverter,
    pub rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>,
    pub result: Option<Box<dyn Mapper<Vec<Row>, Vec<Row>> + Send + Sync>>,
    pub column_mappings: Box<dyn Mapper<String, String> + Send + Sync>,
    pub statements: RepositoryStatements,
    pub ids: Vec<String>,
    pub create: Option<TransformMapper<Value, Row>>,
    pub write: Option<Arc<dyn WriteEnrichmentStrategy>>,
}
