use futures_util::TryStreamExt;
use crate::default_write_enrichment_strategy::{encode_row, encode_rows};
use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::{
    key_of, source_at, source_key, ReadonlyEnrichmentStrategy,
};
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub struct HashsetOptimizedReadonlyEnrichmentStrategy;

#[async_trait]
impl ReadonlyEnrichmentStrategy for HashsetOptimizedReadonlyEnrichmentStrategy {
    async fn apply_object(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let keys = keys(parents, sources, &enrichment.source_property);
        if keys.is_empty() {
            return Ok(());
        }
        let loaded = via
            .read(
                Some(&Where::new(Operator::in_list(
                    enrichment.related_key.clone(),
                    keys.values().cloned().collect(),
                ))),
                None, None
        )
            .try_collect::<Vec<_>>().await?;
        let mut index = HashMap::new();
        for related in loaded {
            if let Some(by) = related.get(&enrichment.related_key) {
                index.insert(key_of(by), related);
            }
        }
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(
                parent,
                &enrichment.source_property,
                source_at(sources, index_i),
            ) else {
                continue;
            };
            let Some(related) = index.get(&key_of(&key)) else {
                continue;
            };
            let value = enrichment
                .related_property
                .as_ref()
                .and_then(|select| related.get(select).cloned())
                .unwrap_or_else(|| encode_row(related));
            parent.insert(enrichment.destination_property.clone(), value);
        }
        Ok(())
    }

    async fn apply_list(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let keys = keys(parents, sources, &enrichment.source_property);
        if keys.is_empty() {
            return Ok(());
        }
        let loaded = via
            .read(
                Some(&Where::new(Operator::in_list(
                    enrichment.related_key.clone(),
                    keys.values().cloned().collect(),
                ))),
                None, None
        )
            .try_collect::<Vec<_>>().await?;
        let mut index: HashMap<String, Vec<Row>> = HashMap::new();
        for row in loaded {
            if let Some(by) = row.get(&enrichment.related_key) {
                index.entry(key_of(by)).or_default().push(row);
            }
        }
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(
                parent,
                &enrichment.source_property,
                source_at(sources, index_i),
            ) else {
                continue;
            };
            let related = index.get(&key_of(&key)).cloned().unwrap_or_default();
            parent.insert(
                enrichment.destination_property.clone(),
                encode_rows(&related),
            );
        }
        Ok(())
    }
}

fn keys(parents: &[Row], sources: Option<&[Option<Row>]>, from: &str) -> HashMap<String, Value> {
    let mut keys = HashMap::new();
    for (index, parent) in parents.iter().enumerate() {
        if let Some(key) = source_key(parent, from, source_at(sources, index)) {
            keys.entry(key_of(&key)).or_insert(key);
        }
    }
    keys
}
