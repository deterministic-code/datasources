use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::readonly_repository::ReadonlyRepository;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct ItemsRepository {
    inner: ReadonlyRepository,
}

impl ItemsRepository {
    pub fn new(items: Vec<Row>) -> Self {
        Self {
            inner: ReadonlyRepository::memory(items),
        }
    }

    pub fn add_enriched(
        &mut self,
        enrichment: crate::enrichment::Enrichment,
        via: impl EnrichmentVia,
        strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>,
    ) -> &mut Self {
        self.inner.add_enriched(enrichment, via, strategy);
        self
    }
}

impl Deref for ItemsRepository {
    type Target = ReadonlyRepository;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ItemsRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl IRepository for ItemsRepository {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for ItemsRepository {
    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Option<Row>> {
        self.inner.read_single(query, parameters).await
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Vec<Row>> {
        self.inner.read(query, parameters).await
    }
}
