use crate::entity::Entity;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use async_trait::async_trait;

pub struct DefaultReadonlyEnrichmentStrategy;

#[async_trait]
impl<T: Entity, TRelated: Entity> ReadonlyEnrichmentStrategy<T, TRelated>
    for DefaultReadonlyEnrichmentStrategy
{
}
