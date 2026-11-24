use crate::datasource_error::DatasourceError;
use crate::entity::{assign_related, Entity};
use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::{
    distinct_keys, key_of, load_by_keys, source_at, source_key_of, ReadonlyEnrichmentStrategy,
};
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct LoadLevelOrderReadonlyEnrichmentStrategy {
    batch_size: usize,
}

impl Default for LoadLevelOrderReadonlyEnrichmentStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadLevelOrderReadonlyEnrichmentStrategy {
    pub fn new() -> Self {
        Self { batch_size: 500 }
    }

    pub fn with_batch_size(self, batch_size: usize) -> Self {
        Self {
            batch_size: batch_size.max(1),
        }
    }
}

#[async_trait]
impl<T: Entity, TRelated: Entity> ReadonlyEnrichmentStrategy<T, TRelated>
    for LoadLevelOrderReadonlyEnrichmentStrategy
{
    fn fill_batch_size(&self) -> usize {
        self.batch_size
    }

    fn linked_batch_size(&self) -> Option<usize> {
        Some(self.batch_size)
    }
}

pub(crate) async fn load_level_order_apply_linked<T: Entity, TRelated: Entity, TLink: Entity>(
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    link: Arc<dyn IReadonlyRepository<TLink>>,
    enrichment: &Enrichment,
    parents: &mut Vec<T>,
    sources: Option<&[Option<Row>]>,
    batch_size: usize,
) -> Result<()> {
    LoadLevelOrderReadonlyEnrichmentStrategy { batch_size }
        .fill_linked(via, link, enrichment, parents, sources)
        .await
}

impl LoadLevelOrderReadonlyEnrichmentStrategy {
    async fn fill_linked<T: Entity, TRelated: Entity, TLink: Entity>(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        link: Arc<dyn IReadonlyRepository<TLink>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let association = enrichment
            .association
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Missing association"))?;
        let parent_keys = distinct_keys(parents, sources, enrichment);
        let links = load_by_keys(link, &association.parent, &parent_keys, self.batch_size).await?;
        let mut related_ids = Vec::new();
        let mut seen = HashSet::new();
        for row in &links {
            let Some(id) = row.cell(&association.related) else {
                continue;
            };
            if seen.insert(key_of(&id)) {
                related_ids.push(id);
            }
        }
        let mut related_by_id = HashMap::new();
        for item in load_by_keys(via, &enrichment.related_key, &related_ids, self.batch_size).await?
        {
            if let Some(id) = item.cell(&enrichment.related_key) {
                related_by_id.entry(key_of(&id)).or_insert(item);
            }
        }
        let mut links_by_parent: HashMap<String, Vec<TLink>> = HashMap::new();
        for row in links {
            let Some(parent_key) = row.cell(&association.parent) else {
                continue;
            };
            links_by_parent
                .entry(key_of(&parent_key))
                .or_default()
                .push(row);
        }
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) =
                source_key_of(parent, &enrichment.source_property, source_at(sources, index))
            else {
                continue;
            };
            let mut related = Vec::new();
            if let Some(rows) = links_by_parent.get(&key_of(&key)) {
                for row in rows {
                    let Some(id) = row.cell(&association.related) else {
                        continue;
                    };
                    if let Some(entity) = related_by_id.get(&key_of(&id)).cloned() {
                        related.push(entity);
                    }
                }
            }
            assign_related(parent, &enrichment.destination_property, related)?;
        }
        Ok(())
    }
}
