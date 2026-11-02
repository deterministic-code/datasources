use crate::from_schema::FromSchema;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;

pub struct DefaultReadonlyEnrichmentStrategy;

impl<T: FromSchema, TRelated: FromSchema> ReadonlyEnrichmentStrategy<T, TRelated>
    for DefaultReadonlyEnrichmentStrategy
{
}
