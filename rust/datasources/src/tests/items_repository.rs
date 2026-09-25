use crate::enriched::Enriched;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::mappers::Mapper;
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

    pub fn with_result(
        items: Vec<Row>,
        result: Box<dyn Mapper<Vec<Row>, Vec<Row>> + Send + Sync>,
    ) -> Self {
        Self {
            inner: ReadonlyRepository::memory_with(items, Some(result)),
        }
    }

    pub fn add_enriched(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: impl EnrichmentVia,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>,
    ) -> &mut Self {
        self.inner
            .add_enriched(into, from, via, by, select, many, strategy);
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
impl Enriched for ItemsRepository {
    async fn enrich(&self, items: &mut [Row]) -> Result<()> {
        self.inner.enrich(items).await
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
