use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

#[async_trait]
pub trait IReadonlyRepository: IRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>>
    where
        Self: Sized,
    {
        None
    }

    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Option<Row>>;
    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Vec<Row>>;
}

#[async_trait]
pub trait ICRUDRepository: IReadonlyRepository {
    async fn create(&self, item: &mut Row) -> Result<Row>;
    async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>;
    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>;
    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>>;

    fn id_names(&self) -> Vec<String> {
        Vec::new()
    }
}

struct AsReadonly(Arc<dyn ICRUDRepository>);

impl IRepository for AsReadonly {
    fn entity_type(&self) -> TypeId {
        self.0.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for AsReadonly {
    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Option<Row>> {
        self.0.read_single(query, parameters).await
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Vec<Row>> {
        self.0.read(query, parameters).await
    }
}

pub fn as_readonly(repo: Arc<dyn ICRUDRepository>) -> Arc<dyn IReadonlyRepository> {
    Arc::new(AsReadonly(repo))
}
