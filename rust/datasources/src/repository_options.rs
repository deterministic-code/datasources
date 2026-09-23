use crate::converters::{RowTypeConverter, TypeConverter};
use crate::crud_mapper::CRUDMapper;
use crate::mappers::TransformMapper;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;

pub struct RepositoryOptions {
    pub ids: Vec<String>,
    pub entity: String,
    pub mapping: Option<RowTypeConverter>,
    pub mapper: Option<TransformMapper<Row, Row>>,
    pub crud_mapper: Option<CRUDMapper>,
    pub row_type_converters: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl Default for RepositoryOptions {
    fn default() -> Self {
        Self {
            ids: vec!["id".into()],
            entity: "items".into(),
            mapping: None,
            mapper: None,
            crud_mapper: None,
            row_type_converters: HashMap::new(),
            fields: HashMap::new(),
        }
    }
}
