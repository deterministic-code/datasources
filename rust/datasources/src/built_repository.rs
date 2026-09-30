use crate::connection::Connection;
use crate::converters::RowTypeConverter;
use crate::mappers::Mapper;
use crate::row::Row;
use crate::table_repository_statements::RepositoryStatements;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use std::sync::Arc;

pub struct BuiltRepository {
    pub connection: Arc<dyn Connection>,
    pub row_converter: Option<RowTypeConverter>,
    pub rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>,
    pub column_mappings: Box<dyn Mapper<String, String> + Send + Sync>,
    pub statements: RepositoryStatements,
    pub ids: Vec<String>,
    pub write: Option<Arc<dyn WriteEnrichmentStrategy>>,
}
