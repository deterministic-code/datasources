use crate::converters::{RowTypeConverter, TypeConverter};
use crate::mappers::{IdentityMapper, Mapper, TransformMapper};
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;

pub struct ReadonlyRepositoryOptions {
    pub entity: String,
    pub entity_mapper: Box<dyn Mapper<String, String> + Send + Sync>,
    pub column_mappings: Box<dyn Mapper<String, String> + Send + Sync>,
    pub query: Option<std::sync::Arc<TransformMapper<String, String>>>,
    pub rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>,
    pub row_converter: Option<RowTypeConverter>,
    pub row_type_converters: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl Default for ReadonlyRepositoryOptions {
    fn default() -> Self {
        Self {
            entity: "items".into(),
            entity_mapper: Box::new(IdentityMapper::new()),
            column_mappings: Box::new(IdentityMapper::new()),
            query: None,
            rows_mapper: Box::new(IdentityMapper::new()),
            row_converter: None,
            row_type_converters: HashMap::new(),
        }
    }
}
