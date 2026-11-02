use crate::create_enrichment_strategy::CreateEnrichmentStrategy;
use crate::from_schema::FromSchema;
use async_trait::async_trait;

pub struct DefaultCreateEnrichmentStrategy;

#[async_trait]
impl<T: FromSchema> CreateEnrichmentStrategy<T> for DefaultCreateEnrichmentStrategy {}
