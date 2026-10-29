use crate::command_middleware::CommandMiddleware;
use crate::entity::{entity_from_row, stored_row, Entity};
use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::StreamExt;
use std::any::TypeId;
use std::pin::Pin;
use std::sync::Arc;

#[async_trait]
pub trait IReadonlyRepository<T: Entity = Row>: IRepository {
    fn is_schema_required(&self) -> bool {
        true
    }

    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<T>>>
    where
        Self: Sized,
    {
        None
    }

    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>>;
    fn read<'a>(
        &'a self,
        query: Option<&'a dyn ReadInput>,
        parameters: Option<&'a [crate::Parameter]>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>>;
}

#[async_trait]
pub trait ICRUDRepository<T: Entity = Row>: IReadonlyRepository<T> {
    async fn create(&self, item: T) -> Result<T>;
    async fn update(&self, item: T, by_where: Option<&dyn IWhere>) -> Result<Response>;
    async fn delete(&self, item: T, by_where: Option<&dyn IWhere>) -> Result<Response>;
    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<T>>;

    fn id_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn add_middleware(&self, _middleware: Arc<dyn CommandMiddleware>) {}
}

struct AsReadonly<T: Entity>(Arc<dyn ICRUDRepository<T>>);

impl<T: Entity> IRepository for AsReadonly<T> {
    fn entity_type(&self) -> TypeId {
        self.0.entity_type()
    }
}

#[async_trait]
impl<T: Entity> IReadonlyRepository<T> for AsReadonly<T> {
    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        self.0.read_single(query, parameters, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn ReadInput>,
        parameters: Option<&'a [crate::Parameter]>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        self.0.read(query, parameters, options)
    }
}

pub fn as_readonly<T: Entity>(repo: Arc<dyn ICRUDRepository<T>>) -> Arc<dyn IReadonlyRepository<T>> {
    Arc::new(AsReadonly(repo))
}

pub(crate) fn map_row<T: Entity>(row: Row) -> Result<T> {
    entity_from_row(row)
}

pub(crate) fn to_stored<T: Entity>(item: &T) -> Row {
    stored_row(item)
}

pub(crate) fn map_stream<'a, T: Entity>(
    rows: Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>>,
) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
    Box::pin(rows.map(|row| row.and_then(entity_from_row)))
}
