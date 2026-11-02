use crate::from_schema::FromSchema;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use async_trait::async_trait;

pub struct DefaultUpdateEnrichmentStrategy;

#[async_trait]
impl<T: FromSchema> UpdateEnrichmentStrategy<T> for DefaultUpdateEnrichmentStrategy {}
