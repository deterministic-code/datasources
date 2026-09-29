use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::i_where::IWhere;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use async_trait::async_trait;

pub struct SingleCommandIBulkWriteStrategy;

#[async_trait]
impl BulkWriteStrategy for SingleCommandIBulkWriteStrategy {
    async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response> {
        let mut result = Value::Null;
        for item in items.iter_mut() {
            result = repo.create_one(item).await?;
        }
        Ok(Response::new(result, items.len() as i64))
    }

    async fn update(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
        repo: &BulkCRUDRepository,
    ) -> Result<Response> {
        let mut result = Value::Null;
        let mut rows_affected = 0;
        for item in items {
            let response = repo.update_one(item, by_where).await?;
            result = response.result;
            rows_affected += response.rows_affected;
        }
        Ok(Response::new(result, rows_affected))
    }

    async fn delete(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
        repo: &BulkCRUDRepository,
    ) -> Result<Response> {
        let mut result = Value::Null;
        let mut rows_affected = 0;
        for item in items {
            let response = repo.delete_one(item, by_where).await?;
            result = response.result;
            rows_affected += response.rows_affected;
        }
        Ok(Response::new(result, rows_affected))
    }
}
