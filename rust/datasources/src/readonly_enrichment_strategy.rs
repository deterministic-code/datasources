use crate::enrichment::Enrichment;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ReadonlyEnrichmentStrategy: Send + Sync {
    async fn apply(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut [Row],
        sources: Option<&[Option<Row>]>,
    ) -> Result<()>;
}
