use crate::enrichment::Enrichment;
use crate::mappers::Mapper;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::sync::Arc;

pub struct NestedToObjectEnrichmentStrategy {
    mapper: Box<dyn Mapper<Vec<Row>, Vec<Row>> + Send + Sync>,
}

impl NestedToObjectEnrichmentStrategy {
    pub fn new(mapper: Box<dyn Mapper<Vec<Row>, Vec<Row>> + Send + Sync>) -> Self {
        Self { mapper }
    }
}

#[async_trait]
impl ReadonlyEnrichmentStrategy for NestedToObjectEnrichmentStrategy {
    async fn apply(
        &self,
        _via: Arc<dyn IReadonlyRepository>,
        _enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let rows = sources
            .unwrap_or(&[])
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        *parents = Mapper::map(&self.mapper, rows);
        Ok(())
    }
}
