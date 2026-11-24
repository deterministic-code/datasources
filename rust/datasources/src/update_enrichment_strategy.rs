use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UpdateEnrichmentStrategy<T: Entity = Row>: Send + Sync {
    async fn update_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()>;

    async fn update_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()>;

    /// Updates the children of every parent. The default loops `update_children`; batching
    /// strategies override it to load all current children in one read.
    async fn update_children_batch(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parents: &mut [&mut T],
    ) -> Result<()> {
        for parent in parents.iter_mut() {
            self.update_children(writer.clone(), enrichment, parent).await?;
        }
        Ok(())
    }

    /// Updates the links of every parent. The default loops `update_linked`.
    async fn update_linked_batch(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parents: &mut [&mut T],
    ) -> Result<()> {
        for parent in parents.iter_mut() {
            self.update_linked(writer.clone(), enrichment, parent).await?;
        }
        Ok(())
    }
}
