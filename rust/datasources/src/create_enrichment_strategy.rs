use crate::child_writes::ChildWrites;
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::result::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CreateEnrichmentStrategy<T: FromSchema>: Send + Sync {
    async fn create_children(
        &self,
        enrichment: &Enrichment,
        parent: &mut T,
        writes: &dyn ChildWrites<T>,
    ) -> Result<()> {
        let _ = self;
        writes.create_children(enrichment, parent).await
    }

    async fn create_linked(
        &self,
        enrichment: &Enrichment,
        parent: &T,
        writes: &dyn ChildWrites<T>,
    ) -> Result<()> {
        let _ = self;
        writes.create_linked(enrichment, parent).await
    }
}
