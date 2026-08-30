use crate::query::Query;
use crate::row::Row;
use crate::response::Response;
use crate::result::Result;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ReadonlyRepository: Send + Sync {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>>;
    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>>;
    async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>>;
    async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>>;
}

#[async_trait]
pub trait Repository: ReadonlyRepository {
    async fn add(&self, obj: &Row) -> Result<Response>;
    async fn update(&self, obj: &Row, by_where: Option<&dyn Where>) -> Result<Response>;
    async fn delete(&self, obj: &Row, by_where: Option<&dyn Where>) -> Result<Response>;
}

struct AsReadonly(Arc<dyn Repository>);

#[async_trait]
impl ReadonlyRepository for AsReadonly {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        self.0.get_single(query).await
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        self.0.get(query).await
    }

    async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.0.get_single_where(where_clause).await
    }

    async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.0.get_where(where_clause).await
    }
}

pub fn as_readonly(repo: Arc<dyn Repository>) -> Arc<dyn ReadonlyRepository> {
    Arc::new(AsReadonly(repo))
}
