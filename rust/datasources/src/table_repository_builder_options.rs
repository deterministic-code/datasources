use crate::connection::Connection;
use crate::converters::{RowTypeConverter, TypeConverter};
use crate::crud_mapper::CRUDMapper;
use crate::mappers::TransformMapper;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RepositoryBuilderOptions {
    pub connection: Arc<dyn Connection>,
    pub ids: Vec<String>,
    pub entity: String,
    pub mapping: Option<RowTypeConverter>,
    pub mapper: Option<TransformMapper<Row, Row>>,
    pub crud_mapper: Option<CRUDMapper>,
    pub types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl RepositoryBuilderOptions {
    pub fn with_factory(connection: Arc<dyn Connection>) -> Self {
        Self {
            connection,
            ids: vec!["id".into()],
            entity: "items".into(),
            mapping: None,
            mapper: None,
            crud_mapper: None,
            types: HashMap::new(),
            fields: HashMap::new(),
        }
    }
}
