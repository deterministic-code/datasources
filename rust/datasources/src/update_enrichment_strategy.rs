use crate::child_writes::ChildWrites;
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::result::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UpdateEnrichmentStrategy<T: FromSchema>: Send + Sync {
    async fn update_children(
        &self,
        enrichment: &Enrichment,
        parent: &mut T,
        writes: &dyn ChildWrites<T>,
    ) -> Result<()> {
        let _ = self;
        writes.update_children(enrichment, parent).await
    }

    async fn update_linked(
        &self,
        enrichment: &Enrichment,
        parent: &T,
        writes: &dyn ChildWrites<T>,
    ) -> Result<()> {
        let _ = self;
        writes.update_linked(enrichment, parent).await
    }
}
