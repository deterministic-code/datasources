use super::not_implemented::fail;
use crate::reader::Reader;
use crate::record::Record;
use crate::result::Result;
use async_trait::async_trait;

pub struct StubReader {
    provider: String,
}

impl StubReader {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
        }
    }
}

#[async_trait]
impl Reader for StubReader {
    fn current(&self) -> Option<Record> {
        None
    }

    async fn next(&self) -> Result<bool> {
        Err(fail(&self.provider))
    }
}
