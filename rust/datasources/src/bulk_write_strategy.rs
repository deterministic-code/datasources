use crate::command::Command;
use crate::connection::Connection;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::IWhere;
use crate::repository::ICRUDRepository;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait BulkWriteStrategy: Send + Sync {
    fn set_repo(&self, repo: Arc<dyn ICRUDRepository>) {
        let _ = repo;
    }

    fn attach(
        &self,
        command: Arc<dyn Command>,
        options: CRUDRepositoryOptions,
        connection: Arc<dyn Connection>,
    ) {
        let _ = (command, options, connection);
    }

    fn persist_children(&self) -> bool {
        false
    }

    async fn create(&self, items: &mut [Row]) -> Result<Response>;
    async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>;
    async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>;
}
