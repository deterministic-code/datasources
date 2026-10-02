use crate::converters::{RowTypeConverter, TypeConverter};
use crate::mappers::Mapper;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ReadonlyRepositoryMappingOptions {
    pub entity: String,
    pub entity_mapper: Option<Arc<dyn Mapper<String, String> + Send + Sync>>,
    pub property_mappings: Option<Arc<dyn Mapper<String, String> + Send + Sync>>,
    pub row_converter: Option<RowTypeConverter>,
    pub object_converter: Option<Box<dyn TypeConverter<Row, Row> + Send + Sync>>,
    pub row_type_converters: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl Default for ReadonlyRepositoryMappingOptions {
    fn default() -> Self {
        Self {
            entity: "items".into(),
            entity_mapper: None,
            property_mappings: None,
            row_converter: None,
            object_converter: None,
            row_type_converters: HashMap::new(),
        }
    }
}
