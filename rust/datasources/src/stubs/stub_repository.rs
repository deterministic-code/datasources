use super::not_implemented::fail;
use crate::i_repository::IRepository;
use crate::id_key::IdKey;
use crate::IWhere;
use crate::statement::Statement;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::stream::once;
use std::any::TypeId;
use std::pin::Pin;
use std::sync::Arc;

pub struct StubRepository {
    provider: String,
    entity_type: TypeId,
}

impl StubRepository {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            entity_type: TypeId::of::<Row>(),
        }
    }
}

impl IRepository for StubRepository {
    fn entity_type(&self) -> TypeId {
        self.entity_type
    }
}

#[async_trait]
impl IReadonlyRepository for StubRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(
        &self,
        _query: Option<&dyn Statement>,
        _options: Option<&crate::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        Err(fail(&self.provider))
    }

    fn read<'a>(
        &'a self,
        _query: Option<&'a dyn Statement>,
        _options: Option<&'a crate::ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        let provider = self.provider.clone();
        Box::pin(once(async move { Err(fail(&provider)) }))
    }

    async fn read_single_where(
        &self,
        _clause: &dyn IWhere,
        _options: Option<&crate::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        Err(fail(&self.provider))
    }

    fn read_where<'a>(
        &'a self,
        _clause: &'a dyn IWhere,
        _options: Option<&'a crate::ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        let provider = self.provider.clone();
        Box::pin(once(async move { Err(fail(&provider)) }))
    }
}

#[async_trait]
impl ICRUDRepository for StubRepository {
    async fn create(&self, _item: Row) -> Result<Row> {
        Err(fail(&self.provider))
    }

    async fn update(&self, _item: Row, _by_where: Option<&dyn crate::IWhere>) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn delete(&self, _item: Row, _by_where: Option<&dyn crate::IWhere>) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn read_single_by_id(&self, _id: &dyn IdKey) -> Result<Option<Row>> {
        Err(fail(&self.provider))
    }
}
