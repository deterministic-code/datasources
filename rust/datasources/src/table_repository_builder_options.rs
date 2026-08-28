use crate::connection::Connection;
use crate::converters::{FieldConverter, TypeMapping};
use std::collections::HashMap;
use std::sync::Arc;

pub struct TableRepositoryBuilderOptions {
    pub connection: Arc<dyn Connection>,
    pub ids: Vec<String>,
    pub assign_identity_on_add: bool,
    pub converters: HashMap<String, FieldConverter>,
    pub mapping: Option<TypeMapping>,
}

impl TableRepositoryBuilderOptions {
    pub fn with_factory(connection: Arc<dyn Connection>) -> Self {
        Self {
            connection,
            ids: vec!["id".into()],
            assign_identity_on_add: true,
            converters: HashMap::new(),
            mapping: None,
        }
    }
}
