use crate::converters::{RowTypeConverter, TypeConverter};
use crate::mappers::Mapper;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ReadonlyRepositoryMappingOptions {
    pub entity: String,
    pub entity_mapper: Option<Arc<dyn Mapper<String, String> + Send + Sync>>,
    pub property_mappings: Option<Arc<dyn Mapper<String, String> + Send + Sync>>,
    pub read_properties: Vec<String>,
    pub row_converter: Option<RowTypeConverter>,
    pub object_converter: Option<Box<dyn TypeConverter<Row, Row> + Send + Sync>>,
    pub row_type_converters: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub read_enrichment: Option<Arc<dyn ReadonlyEnrichmentStrategy>>,
}

impl Clone for ReadonlyRepositoryMappingOptions {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            entity_mapper: self.entity_mapper.clone(),
            property_mappings: self.property_mappings.clone(),
            read_properties: self.read_properties.clone(),
            row_converter: self.row_converter.clone(),
            object_converter: None,
            row_type_converters: HashMap::new(),
            read_enrichment: self.read_enrichment.clone(),
        }
    }
}

impl Default for ReadonlyRepositoryMappingOptions {
    fn default() -> Self {
        Self {
            entity: "items".into(),
            entity_mapper: None,
            property_mappings: None,
            read_properties: Vec::new(),
            row_converter: None,
            object_converter: None,
            row_type_converters: HashMap::new(),
            read_enrichment: None,
        }
    }
}
