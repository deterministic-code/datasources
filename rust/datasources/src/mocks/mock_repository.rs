use crate::datasource_error::DatasourceError;
use crate::i_repository::IRepository;
use crate::query::Query;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::sync::Mutex;

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
}

impl IRepository for MockRepository {}

#[async_trait]
impl IReadonlyRepository for MockRepository {
    async fn read_single(&self, _query: Option<&dyn Query>) -> Result<Option<Row>> {
        if let Some(error) = &*self.read_single_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(self.items.lock().expect("mock").first().cloned())
    }

    async fn read(&self, _query: Option<&dyn Query>) -> Result<Vec<Row>> {
        if let Some(error) = &*self.get_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(self.items.lock().expect("mock").clone())
    }

    async fn read_single_where(&self, _where_clause: &dyn Where) -> Result<Option<Row>> {
        self.read_single(None).await
    }

    async fn read_where(&self, _where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.read(None).await
    }
}

#[async_trait]
impl ICRUDRepository for MockRepository {
    async fn create(&self, item: &mut Row) -> Result<Response> {
        if let Some(error) = &*self.add_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        self.items.lock().expect("mock").push(item.clone());
        Ok(Response::new(Value::Null, 1))
    }

    async fn update(&self, _item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        if let Some(error) = &*self.update_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(Response::new(Value::Null, 1))
    }

    async fn delete(&self, _item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        if let Some(error) = &*self.delete_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(Response::new(Value::Null, 1))
    }
}
