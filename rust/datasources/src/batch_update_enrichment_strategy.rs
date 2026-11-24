use crate::batch_enrichment_load::load_grouped;
use crate::default_update_enrichment_strategy::{
    update_children, update_children_with, update_linked, update_linked_with,
};
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use async_trait::async_trait;
use std::sync::Arc;

/// Same writes as `DefaultUpdateEnrichmentStrategy`, but the current children or links of all
/// parents are loaded with one batched read instead of one read per parent. Enrichments keyed
/// by several columns load per parent.
pub struct BatchUpdateEnrichmentStrategy;

#[async_trait]
impl<T: Entity> UpdateEnrichmentStrategy<T> for BatchUpdateEnrichmentStrategy {
    async fn update_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        update_children(writer.as_ref(), enrichment, parent).await
    }

    async fn update_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        update_linked(writer.as_ref(), enrichment, parent).await
    }

    async fn update_children_batch(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parents: &mut [&mut T],
    ) -> Result<()> {
        let pairs = enrichment.pairs();
        if parents.len() < 2 || pairs.len() != 1 {
            for parent in parents.iter_mut() {
                update_children(writer.as_ref(), enrichment, &mut **parent).await?;
            }
            return Ok(());
        }
        let (source, related) = &pairs[0];
        let existing = {
            let view: Vec<&T> = parents.iter().map(|parent| &**parent).collect();
            load_grouped(writer.as_ref(), related, source, &view).await?
        };
        for (parent, rows) in parents.iter_mut().zip(existing) {
            update_children_with(writer.as_ref(), enrichment, &mut **parent, rows).await?;
        }
        Ok(())
    }

    async fn update_linked_batch(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parents: &mut [&mut T],
    ) -> Result<()> {
        let Some(association) = enrichment.association.as_ref() else {
            for parent in parents.iter_mut() {
                update_linked(writer.as_ref(), enrichment, &mut **parent).await?;
            }
            return Ok(());
        };
        if parents.len() < 2 {
            for parent in parents.iter_mut() {
                update_linked(writer.as_ref(), enrichment, &mut **parent).await?;
            }
            return Ok(());
        }
        let existing = {
            let view: Vec<&T> = parents.iter().map(|parent| &**parent).collect();
            load_grouped(
                writer.as_ref(),
                &association.parent,
                &enrichment.source_property,
                &view,
            )
            .await?
        };
        for (parent, rows) in parents.iter_mut().zip(existing) {
            update_linked_with(writer.as_ref(), enrichment, &mut **parent, Some(rows)).await?;
        }
        Ok(())
    }
}
