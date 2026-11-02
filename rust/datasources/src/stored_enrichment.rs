use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::enrichment::Enrichment;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use crate::row::Row;
use std::sync::Arc;

pub(crate) struct StoredEnrichment {
    pub enrichment: Enrichment,
    pub via: Arc<dyn IReadonlyRepository>,
    pub persist: Option<Arc<dyn ICRUDRepository>>,
    pub persist_bulk: Option<Arc<BulkCRUDRepository>>,
    pub link: Option<Arc<dyn ICRUDRepository>>,
    pub strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    linked: bool,
}

impl StoredEnrichment {
    pub(crate) fn direct(
        enrichment: Enrichment,
        via: Arc<dyn IReadonlyRepository>,
        persist: Option<Arc<dyn ICRUDRepository>>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> Self {
        Self::direct_bulk(enrichment, via, persist, None, strategy)
    }

    pub(crate) fn direct_bulk(
        enrichment: Enrichment,
        via: Arc<dyn IReadonlyRepository>,
        persist: Option<Arc<dyn ICRUDRepository>>,
        persist_bulk: Option<Arc<BulkCRUDRepository>>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> Self {
        Self {
            enrichment,
            via,
            persist,
            persist_bulk,
            link: None,
            strategy,
            linked: false,
        }
    }

    pub(crate) fn linked(
        enrichment: Enrichment,
        via: Arc<dyn IReadonlyRepository>,
        link: Arc<dyn ICRUDRepository>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> Self {
        Self {
            enrichment,
            via,
            persist: None,
            persist_bulk: None,
            link: Some(link),
            strategy,
            linked: true,
        }
    }

    pub(crate) async fn apply(
        &self,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        match self.linked {
            true => {
                self.strategy
                    .apply_linked(
                        self.via.clone(),
                        as_readonly(self.link.clone().expect("linked enrichment")),
                        &self.enrichment,
                        parents,
                        sources,
                    )
                    .await
            }
            false => {
                self.strategy
                    .apply(self.via.clone(), &self.enrichment, parents, sources)
                    .await
            }
        }
    }
}

impl Clone for StoredEnrichment {
    fn clone(&self) -> Self {
        Self {
            enrichment: self.enrichment.clone(),
            via: self.via.clone(),
            persist: self.persist.clone(),
            persist_bulk: self.persist_bulk.clone(),
            link: self.link.clone(),
            strategy: self.strategy.clone(),
            linked: self.linked,
        }
    }
}
