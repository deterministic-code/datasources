use crate::default_write_enrichment_strategy::encode_rows;
use crate::enrichment::Enrichment;
use crate::mappers::{FlatToNestedMapper, MappedRow, MappedValue, Mapper, TypeMap};
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use async_trait::async_trait;
use std::sync::Arc;

pub struct NestedToObjectEnrichmentStrategy {
    nested: FlatToNestedMapper,
}

impl NestedToObjectEnrichmentStrategy {
    pub fn new(type_map: TypeMap) -> Self {
        Self {
            nested: FlatToNestedMapper::new(type_map),
        }
    }
}

#[async_trait]
impl ReadonlyEnrichmentStrategy for NestedToObjectEnrichmentStrategy {
    async fn apply(
        &self,
        _via: Arc<dyn IReadonlyRepository>,
        _enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let rows = sources
            .unwrap_or(&[])
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        *parents = Mapper::map(&self.nested, rows)
            .into_iter()
            .map(row_of)
            .collect();
        Ok(())
    }
}

fn row_of(mapped: MappedRow) -> Row {
    mapped
        .into_iter()
        .map(|(key, value)| (key, value_of(value)))
        .collect()
}

fn value_of(value: MappedValue) -> Value {
    match value {
        MappedValue::Cell(cell) => cell,
        MappedValue::Many(items) => encode_rows(&items.into_iter().map(row_of).collect::<Vec<_>>()),
    }
}
