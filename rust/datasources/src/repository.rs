use crate::query::Query;
use crate::record::Record;
use crate::response::Response;
use crate::result::Result;
use crate::where_trait::Where;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Record>>;
    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Record>>;
    async fn add(&self, obj: &Record) -> Result<Response>;
    async fn update(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response>;
    async fn delete(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response>;
}
