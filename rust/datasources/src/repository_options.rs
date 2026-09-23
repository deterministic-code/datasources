use crate::converters::{RowTypeConverter, TypeConverter};
use crate::crud_mapper_options::CRUDMapperOptions;
use crate::value::Value;
use std::collections::HashMap;

pub struct RepositoryOptions {
    pub ids: Vec<String>,
    pub entity: String,
    pub row_converter: Option<RowTypeConverter>,
    pub crud_mapper: Option<CRUDMapperOptions>,
    pub row_type_converters: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl Default for RepositoryOptions {
    fn default() -> Self {
        Self {
            ids: vec!["id".into()],
            entity: "items".into(),
            row_converter: None,
            crud_mapper: None,
            row_type_converters: HashMap::new(),
        }
    }
}
