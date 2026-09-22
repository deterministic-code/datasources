use crate::mappers::{Mapper, TransformMapper};
use crate::row::Row;
use crate::value::Value;
use std::sync::Arc;

pub struct CRUDMapper {
    pub entity: TransformMapper<String, String>,
    pub columns: TransformMapper<String, String>,
    pub query: Option<Arc<TransformMapper<String, String>>>,
    pub insert: Option<Arc<TransformMapper<String, String>>>,
    pub update: Option<Arc<TransformMapper<String, String>>>,
    pub delete: Option<Arc<TransformMapper<String, String>>>,
    pub return_: Option<Arc<TransformMapper<String, String>>>,
    pub rows: TransformMapper<Row, Row>,
    pub create: TransformMapper<Value, Row>,
}

impl CRUDMapper {
    pub fn identity(_source: impl Into<String>, _destination: impl Into<String>) -> Self {
        Self {
            entity: TransformMapper::new(|name| name, |name| name),
            columns: TransformMapper::new(|name| name, |name| name),
            query: None,
            insert: None,
            update: None,
            delete: None,
            return_: None,
            rows: TransformMapper::new(|row| row, |row| row),
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
        }
    }

    pub fn map_sql(mapper: &Option<Arc<TransformMapper<String, String>>>, text: String) -> String {
        match mapper {
            Some(map) => Mapper::map(map.as_ref(), text),
            None => text,
        }
    }
}
