use crate::array_reader::ArrayReader;
use crate::datasource_error::DatasourceError;
use crate::reader::Reader;
use crate::record::Record;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct MockReader {
    inner: ArrayReader,
    error: Mutex<Option<String>>,
}

impl MockReader {
    pub fn new(rows: Vec<Record>) -> Self {
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
impl Reader for MockReader {
    fn current(&self) -> Option<Record> {
        self.inner.current()
    }

    async fn next(&self) -> Result<bool> {
        if let Some(error) = &*self.error.lock().expect("mock") {
            return Err(DatasourceError::message(error.clone()));
        }
        self.inner.next().await
    }
}
