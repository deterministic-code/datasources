use super::not_implemented::fail;
use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct StubRepository {
    provider: String,
    entity_type: TypeId,
}

impl StubRepository {
    pub fn new(provider: impl Into<String>) -> Self {
        Self::typed::<Row>(provider)
    }

    pub fn typed<T: 'static>(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            entity_type: TypeId::of::<T>(),
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
        _query: Option<&dyn ReadInput>,
        _parameters: Option<&[crate::Parameter]>,
        _options: Option<&crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        Err(fail(&self.provider))
    }

    async fn read(
        &self,
        _query: Option<&dyn ReadInput>,
        _parameters: Option<&[crate::Parameter]>,
        _options: Option<&crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<Row>> {
        Err(fail(&self.provider))
    }
}

#[async_trait]
impl ICRUDRepository for StubRepository {
    async fn create(&self, _item: &mut Row) -> Result<Row> {
        Err(fail(&self.provider))
    }

    async fn update(&self, _item: &Row, _by_where: Option<&dyn IWhere>) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn delete(&self, _item: &Row, _by_where: Option<&dyn IWhere>) -> Result<Response> {
        Err(fail(&self.provider))
    }

    async fn read_single_by_id(&self, _id: &dyn IdKey) -> Result<Option<Row>> {
        Err(fail(&self.provider))
    }
}
