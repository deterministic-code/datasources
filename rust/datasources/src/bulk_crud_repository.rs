use crate::enriched::Enriched;
use crate::query::Query;
use crate::repository::{ReadonlyRepository, CRUDRepository};
use crate::repository_tree_node::RepositoryTreeNode;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::where_trait::Where;
use crate::DeepCRUDRepository;
use async_trait::async_trait;
use std::sync::Arc;

pub struct BulkCRUDRepository {
    inner: DeepCRUDRepository,
}

impl BulkCRUDRepository {
    pub fn new(inner: Arc<dyn CRUDRepository>) -> Self {
        Self {
            inner: DeepCRUDRepository::new(inner),
        }
    }

    pub fn add_enriched(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn ReadonlyRepository>,
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
        via: Arc<dyn ReadonlyRepository>,
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
        via: Arc<dyn CRUDRepository>,
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
        via: Arc<dyn CRUDRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl FnOnce(&mut RepositoryTreeNode),
    ) -> &mut Self {
        self.inner
            .add_enriched_repo_with(into, from, via, by, select, many, nest);
        self
    }

    pub async fn create_bulk(&self, items: &[Row]) -> Result<Response> {
        fold(items, |item| self.inner.create(item)).await
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

#[async_trait]
impl ReadonlyRepository for BulkCRUDRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        self.inner.get_single(query).await
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        self.inner.get(query).await
    }

    async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.inner.get_single_where(where_clause).await
    }

    async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.inner.get_where(where_clause).await
    }
}

#[async_trait]
impl CRUDRepository for BulkCRUDRepository {
    async fn create(&self, item: &Row) -> Result<Response> {
        self.inner.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }
}
