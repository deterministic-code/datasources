use crate::entity::{assign_related, Entity};
use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::{match_key, row_match_key, source_at, ReadonlyEnrichmentStrategy};
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use futures_util::TryStreamExt;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct TreeReadonlyEnrichmentStrategy;

#[async_trait]
impl<T: Entity> ReadonlyEnrichmentStrategy<T, T> for TreeReadonlyEnrichmentStrategy {
    async fn apply(
        &self,
        via: Arc<dyn IReadonlyRepository<T>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let rows = via.read(None, None).try_collect::<Vec<_>>().await?;
        let mut index: HashMap<String, Vec<T>> = HashMap::new();
        for item in rows {
            if let Some(parent_key) = row_match_key(&item, enrichment) {
                index.entry(parent_key).or_default().push(item);
            }
        }
        let mut ids = HashSet::new();
        for (offset, parent) in parents.iter_mut().enumerate() {
            attach(parent, enrichment, &index, &mut HashSet::new(), source_at(sources, offset))?;
            if let Some(id) = match_key(parent, enrichment, source_at(sources, offset)) {
                ids.insert(id);
            }
        }
        parents.retain(|parent| {
            row_match_key(parent, enrichment).is_none_or(|parent_key| !ids.contains(&parent_key))
        });
        Ok(())
    }
}

fn attach<T: Entity>(
    parent: &mut T,
    enrichment: &Enrichment,
    index: &HashMap<String, Vec<T>>,
    path: &mut HashSet<String>,
    source: Option<&Row>,
) -> Result<()> {
    let Some(id) = match_key(parent, enrichment, source) else {
        assign_related::<T, T>(parent, &enrichment.destination_property, Vec::new())?;
        return Ok(());
    };
    path.insert(id.clone());
    let children = nest(id.clone(), enrichment, index, path)?;
    path.remove(&id);
    assign_related(parent, &enrichment.destination_property, children)
}

fn nest<T: Entity>(
    id: String,
    enrichment: &Enrichment,
    index: &HashMap<String, Vec<T>>,
    path: &mut HashSet<String>,
) -> Result<Vec<T>> {
    let mut children = Vec::new();
    for child in index.get(&id).cloned().unwrap_or_default() {
        let Some(child_id) = match_key(&child, enrichment, None) else {
            continue;
        };
        if path.contains(&child_id) {
            continue;
        }
        let mut copy = copy_except(&child, &enrichment.destination_property)?;
        path.insert(child_id.clone());
        let nested = nest(child_id.clone(), enrichment, index, path)?;
        path.remove(&child_id);
        assign_related(&mut copy, &enrichment.destination_property, nested)?;
        children.push(copy);
    }
    Ok(children)
}

fn copy_except<T: Entity>(item: &T, dest: &str) -> Result<T> {
    let mut copy = item.clone();
    assign_related::<T, T>(&mut copy, dest, Vec::new())?;
    Ok(copy)
}
