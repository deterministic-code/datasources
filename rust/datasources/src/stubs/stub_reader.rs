use super::not_implemented::fail;
use crate::reader::Reader;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::marker::PhantomData;

pub struct StubReader<T = Row> {
    provider: String,
    _item: PhantomData<T>,
}

impl<T> StubReader<T> {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            _item: PhantomData,
        }
    }
}

#[async_trait]
impl<T> Reader<T> for StubReader<T>
where
    T: Send + Sync,
{
    fn current(&self) -> Option<T> {
        None
    }

    async fn next(&self) -> Result<bool> {
        Err(fail(&self.provider))
    }
}
