use crate::repository::IReadonlyRepository;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use crate::value::Value;
use std::sync::{Arc, Mutex};

pub struct RepositoryTreeNode {
    pub value: Arc<dyn IReadonlyRepository>,
    pub property_name: String,
    pub enrichments: Arc<Mutex<Vec<StoredEnrichment>>>,
}

impl RepositoryTreeNode {
    pub fn new(
        value: Arc<dyn IReadonlyRepository>,
        property_name: impl Into<String>,
        enrichments: Arc<Mutex<Vec<StoredEnrichment>>>,
    ) -> Self {
        Self {
            value,
            property_name: property_name.into(),
            enrichments,
        }
    }

    pub fn key_of(key: &Value) -> String {
        key.wire_text()
    }
}

pub fn source_key(parent: &Row, name: &str, source: Option<&Row>) -> Option<Value> {
    if let Some(source) = source {
        if let Some(cell) = source.get(name) {
            if *cell != Value::Null {
                return Some(cell.clone());
            }
        }
    }
    parent
        .get(name)
        .filter(|value| **value != Value::Null)
        .cloned()
}

pub(crate) fn source_at<'a>(sources: Option<&'a [Option<Row>]>, index: usize) -> Option<&'a Row> {
    sources
        .and_then(|rows| rows.get(index))
        .and_then(|row| row.as_ref())
}

pub fn encode_rows(rows: &[Row]) -> Value {
    Value::Text(
        rows.iter()
            .map(encode_row_text)
            .collect::<Vec<_>>()
            .join("|"),
    )
}

pub(crate) fn encode_row(row: &Row) -> Value {
    Value::Text(encode_row_text(row))
}

fn encode_row_text(row: &Row) -> String {
    let mut keys: Vec<_> = row.keys().cloned().collect();
    keys.sort();
    keys.into_iter()
        .filter_map(|key| {
            row.get(&key)
                .map(|value| format!("{key}={}", value.wire_text()))
        })
        .collect::<Vec<_>>()
        .join(",")
}
