use crate::record::Record;
use crate::result::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Reader<T = Record>: Send + Sync {
    fn current(&self) -> Option<T>;
    async fn next(&self) -> Result<bool>;
}
