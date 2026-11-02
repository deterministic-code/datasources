use crate::child_writes::ChildWrites;
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::result::Result;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use async_trait::async_trait;

pub struct CascadeDeleteUpdateEnrichmentStrategy;

#[async_trait]
impl<T: FromSchema> UpdateEnrichmentStrategy<T> for CascadeDeleteUpdateEnrichmentStrategy {
    async fn update_children(
        &self,
        enrichment: &Enrichment,
        parent: &mut T,
        writes: &dyn ChildWrites<T>,
    ) -> Result<()> {
        writes.delete_children(enrichment, parent).await?;
        writes.create_children(enrichment, parent).await
    }

    async fn update_linked(
        &self,
        enrichment: &Enrichment,
        parent: &T,
        writes: &dyn ChildWrites<T>,
    ) -> Result<()> {
        writes.delete_linked(enrichment, parent).await?;
        writes.create_linked(enrichment, parent).await
    }
}
