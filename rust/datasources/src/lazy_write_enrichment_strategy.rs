use crate::default_write_enrichment_strategy::DefaultWriteEnrichmentStrategy;
use crate::delete_enrichment_strategy::DeleteEnrichmentStrategy;
use std::sync::Arc;

pub struct LazyWriteEnrichmentStrategy;

impl LazyWriteEnrichmentStrategy {
    pub fn new(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>) -> DefaultWriteEnrichmentStrategy {
        DefaultWriteEnrichmentStrategy::lazy(delete)
    }
}
