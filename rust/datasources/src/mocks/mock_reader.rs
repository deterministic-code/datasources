use crate::array_reader::ArrayReader;
use crate::datasource_error::DatasourceError;
use crate::reader::Reader;
use crate::record::Record;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct MockReader<T = Record> {
    inner: ArrayReader<T>,
    error: Mutex<Option<String>>,
}

impl<T> MockReader<T> {
    pub fn new(rows: Vec<T>) -> Self {
        Self {
            inner: ArrayReader::new(rows),
            error: Mutex::new(None),
        }
    }

    pub fn set_error(&self, message: impl Into<String>) {
        *self.error.lock().expect("mock") = Some(message.into());
    }
}

#[async_trait]
impl<T> Reader<T> for MockReader<T>
where
    T: Clone + Send + Sync,
{
    fn current(&self) -> Option<T> {
        self.inner.current()
    }

    async fn next(&self) -> Result<bool> {
        if let Some(error) = &*self.error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        self.inner.next().await
    }
}
