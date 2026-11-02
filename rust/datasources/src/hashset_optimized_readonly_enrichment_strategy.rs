use crate::entity::{assign_one, assign_related, Entity};
use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::{
    key_of, source_at, source_key_of, ReadonlyEnrichmentStrategy,
};
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

pub struct HashsetOptimizedReadonlyEnrichmentStrategy;

#[async_trait]
impl<T: Entity, TRelated: Entity> ReadonlyEnrichmentStrategy<T, TRelated>
    for HashsetOptimizedReadonlyEnrichmentStrategy
{
    async fn apply_object(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let keys = keys(parents, sources, &enrichment.source_property);
        if keys.is_empty() {
            return Ok(());
        }
        let loaded = via
            .read(&Where::new(Operator::in_list(
                enrichment.related_key.clone(),
                keys.values().cloned().collect(),
            )))
            .try_collect::<Vec<_>>()
            .await?;
        let mut index = HashMap::new();
        for related in loaded {
            if let Some(by) = related.cell(&enrichment.related_key) {
                index.insert(key_of(&by), related);
            }
        }
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key_of(
                parent,
                &enrichment.source_property,
                source_at(sources, index_i),
            ) else {
                continue;
            };
            let Some(related) = index.get(&key_of(&key)).cloned() else {
                continue;
            };
            match enrichment.related_property.as_deref() {
                Some(select) => {
                    if let Some(cell) = related.cell(select) {
                        parent.set_cell(&enrichment.destination_property, cell)?;
                    }
                }
                None => assign_one(parent, &enrichment.destination_property, related)?,
            }
        }
        Ok(())
    }

    async fn apply_list(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let keys = keys(parents, sources, &enrichment.source_property);
        if keys.is_empty() {
            return Ok(());
        }
        let loaded = via
            .read(&Where::new(Operator::in_list(
                enrichment.related_key.clone(),
                keys.values().cloned().collect(),
            )))
            .try_collect::<Vec<_>>()
            .await?;
        let mut index: HashMap<String, Vec<TRelated>> = HashMap::new();
        for related in loaded {
            if let Some(by) = related.cell(&enrichment.related_key) {
                index.entry(key_of(&by)).or_default().push(related);
            }
        }
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key_of(
                parent,
                &enrichment.source_property,
                source_at(sources, index_i),
            ) else {
                continue;
            };
            let related = index.get(&key_of(&key)).cloned().unwrap_or_default();
            assign_related(parent, &enrichment.destination_property, related)?;
        }
        Ok(())
    }
}

fn keys<T: Entity>(parents: &[T], sources: Option<&[Option<Row>]>, from: &str) -> HashMap<String, Value> {
    let mut keys = HashMap::new();
    for (index, parent) in parents.iter().enumerate() {
        if let Some(key) = source_key_of(parent, from, source_at(sources, index)) {
            keys.entry(key_of(&key)).or_insert(key);
        }
    }
    keys
}
