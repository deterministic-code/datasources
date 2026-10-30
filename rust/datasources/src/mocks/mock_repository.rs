use crate::datasource_error::DatasourceError;
use crate::i_repository::IRepository;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::stream::{iter, once};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub struct MockRepository {
    pub items: Mutex<Vec<Row>>,
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

    pub async fn update(&self, item: Row) -> Result<Response> {
        ICRUDRepository::update(self, item, None).await
    }

    pub async fn delete(&self, item: Row) -> Result<Response> {
        ICRUDRepository::delete(self, item, None).await
    }
}

impl IRepository for MockRepository {}

#[async_trait]
impl IReadonlyRepository for MockRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(&self, _query: &dyn ReadInput) -> Result<Option<Row>> {
        if let Some(error) = &*self.read_single_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(self.items.lock().expect("mock").first().cloned())
    }

    fn read<'a>(&'a self, _query: &'a dyn ReadInput) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        if let Some(error) = &*self.get_error.lock().expect("mock") {
            let error = error.clone();
            return Box::pin(once(async move { Err(DatasourceError::message(error)) }));
        }
        let items = self.items.lock().expect("mock").clone();
        Box::pin(iter(items.into_iter().map(Ok)))
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
        self.read_single(&()).await
    }
}
