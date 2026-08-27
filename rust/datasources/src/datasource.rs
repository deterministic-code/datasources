use crate::command::Command;
use crate::connection::Connection;
use crate::repository::Repository;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Datasource: Send + Sync {
    fn provider(&self) -> &str;
    async fn create_connection(&self) -> Result<Arc<dyn Connection>>;
    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command>;
    fn create_repository(
        &self,
        connection: Option<Arc<dyn Connection>>,
    ) -> Arc<dyn Repository>;
}
