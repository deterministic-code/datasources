use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait Reader<T = Row>: Send + Sync {
    fn current(&self) -> Option<T>;
    async fn next(&self) -> Result<bool>;
}
