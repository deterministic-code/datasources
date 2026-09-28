use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::repository_tree_node::{
    encode_row, encode_rows, source_at, source_key, RepositoryTreeNode,
};
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
    async fn apply_lookup(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let keys = keys(parents, sources, &enrichment.from);
        if keys.is_empty() {
            return Ok(());
        }
        let loaded = via
            .read(
                Some(&Where::new(Operator::in_list(
                    enrichment.by.clone(),
                    keys.values().cloned().collect(),
                ))),
                None,
            )
            .await?;
        let mut index = HashMap::new();
        for related in loaded {
            if let Some(by) = related.get(&enrichment.by) {
                index.insert(RepositoryTreeNode::key_of(by), related);
            }
        }
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(parent, &enrichment.from, source_at(sources, index_i))
            else {
                continue;
            };
            let Some(related) = index.get(&RepositoryTreeNode::key_of(&key)) else {
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
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let keys = keys(parents, sources, &enrichment.from);
        if keys.is_empty() {
            return Ok(());
        }
        let loaded = via
            .read(
                Some(&Where::new(Operator::in_list(
                    enrichment.by.clone(),
                    keys.values().cloned().collect(),
                ))),
                None,
            )
            .await?;
        let mut index: HashMap<String, Vec<Row>> = HashMap::new();
        for row in loaded {
            if let Some(by) = row.get(&enrichment.by) {
                index
                    .entry(RepositoryTreeNode::key_of(by))
                    .or_default()
                    .push(row);
            }
        }
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(parent, &enrichment.from, source_at(sources, index_i))
            else {
                continue;
            };
            let related = index
                .get(&RepositoryTreeNode::key_of(&key))
                .cloned()
                .unwrap_or_default();
            parent.insert(enrichment.into.clone(), encode_rows(&related));
        }
        Ok(())
    }
}

fn keys(parents: &[Row], sources: Option<&[Option<Row>]>, from: &str) -> HashMap<String, Value> {
    let mut keys = HashMap::new();
    for (index, parent) in parents.iter().enumerate() {
        if let Some(key) = source_key(parent, from, source_at(sources, index)) {
            keys.entry(RepositoryTreeNode::key_of(&key)).or_insert(key);
        }
    }
    keys
}
