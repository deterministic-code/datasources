use crate::datasource_error::DatasourceError;
use crate::query::Query;
use crate::row::Row;
use crate::repository::{ReadonlyRepository, Repository};
use crate::response::Response;
use crate::result::Result;
use crate::value::Value;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct MockRepository {
    pub items: Mutex<Vec<Row>>,
    get_single_error: Mutex<Option<String>>,
    get_error: Mutex<Option<String>>,
    add_error: Mutex<Option<String>>,
    update_error: Mutex<Option<String>>,
    delete_error: Mutex<Option<String>>,
}

impl MockRepository {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(Vec::new()),
            get_single_error: Mutex::new(None),
            get_error: Mutex::new(None),
            add_error: Mutex::new(None),
            update_error: Mutex::new(None),
            delete_error: Mutex::new(None),
        }
    }

    pub fn set_get_single_error(&self, message: impl Into<String>) {
        *self.get_single_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_get_error(&self, message: impl Into<String>) {
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

#[async_trait]
impl ReadonlyRepository for MockRepository {
    async fn get_single(&self, _query: Option<&dyn Query>) -> Result<Option<Row>> {
        if let Some(error) = &*self.get_single_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(self.items.lock().expect("mock").first().cloned())
    }

    async fn get(&self, _query: Option<&dyn Query>) -> Result<Vec<Row>> {
        if let Some(error) = &*self.get_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(self.items.lock().expect("mock").clone())
    }

    async fn get_single_where(&self, _where_clause: &dyn Where) -> Result<Option<Row>> {
        self.get_single(None).await
    }

    async fn get_where(&self, _where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.get(None).await
    }
}

#[async_trait]
impl Repository for MockRepository {
    async fn add(&self, obj: &Row) -> Result<Response> {
        if let Some(error) = &*self.add_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        self.items.lock().expect("mock").push(obj.clone());
        Ok(Response::new(Value::Null, 1))
    }

    async fn update(&self, _obj: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        if let Some(error) = &*self.update_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(Response::new(Value::Null, 1))
    }

    async fn delete(&self, _obj: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        if let Some(error) = &*self.delete_error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        Ok(Response::new(Value::Null, 1))
    }
}
