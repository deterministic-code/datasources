use crate::built_repository::BuiltRepository;
use crate::connection::Connection;
use crate::crud_repository::CRUDRepository;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::enriched::Enriched;
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
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use async_trait::async_trait;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct BulkCRUDRepository {
    crud: CRUDRepository,
}

impl BulkCRUDRepository {
    pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self> {
        Ok(Self {
            crud: CRUDRepository::new(connection, options)?,
        })
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        Self {
            crud: CRUDRepository::from_built(built),
        }
    }

    pub fn memory(items: Vec<Row>) -> Self {
        Self::memory_with(items, None)
    }

    pub fn memory_with(items: Vec<Row>, write: Option<Arc<dyn WriteEnrichmentStrategy>>) -> Self {
        Self {
            crud: CRUDRepository::memory_with(items, write),
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
        self.crud
            .add_enriched(into, from, via, by, select, many, strategy);
        self
    }

    pub async fn create_bulk(&self, items: &mut [Row]) -> Result<Response> {
        let mut result = Value::Null;
        let mut rows_affected = 0;
        for item in items {
            let created = self.crud.create(item).await?;
            result = created
                .get("id")
                .or_else(|| created.get("Id"))
                .cloned()
                .unwrap_or(Value::Null);
            rows_affected += 1;
        }
        Ok(Response::new(result, rows_affected))
    }

    pub async fn update_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        fold(items, |item| self.crud.update(item, by_where)).await
    }

    pub async fn delete_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        fold(items, |item| self.crud.delete(item, by_where)).await
    }
}

async fn fold<'a, F, Fut>(items: &'a [Row], mut write: F) -> Result<Response>
where
    F: FnMut(&'a Row) -> Fut,
    Fut: std::future::Future<Output = Result<Response>>,
{
    let mut result = Value::Null;
    let mut rows_affected = 0;
    for item in items {
        let response = write(item).await?;
        result = response.result;
        rows_affected += response.rows_affected;
    }
    Ok(Response::new(result, rows_affected))
}

impl Deref for BulkCRUDRepository {
    type Target = CRUDRepository;

    fn deref(&self) -> &Self::Target {
        &self.crud
    }
}

impl DerefMut for BulkCRUDRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.crud
    }
}

#[async_trait]
impl Enriched for BulkCRUDRepository {
    async fn enrich(&self, items: &mut [Row]) -> Result<()> {
        self.crud.enrich(items).await
    }
}

impl IRepository for BulkCRUDRepository {
    fn entity_type(&self) -> TypeId {
        self.crud.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for BulkCRUDRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Option<Row>> {
        self.crud.read_single(query, parameters).await
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Vec<Row>> {
        self.crud.read(query, parameters).await
    }
}

#[async_trait]
impl ICRUDRepository for BulkCRUDRepository {
    fn id_names(&self) -> Vec<String> {
        self.crud.id_names()
    }

    async fn create(&self, item: &mut Row) -> Result<Row> {
        self.crud.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        self.crud.read_single_by_id(id).await
    }
}
