use crate::create_enrichment_strategy::CreateEnrichmentStrategy;
use crate::default_write_enrichment_strategy::DefaultWriteEnrichmentStrategy;
use crate::delete_enrichment_strategy::DeleteEnrichmentStrategy;
use crate::from_schema::FromSchema;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use std::sync::Arc;

pub struct LazyWriteEnrichmentStrategy;

impl LazyWriteEnrichmentStrategy {
    pub fn new<T: FromSchema>(
        delete: Option<Arc<dyn DeleteEnrichmentStrategy<T>>>,
        update: Option<Arc<dyn UpdateEnrichmentStrategy<T>>>,
        create: Option<Arc<dyn CreateEnrichmentStrategy<T>>>,
        use_transaction: bool,
    ) -> DefaultWriteEnrichmentStrategy<T> {
        DefaultWriteEnrichmentStrategy::lazy(delete, update, create, use_transaction)
    }
}
