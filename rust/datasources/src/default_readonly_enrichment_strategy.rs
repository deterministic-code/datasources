use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use async_trait::async_trait;

pub struct DefaultReadonlyEnrichmentStrategy;

#[async_trait]
impl ReadonlyEnrichmentStrategy for DefaultReadonlyEnrichmentStrategy {}
