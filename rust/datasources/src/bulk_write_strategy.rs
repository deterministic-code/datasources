use crate::bulk_write::BulkWrite;
use crate::i_where::IWhere;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait BulkWriteStrategy: Send + Sync {
    async fn create(&self, items: &mut [Row], write: &dyn BulkWrite) -> Result<Response>;
    async fn update(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
        write: &dyn BulkWrite,
    ) -> Result<Response>;
    async fn delete(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
        write: &dyn BulkWrite,
    ) -> Result<Response>;
}
