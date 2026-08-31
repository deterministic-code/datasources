use crate::connection::Connection;
use crate::converters::{RowTypeConverter, TypeConverter};
use crate::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct TableRepositoryBuilderOptions {
    pub connection: Arc<dyn Connection>,
    pub ids: Vec<String>,
    pub assign_identity_on_add: bool,
    pub table: String,
    pub mapping: Option<RowTypeConverter>,
    pub types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl TableRepositoryBuilderOptions {
    pub fn with_factory(connection: Arc<dyn Connection>) -> Self {
        Self {
            connection,
            ids: vec!["id".into()],
            assign_identity_on_add: true,
            table: "items".into(),
            mapping: None,
            types: HashMap::new(),
            fields: HashMap::new(),
        }
    }
}
