use crate::enrichment::Enrichment;
use crate::result::Result;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use async_trait::async_trait;

#[async_trait]
pub trait UpdateEnrichmentStrategy: Send + Sync {
    async fn update_owned(
        &self,
        entry: &StoredEnrichment,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()>;

    async fn update_linked(
        &self,
        entry: &StoredEnrichment,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()>;
}
