use crate::command::Command;
use crate::connection::Connection;
use crate::repository::Repository;
use std::sync::Arc;

pub trait Datasource: Send + Sync {
    fn provider(&self) -> &str;
    fn create_connection(&self) -> Arc<dyn Connection>;
    fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command>;
    fn create_repository(
        &self,
        connection: Option<Arc<dyn Connection>>,
        table: Option<&str>,
    ) -> Arc<dyn Repository>;
}
