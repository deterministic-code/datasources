use super::not_implemented::fail;
use crate::i_repository::IRepository;
use crate::query::Query;
use crate::repository::{CRUDRepository, ReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
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

impl IRepository for StubRepository {}

#[async_trait]
impl ReadonlyRepository for StubRepository {
    async fn read_single(&self, _query: Option<&dyn Query>) -> Result<Option<Row>> {
        Err(fail(&self.provider))
    }

    async fn read(&self, _query: Option<&dyn Query>) -> Result<Vec<Row>> {
        Err(fail(&self.provider))
    }

    async fn read_single_where(&self, _where_clause: &dyn Where) -> Result<Option<Row>> {
        self.read_single(None).await
    }

    async fn read_where(&self, _where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.read(None).await
    }
}

#[async_trait]
impl CRUDRepository for StubRepository {
    async fn create(&self, _item: &Row) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn update(&self, _item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn delete(&self, _item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(fail(&self.provider))
    }
}
