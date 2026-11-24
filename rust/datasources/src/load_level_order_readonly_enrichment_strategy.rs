use crate::datasource_error::DatasourceError;
use crate::entity::{assign_one, assign_related, Entity};
use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::{
    key_of, match_key, row_match_key, shape_dictionary_rows, source_at, source_key_of,
    ReadonlyEnrichmentStrategy,
};
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use futures_util::TryStreamExt;
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

    async fn load_by_keys<TItem: Entity>(
        &self,
        via: Arc<dyn IReadonlyRepository<TItem>>,
        column: &str,
        keys: &[Value],
    ) -> Result<Vec<TItem>> {
        let mut related = Vec::new();
        if keys.is_empty() {
            return Ok(related);
        }
        for chunk in keys.chunks(self.batch_size) {
            let loaded = via
                .read_where(
                    &Where::new(Operator::in_list(column.to_string(), chunk.to_vec())),
                    None,
                )
                .try_collect::<Vec<_>>()
                .await?;
            related.extend(loaded);
        }
        Ok(related)
    }
}

fn distinct_keys<T: Entity>(
    parents: &[T],
    sources: Option<&[Option<Row>]>,
    enrichment: &Enrichment,
) -> Vec<Value> {
    let mut keys = Vec::new();
    let mut seen = HashSet::new();
    for (index, parent) in parents.iter().enumerate() {
        let Some(key) = source_key_of(parent, &enrichment.source_property, source_at(sources, index))
        else {
            continue;
        };
        if seen.insert(key_of(&key)) {
            keys.push(key);
        }
    }
    keys
}

fn group<TRelated: Entity>(
    items: Vec<TRelated>,
    enrichment: &Enrichment,
) -> HashMap<String, Vec<TRelated>> {
    let mut index: HashMap<String, Vec<TRelated>> = HashMap::new();
    for related in items {
        if let Some(cache_key) = row_match_key(&related, enrichment) {
            index.entry(cache_key).or_default().push(related);
        }
    }
    index
}

#[async_trait]
impl<T: Entity, TRelated: Entity> ReadonlyEnrichmentStrategy<T, TRelated>
    for LoadLevelOrderReadonlyEnrichmentStrategy
{
    async fn apply_object(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let loaded = self
            .load_by_keys(via, &enrichment.related_key, &distinct_keys(parents, sources, enrichment))
            .await?;
        let mut index = HashMap::new();
        for related in loaded {
            if let Some(cache_key) = row_match_key(&related, enrichment) {
                index.entry(cache_key).or_insert(related);
            }
        }
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(cache_key) = match_key(parent, enrichment, source_at(sources, index_i)) else {
                continue;
            };
            let Some(related) = index.get(&cache_key).cloned() else {
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
        let index = group(
            self.load_by_keys(via, &enrichment.related_key, &distinct_keys(parents, sources, enrichment))
                .await?,
            enrichment,
        );
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(cache_key) = match_key(parent, enrichment, source_at(sources, index_i)) else {
                continue;
            };
            let related = index.get(&cache_key).cloned().unwrap_or_default();
            assign_related(parent, &enrichment.destination_property, related)?;
        }
        Ok(())
    }

    async fn apply_dictionary(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let schema = via.schema().clone();
        let index = group(
            self.load_by_keys(
                via,
                &enrichment.related_key,
                &distinct_keys(parents, sources, enrichment),
            )
            .await?,
            enrichment,
        );
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(cache_key) = match_key(parent, enrichment, source_at(sources, index_i)) else {
                continue;
            };
            let related = index.get(&cache_key).cloned().unwrap_or_default();
            let rows = shape_dictionary_rows(related, enrichment, &schema)?;
            parent.set_payload(&enrichment.destination_property, rows)?;
        }
        Ok(())
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
        let links = self
            .load_by_keys(link, &association.parent, &parent_keys)
            .await?;
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
        for item in self
            .load_by_keys(via, &enrichment.related_key, &related_ids)
            .await?
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
