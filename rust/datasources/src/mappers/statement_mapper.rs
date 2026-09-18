use super::{Mapper, TransformMapper};
use crate::row::Row;
use std::sync::Arc;

pub struct StatementMapper {
    pub entity: TransformMapper<String, String>,
    pub columns: TransformMapper<String, String>,
    pub query: Option<Arc<TransformMapper<String, String>>>,
    pub insert: Option<Arc<TransformMapper<String, String>>>,
    pub update: Option<Arc<TransformMapper<String, String>>>,
    pub delete: Option<Arc<TransformMapper<String, String>>>,
    pub stored_procedure: Option<Arc<TransformMapper<String, String>>>,
    pub rows: TransformMapper<Row, Row>,
}

impl StatementMapper {
    pub fn identity(_source: impl Into<String>, _destination: impl Into<String>) -> Self {
        Self {
            entity: TransformMapper::new(|name| name, |name| name),
            columns: TransformMapper::new(|name| name, |name| name),
            query: None,
            insert: None,
            update: None,
            delete: None,
            stored_procedure: None,
            rows: TransformMapper::new(|row| row, |row| row),
        }
    }

    pub fn map_sql(mapper: &Option<Arc<TransformMapper<String, String>>>, text: String) -> String {
        match mapper {
            Some(map) => Mapper::map(map.as_ref(), text),
            None => text,
        }
    }
}

impl Mapper<Row, Row> for StatementMapper {
    fn map(&self, value: Row) -> Row {
        Mapper::map(&self.rows, value)
    }

    fn reverse_map(&self, value: Row) -> Row {
        Mapper::reverse_map(&self.rows, value)
    }
}
