use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use async_trait::async_trait;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct ListRepository {
    inner: BulkCRUDRepository,
}

impl ListRepository {
    pub fn new(items: Vec<Row>) -> Self {
        Self {
            inner: BulkCRUDRepository::memory(items),
        }
    }

    pub fn with_write(items: Vec<Row>, write: Arc<dyn WriteEnrichmentStrategy>) -> Self {
        Self {
            inner: BulkCRUDRepository::memory_with(items, Some(write)),
        }
    }

    pub fn snapshot(&self) -> Vec<Row> {
        self.inner
            .items()
            .expect("memory store")
            .snapshot()
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

impl Deref for ListRepository {
    type Target = BulkCRUDRepository;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ListRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl IRepository for ListRepository {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for ListRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

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

#[async_trait]
impl ICRUDRepository for ListRepository {
    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    async fn create(&self, item: &mut Row) -> Result<Row> {
        self.inner.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        self.inner.read_single_by_id(id).await
    }
}
