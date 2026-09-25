use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait Reader<T = Row>: Send + Sync {
    fn current(&self) -> Option<T>;
    async fn next(&self) -> Result<bool>;
    async fn next_result(&self) -> Result<bool> {
        Ok(false)
    }
}
