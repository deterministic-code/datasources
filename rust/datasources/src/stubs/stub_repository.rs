use super::not_implemented::fail;
use crate::query::Query;
use crate::row::Row;
use crate::repository::{ReadonlyRepository, Repository};
use crate::response::Response;
use crate::result::Result;
use crate::where_trait::Where;
use async_trait::async_trait;

pub struct StubRepository {
    provider: String,
}

impl StubRepository {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
        }
    }
}

#[async_trait]
impl ReadonlyRepository for StubRepository {
    async fn get_single(&self, _query: Option<&dyn Query>) -> Result<Option<Row>> {
        Err(fail(&self.provider))
    }

    async fn get(&self, _query: Option<&dyn Query>) -> Result<Vec<Row>> {
        Err(fail(&self.provider))
    }

    async fn get_single_where(&self, _where_clause: &dyn Where) -> Result<Option<Row>> {
        self.get_single(None).await
    }

    async fn get_where(&self, _where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.get(None).await
    }
}

#[async_trait]
impl Repository for StubRepository {
    async fn add(&self, _item: &Row) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn update(&self, _item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn delete(&self, _item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(fail(&self.provider))
    }
}
