use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait DeleteNested: Send + Sync {
    async fn delete_nested(&self, rows: &mut [Row]) -> Result<()>;
}
