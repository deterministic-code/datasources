use crate::connection::Connection;
use crate::converters::{TypeConverter, ValueConverter};
use std::collections::HashMap;
use std::sync::Arc;

pub struct TableRepositoryBuilderOptions {
    pub connection: Arc<dyn Connection>,
    pub ids: Vec<String>,
    pub assign_identity_on_add: bool,
    pub table: String,
    pub mapping: Option<TypeConverter>,
    pub types: HashMap<String, ValueConverter>,
    pub fields: HashMap<String, ValueConverter>,
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
