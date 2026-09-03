use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait Enriched: Send + Sync {
    async fn enrich(&self, items: &mut [Row]) -> Result<()>;
}
