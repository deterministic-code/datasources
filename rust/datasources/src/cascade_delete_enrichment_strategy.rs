use crate::delete_enrichment_strategy::DeleteEnrichmentStrategy;
use crate::from_schema::FromSchema;
use async_trait::async_trait;

pub struct CascadeDeleteEnrichmentStrategy;

#[async_trait]
impl<T: FromSchema> DeleteEnrichmentStrategy<T> for CascadeDeleteEnrichmentStrategy {}
