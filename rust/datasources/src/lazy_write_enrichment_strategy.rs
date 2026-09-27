use crate::default_write_enrichment_strategy::DefaultWriteEnrichmentStrategy;
use crate::delete_enrichment_strategy::DeleteEnrichmentStrategy;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use std::sync::Arc;

pub struct LazyWriteEnrichmentStrategy;

impl LazyWriteEnrichmentStrategy {
    pub fn new(
        delete: Option<Arc<dyn DeleteEnrichmentStrategy>>,
        update: Option<Arc<dyn UpdateEnrichmentStrategy>>,
        use_transaction: bool,
    ) -> DefaultWriteEnrichmentStrategy {
        DefaultWriteEnrichmentStrategy::lazy(delete, update, use_transaction)
    }
}
