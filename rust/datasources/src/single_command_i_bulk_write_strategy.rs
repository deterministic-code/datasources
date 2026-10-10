use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::i_where::IWhere;
use crate::repository::ICRUDRepository;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use async_trait::async_trait;
use std::sync::{Arc, OnceLock, Weak};

#[derive(Default)]
pub struct SingleCommandIBulkWriteStrategy {
    repo: OnceLock<Weak<dyn ICRUDRepository>>,
}

impl SingleCommandIBulkWriteStrategy {
    pub fn new() -> Self {
        Self::default()
    }

    fn repo(&self) -> Result<Arc<dyn ICRUDRepository>> {
        self.repo
            .get()
            .and_then(Weak::upgrade)
            .ok_or_else(|| crate::DatasourceError::message("Repo required"))
    }
}

#[async_trait]
impl BulkWriteStrategy for SingleCommandIBulkWriteStrategy {
    fn set_repo(&self, repo: Arc<dyn ICRUDRepository>) {
        let _ = self.repo.set(Arc::downgrade(&repo));
    }

    async fn create(&self, items: &mut [Row]) -> Result<Response> {
        let repo = self.repo()?;
        for item in items.iter_mut() {
            repo.create(item).await?;
        }
        Ok(Response::new(Value::Null, items.len() as i64, None))
    }

    async fn update(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        let repo = self.repo()?;
        let mut result = Value::Null;
        let mut rows_affected = 0;
        for item in items {
            let response = repo.update(item, by_where).await?;
            result = response.result;
            rows_affected += response.rows_affected;
        }
        Ok(Response::new(result, rows_affected, None))
    }

    async fn delete(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        let repo = self.repo()?;
        let mut result = Value::Null;
        let mut rows_affected = 0;
        for item in items {
            let response = repo.delete(item, by_where).await?;
            result = response.result;
            rows_affected += response.rows_affected;
        }
        Ok(Response::new(result, rows_affected, None))
    }
}
