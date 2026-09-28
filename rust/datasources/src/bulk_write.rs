use crate::bulk_command::BulkCommand;
use crate::command::Command;
use crate::i_where::IWhere;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait BulkWrite: Send + Sync {
    fn command(&self) -> Arc<dyn Command>;
    async fn create_one(&self, item: &mut Row) -> Result<Value>;
    async fn update_one(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>;
    async fn delete_one(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>;
    fn inserts(&self, items: &[Row]) -> Result<Vec<BulkCommand>>;
    fn updates(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Vec<BulkCommand>>;
    fn deletes(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Vec<BulkCommand>>;
    fn rows(&self, items: &[Row]) -> Result<Vec<Row>>;
}
