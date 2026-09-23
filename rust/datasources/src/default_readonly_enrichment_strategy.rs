use crate::enrichment::Enrichment;
use crate::query::Query;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::{encode_row, encode_rows, RepositoryTreeNode};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::where_trait::Where;
use crate::{Operator, WhereClause};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct DefaultReadonlyEnrichmentStrategy;

#[async_trait]
impl ReadonlyEnrichmentStrategy for DefaultReadonlyEnrichmentStrategy {
    async fn read(
        &self,
        inner: &dyn IReadonlyRepository,
        tree: &RepositoryTree,
        query: Option<&dyn Query>,
    ) -> Result<Vec<Row>> {
        let mut items = inner.read(query).await?;
        self.enrich(tree, &mut items).await?;
        Ok(items)
    }

    async fn read_where(
        &self,
        inner: &dyn IReadonlyRepository,
        tree: &RepositoryTree,
        where_clause: &dyn Where,
    ) -> Result<Vec<Row>> {
        let mut items = inner.read_where(where_clause).await?;
        self.enrich(tree, &mut items).await?;
        Ok(items)
    }

    async fn enrich(&self, tree: &RepositoryTree, items: &mut [Row]) -> Result<()> {
        tree.parent.enrich_from(items).await
    }

    async fn apply(&self, node: &RepositoryTreeNode, parents: &mut [Row]) -> Result<()> {
        match &node.enrichment {
            Some(enrichment) => apply(node.value.as_ref(), enrichment, parents, &node.items).await,
            None => {
                for child in &node.items {
                    child.enrich_from(parents).await?;
                }
                Ok(())
            }
        }
    }
}

async fn apply(
    via: &dyn IReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
    children: &[RepositoryTreeNode],
) -> Result<()> {
    if enrichment.many {
        apply_many(via, enrichment, parents, children).await
    } else {
        apply_lookup(via, enrichment, parents, children).await
    }
}

async fn apply_lookup(
    via: &dyn IReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
    children: &[RepositoryTreeNode],
) -> Result<()> {
    let mut cache: HashMap<String, Option<Row>> = HashMap::new();
    for parent in parents.iter_mut() {
        let Some(key) = parent.get(&enrichment.from).cloned() else {
            continue;
        };
        let cache_key = key.wire_text();
        if !cache.contains_key(&cache_key) {
            let related = via
                .read_single_where(&WhereClause::new(Operator::equals(
                    enrichment.by.clone(),
                    key.clone(),
                )))
                .await?;
            cache.insert(cache_key.clone(), related);
        }
        let Some(mut related) = cache.get(&cache_key).cloned().flatten() else {
            continue;
        };
        for child in children {
            child
                .enrich_from(std::slice::from_mut(&mut related))
                .await?;
        }
        cache.insert(cache_key, Some(related.clone()));
        let value = selected(enrichment, &related);
        parent.insert(enrichment.into.clone(), value);
    }
    Ok(())
}

async fn apply_many(
    via: &dyn IReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
    children: &[RepositoryTreeNode],
) -> Result<()> {
    for parent in parents.iter_mut() {
        let Some(key) = parent.get(&enrichment.from).cloned() else {
            continue;
        };
        let mut related = via
            .read_where(&WhereClause::new(Operator::equals(
                enrichment.by.clone(),
                key,
            )))
            .await?;
        for child in children {
            child.enrich_from(&mut related).await?;
        }
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
