use crate::entity::{assign_related, Entity};
use crate::enrichment::Enrichment;
use crate::enrichment_via::EnrichmentVia;
use crate::readonly_enrichment_strategy::{key_of, match_key, source_at, ReadonlyEnrichmentStrategy};
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use futures_util::TryStreamExt;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct GraphReadonlyEnrichmentStrategy<TLink: Entity> {
    link: Arc<dyn IReadonlyRepository<TLink>>,
}

impl<TLink: Entity> GraphReadonlyEnrichmentStrategy<TLink> {
    pub fn new(link: impl EnrichmentVia<TLink>) -> Self {
        Self {
            link: link.read_via(),
        }
    }
}

#[async_trait]
impl<T: Entity, TRelated: Entity, TLink: Entity> ReadonlyEnrichmentStrategy<T, TRelated>
    for GraphReadonlyEnrichmentStrategy<TLink>
{
    async fn apply(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let association = enrichment
            .association
            .as_ref()
            .ok_or_else(|| crate::DatasourceError::message("Missing association"))?;
        let loaded = via.read(None, None).try_collect::<Vec<_>>().await?;
        let mut nodes = HashMap::new();
        for item in loaded {
            if let Some(id) = match_key(&item, enrichment, None) {
                nodes.insert(id, item);
            }
        }
        let edges = self.link.read(None, None).try_collect::<Vec<_>>().await?;
        let mut outgoing: HashMap<String, Vec<String>> = HashMap::new();
        for edge in edges {
            let Some(from) = edge.cell(&association.parent) else {
                continue;
            };
            let Some(to) = edge.cell(&association.related) else {
                continue;
            };
            outgoing
                .entry(key_of(&from))
                .or_default()
                .push(key_of(&to));
        }
        let mut filled = HashSet::new();
        for (offset, parent) in parents.iter_mut().enumerate() {
            let Some(id) = match_key(parent, enrichment, source_at(sources, offset)) else {
                assign_related::<T, TRelated>(parent, &enrichment.destination_property, Vec::new())?;
                continue;
            };
            filled.insert(id.clone());
            let related = walk(&id, enrichment, &nodes, &outgoing, &mut HashSet::from([id.clone()]), &mut filled)?;
            assign_related(parent, &enrichment.destination_property, related)?;
        }
        Ok(())
    }
}

fn walk<TRelated: Entity>(
    from: &str,
    enrichment: &Enrichment,
    nodes: &HashMap<String, TRelated>,
    outgoing: &HashMap<String, Vec<String>>,
    path: &mut HashSet<String>,
    filled: &mut HashSet<String>,
) -> Result<Vec<TRelated>> {
    let mut edges = Vec::new();
    for to in outgoing.get(from).cloned().unwrap_or_default() {
        if path.contains(&to) {
            continue;
        }
        let Some(node) = nodes.get(&to) else {
            continue;
        };
        let first = filled.insert(to.clone());
        let mut copy = node.clone();
        assign_related::<TRelated, TRelated>(&mut copy, &enrichment.destination_property, Vec::new())?;
        if first {
            path.insert(to.clone());
            let nested = walk(&to, enrichment, nodes, outgoing, path, filled)?;
            path.remove(&to);
            assign_related(&mut copy, &enrichment.destination_property, nested)?;
        }
        edges.push(copy);
    }
    Ok(edges)
}
