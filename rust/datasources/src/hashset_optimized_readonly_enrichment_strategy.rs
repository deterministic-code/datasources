use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::{encode_row, encode_rows, RepositoryTreeNode};
use crate::result::Result;
use crate::row::Row;
use crate::statement::Statement;
use crate::value::Value;
use crate::where_trait::Where;
use crate::{Operator, WhereClause};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct HashsetOptimizedReadonlyEnrichmentStrategy;

#[async_trait]
impl ReadonlyEnrichmentStrategy for HashsetOptimizedReadonlyEnrichmentStrategy {
    async fn read(
        &self,
        inner: &dyn IReadonlyRepository,
        tree: &RepositoryTree,
        query: Option<&dyn Statement>,
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

fn keys(parents: &[Row], from: &str) -> Vec<Value> {
    let mut unique = HashMap::new();
    for parent in parents {
        if let Some(key) = parent.get(from) {
            unique.entry(key.wire_text()).or_insert_with(|| key.clone());
        }
    }
    unique.into_values().collect()
}

async fn apply_lookup(
    via: &dyn IReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
    children: &[RepositoryTreeNode],
) -> Result<()> {
    let values = keys(parents, &enrichment.from);
    if values.is_empty() {
        return Ok(());
    }
    let loaded = via
        .read_where(&WhereClause::new(Operator::in_list(
            enrichment.by.clone(),
            values,
        )))
        .await?;
    let mut index = HashMap::new();
    for mut related in loaded {
        for child in children {
            child
                .enrich_from(std::slice::from_mut(&mut related))
                .await?;
        }
        if let Some(by) = related.get(&enrichment.by).cloned() {
            index.insert(by.wire_text(), related);
        }
    }
    for parent in parents.iter_mut() {
        let Some(key) = parent.get(&enrichment.from).cloned() else {
            continue;
        };
        let Some(related) = index.get(&key.wire_text()) else {
            continue;
        };
        let value = enrichment
            .select
            .as_ref()
            .and_then(|select| related.get(select).cloned())
            .unwrap_or_else(|| encode_row(related));
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
    let values = keys(parents, &enrichment.from);
    if values.is_empty() {
        return Ok(());
    }
    let loaded = via
        .read_where(&WhereClause::new(Operator::in_list(
            enrichment.by.clone(),
            values,
        )))
        .await?;
    let mut index: HashMap<String, Vec<Row>> = HashMap::new();
    for row in loaded {
        if let Some(by) = row.get(&enrichment.by) {
            index.entry(by.wire_text()).or_default().push(row);
        }
    }
    for related in index.values_mut() {
        for child in children {
            child.enrich_from(related).await?;
        }
    }
    for parent in parents.iter_mut() {
        let Some(key) = parent.get(&enrichment.from).cloned() else {
            continue;
        };
        let related = index.get(&key.wire_text()).cloned().unwrap_or_default();
        parent.insert(enrichment.into.clone(), encode_rows(&related));
    }
    Ok(())
}
