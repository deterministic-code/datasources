use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::i_where::IWhere;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait BulkWriteStrategy: Send + Sync {
    async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response>;
    async fn update(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
        repo: &BulkCRUDRepository,
    ) -> Result<Response>;
    async fn delete(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
        repo: &BulkCRUDRepository,
    ) -> Result<Response>;
}
