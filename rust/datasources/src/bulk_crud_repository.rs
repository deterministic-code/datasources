use crate::enriched::Enriched;
use crate::i_repository::IRepository;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::repository_tree_node::RepositoryTreeNode;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::statement::Statement;
use crate::value::Value;
use crate::where_trait::Where;
use crate::EnrichedCRUDRepository;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct BulkCRUDRepository {
    inner: EnrichedCRUDRepository,
}

impl BulkCRUDRepository {
    pub fn new(inner: Arc<dyn ICRUDRepository>) -> Self {
        Self {
            inner: EnrichedCRUDRepository::new(inner),
        }
    }

    pub fn add_enriched(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn IReadonlyRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> &mut Self {
        self.inner.add_enriched(into, from, via, by, select, many);
        self
    }

    pub fn add_enriched_with(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn IReadonlyRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl FnOnce(&mut RepositoryTreeNode),
    ) -> &mut Self {
        self.inner
            .add_enriched_with(into, from, via, by, select, many, nest);
        self
    }

    pub fn add_enriched_repo(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn ICRUDRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> &mut Self {
        self.inner
            .add_enriched_repo(into, from, via, by, select, many);
        self
    }

    pub fn add_enriched_repo_with(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn ICRUDRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl FnOnce(&mut RepositoryTreeNode),
    ) -> &mut Self {
        self.inner
            .add_enriched_repo_with(into, from, via, by, select, many, nest);
        self
    }

    pub async fn create_bulk(&self, items: &mut [Row]) -> Result<Response> {
        let mut result = Value::Null;
        let mut rows_affected = 0;
        for item in items {
            let response = self.inner.create(item).await?;
            result = response.result;
            rows_affected += response.rows_affected;
        }
        Ok(Response::new(result, rows_affected))
    }

    pub async fn update_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn Where>,
    ) -> Result<Response> {
        fold(items, |item| self.inner.update(item, by_where)).await
    }

    pub async fn delete_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn Where>,
    ) -> Result<Response> {
        fold(items, |item| self.inner.delete(item, by_where)).await
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

#[async_trait]
impl Enriched for BulkCRUDRepository {
    async fn enrich(&self, items: &mut [Row]) -> Result<()> {
        self.inner.enrich(items).await
    }
}

impl IRepository for BulkCRUDRepository {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for BulkCRUDRepository {
    async fn read_single(&self, query: Option<&dyn Statement>) -> Result<Option<Row>> {
        self.inner.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn Statement>) -> Result<Vec<Row>> {
        self.inner.read(query).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.inner.read_single_where(where_clause).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.inner.read_where(where_clause).await
    }
}

#[async_trait]
impl ICRUDRepository for BulkCRUDRepository {
    async fn create(&self, item: &mut Row) -> Result<Response> {
        self.inner.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }
}
