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
}
