use crate::datasource_error::DatasourceError;
use crate::i_repository::IRepository;
use crate::id_key::IdKey;
use crate::IWhere;
use crate::statement::Statement;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::value::Value;
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::stream::{iter, once};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub struct MockRepository {
    pub items: Mutex<Vec<Row>>,
    pub schema: Schema,
    read_single_error: Mutex<Option<String>>,
    get_error: Mutex<Option<String>>,
    add_error: Mutex<Option<String>>,
    update_error: Mutex<Option<String>>,
    delete_error: Mutex<Option<String>>,
}

impl MockRepository {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(Vec::new()),
            schema: Schema::default(),
            read_single_error: Mutex::new(None),
            get_error: Mutex::new(None),
            add_error: Mutex::new(None),
            update_error: Mutex::new(None),
            delete_error: Mutex::new(None),
        }
    }

    pub fn set_read_single_error(&self, message: impl Into<String>) {
        *self.read_single_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_read_error(&self, message: impl Into<String>) {
        *self.get_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_add_error(&self, message: impl Into<String>) {
        *self.add_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_update_error(&self, message: impl Into<String>) {
        *self.update_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_delete_error(&self, message: impl Into<String>) {
        *self.delete_error.lock().expect("mock") = Some(message.into());
    }

    pub async fn update(&self, item: Row, by_where: Option<&dyn crate::IWhere>) -> Result<Response> {
        ICRUDRepository::update(self, item, by_where).await
    }

    pub async fn delete(&self, item: Row, by_where: Option<&dyn crate::IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self, item, by_where).await
    }
}

impl IRepository for MockRepository {
    fn schema(&self) -> &Schema {
        &self.schema
    }
}

#[async_trait]
impl IReadonlyRepository for MockRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(
        &self,
        _query: Option<&dyn Statement>,
        _options: Option<&crate::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        if let Some(error) = &*self.read_single_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(self.items.lock().expect("mock").first().cloned())
    }

    fn read<'a>(
        &'a self,
        _query: Option<&'a dyn Statement>,
        _options: Option<&'a crate::ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        if let Some(error) = &*self.get_error.lock().expect("mock") {
            let error = error.clone();
            return Box::pin(once(async move { Err(DatasourceError::message(error)) }));
        }
        let items = self.items.lock().expect("mock").clone();
        Box::pin(iter(items.into_iter().map(Ok)))
    }

    async fn read_single_where(
        &self,
        _clause: &dyn IWhere,
        _options: Option<&crate::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        self.read_single(None, _options).await
    }

    fn read_where<'a>(
        &'a self,
        _clause: &'a dyn IWhere,
        _options: Option<&'a crate::ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        self.read(None, _options)
    }
}

#[async_trait]
impl ICRUDRepository for MockRepository {
    async fn create(&self, item: Row) -> Result<Row> {
        if let Some(error) = &*self.add_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        self.items.lock().expect("mock").push(item.clone());
        Ok(item)
    }

    async fn update(&self, _item: Row, _by_where: Option<&dyn crate::IWhere>) -> Result<Response> {
        if let Some(error) = &*self.update_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(Response::new(Value::Null, 1, None))
    }

    async fn delete(&self, _item: Row, _by_where: Option<&dyn crate::IWhere>) -> Result<Response> {
        if let Some(error) = &*self.delete_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(Response::new(Value::Null, 1, None))
    }

    async fn read_single_by_id(&self, _id: &dyn IdKey) -> Result<Option<Row>> {
        self.read_single(None, None).await
    }
}
