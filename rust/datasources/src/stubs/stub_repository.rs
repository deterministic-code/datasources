use super::not_implemented::fail;
use crate::query::Query;
use crate::record::Record;
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
    async fn get_single(&self, _query: Option<&dyn Query>) -> Result<Option<Record>> {
        Err(fail(&self.provider))
    }

    async fn get(&self, _query: Option<&dyn Query>) -> Result<Vec<Record>> {
        Err(fail(&self.provider))
    }
}

#[async_trait]
impl Repository for StubRepository {
    async fn add(&self, _obj: &Record) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn update(&self, _obj: &Record, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn delete(&self, _obj: &Record, _by_where: Option<&dyn Where>) -> Result<Response> {
        Err(fail(&self.provider))
    }
}
