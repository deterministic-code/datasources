use crate::record::Record;
use crate::result::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Reader: Send + Sync {
    fn current(&self) -> Option<Record>;
    async fn next(&self) -> Result<bool>;
}
