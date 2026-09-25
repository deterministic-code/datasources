use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::repository_tree_node::{encode_row, encode_rows, source_key, RepositoryTreeNode};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub struct DefaultReadonlyEnrichmentStrategy;

#[async_trait]
impl ReadonlyEnrichmentStrategy for DefaultReadonlyEnrichmentStrategy {
    async fn apply(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut [Row],
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        if enrichment.many {
            apply_many(via.as_ref(), enrichment, parents, sources).await
        } else {
            apply_lookup(via.as_ref(), enrichment, parents, sources).await
        }
    }
}

async fn apply_lookup(
    via: &dyn IReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    let mut cache: HashMap<String, Option<Row>> = HashMap::new();
    for (index, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key(parent, &enrichment.from, at(sources, index)) else {
            continue;
        };
        let cache_key = RepositoryTreeNode::key_of(&key);
        if !cache.contains_key(&cache_key) {
            let related = via
                .read_single(
                    Some(&Where::new(Operator::equals(
                        enrichment.by.clone(),
                        key.clone(),
                    ))),
                    None,
                )
                .await?;
            cache.insert(cache_key.clone(), related);
        }
        let Some(related) = cache.get(&cache_key).cloned().flatten() else {
            continue;
        };
        parent.insert(enrichment.into.clone(), selected(enrichment, &related));
    }
    Ok(())
}

async fn apply_many(
    via: &dyn IReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    for (index, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key(parent, &enrichment.from, at(sources, index)) else {
            continue;
        };
        let related = via
            .read(
                Some(&Where::new(Operator::equals(
                    enrichment.by.clone(),
                    key,
                ))),
                None,
            )
            .await?;
        parent.insert(enrichment.into.clone(), encode_rows(&related));
    }
    Ok(())
}

fn selected(enrichment: &Enrichment, related: &Row) -> Value {
    enrichment
        .select
        .as_ref()
        .and_then(|select| related.get(select).cloned())
        .unwrap_or_else(|| encode_row(related))
}

fn at<'a>(sources: Option<&'a [Option<Row>]>, index: usize) -> Option<&'a Row> {
    sources
        .and_then(|rows| rows.get(index))
        .and_then(|row| row.as_ref())
}
