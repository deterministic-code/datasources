use crate::mappers::TransformMapper;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::row::Row;
use crate::value::Value;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct CRUDRepositoryOptions {
    pub ids: Vec<String>,
    pub insert: Option<Arc<TransformMapper<String, String>>>,
    pub update: Option<Arc<TransformMapper<String, String>>>,
    pub delete: Option<Arc<TransformMapper<String, String>>>,
    pub return_: Option<Arc<TransformMapper<String, String>>>,
    pub create: TransformMapper<Value, Row>,
    pub read: ReadonlyRepositoryOptions,
}

impl Default for CRUDRepositoryOptions {
    fn default() -> Self {
        Self {
            ids: vec!["id".into()],
            insert: None,
            update: None,
            delete: None,
            return_: None,
            create: TransformMapper::new(
                |value| {
                    let mut row = Row::new();
                    if value != Value::Null {
                        row.insert("id".into(), value);
                    }
                    row
                },
                |_| Value::Null,
            ),
            read: ReadonlyRepositoryOptions::default(),
        }
    }
}

impl Deref for CRUDRepositoryOptions {
    type Target = ReadonlyRepositoryOptions;

    fn deref(&self) -> &Self::Target {
        &self.read
    }
}

impl DerefMut for CRUDRepositoryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.read
    }
}
