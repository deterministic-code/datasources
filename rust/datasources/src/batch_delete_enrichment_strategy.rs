use crate::batch_enrichment_load::load_grouped;
use crate::cascade_delete_enrichment_strategy::{
    delete_children, delete_existing, delete_linked, delete_linked_with,
};
use crate::delete_enrichment_strategy::DeleteEnrichmentStrategy;
use crate::delete_nested::DeleteNested;
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::sync::Arc;

/// Same deletes as `CascadeDeleteEnrichmentStrategy`, but the children or links of all parents
/// are loaded with one batched read, and the nested cascade runs once over all their rows.
/// Enrichments keyed by several columns delete per parent.
pub struct BatchDeleteEnrichmentStrategy;

#[async_trait]
impl<T: Entity> DeleteEnrichmentStrategy<T> for BatchDeleteEnrichmentStrategy {
    async fn delete_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &T,
        delete_nested: &dyn DeleteNested,
    ) -> Result<()> {
        delete_children(writer.as_ref(), enrichment, parent, delete_nested).await
    }

    async fn delete_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &T,
    ) -> Result<()> {
        delete_linked(writer.as_ref(), enrichment, parent).await
    }

    async fn delete_children_batch(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parents: &[&T],
        delete_nested: &dyn DeleteNested,
    ) -> Result<()> {
        let pairs = enrichment.pairs();
        if parents.len() < 2 || pairs.len() != 1 {
            for &parent in parents {
                delete_children(writer.as_ref(), enrichment, parent, delete_nested).await?;
            }
            return Ok(());
        }
        let (source, related) = &pairs[0];
        let existing = load_grouped(writer.as_ref(), related, source, parents).await?;
        let mut rows: Vec<Row> = existing
            .iter()
            .flatten()
            .filter_map(|child| <Row as Entity>::object_converter().reverse_convert(child).ok())
            .collect();
        delete_nested.delete_nested(&mut rows).await?;
        for (&parent, children) in parents.iter().zip(existing) {
            delete_existing(writer.as_ref(), enrichment, parent, children).await?;
        }
        Ok(())
    }

    async fn delete_linked_batch(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parents: &[&T],
    ) -> Result<()> {
        let Some(association) = enrichment.association.as_ref() else {
            for &parent in parents {
                delete_linked(writer.as_ref(), enrichment, parent).await?;
            }
            return Ok(());
        };
        if parents.len() < 2 {
            for &parent in parents {
                delete_linked(writer.as_ref(), enrichment, parent).await?;
            }
            return Ok(());
        }
        let existing = load_grouped(
            writer.as_ref(),
            &association.parent,
            &enrichment.source_property,
            parents,
        )
        .await?;
        for (&parent, rows) in parents.iter().zip(existing) {
            delete_linked_with(writer.as_ref(), enrichment, parent, Some(rows)).await?;
        }
        Ok(())
    }
}
