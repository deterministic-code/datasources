use crate::i_repository::IRepository;
use crate::query::Query;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

#[async_trait]
pub trait IReadonlyRepository: IRepository {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>>;
    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>>;
    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>>;
    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>>;
}

#[async_trait]
pub trait ICRUDRepository: IReadonlyRepository {
    async fn create(&self, item: &mut Row) -> Result<Response>;
    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response>;
    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response>;
}

struct AsReadonly(Arc<dyn ICRUDRepository>);

impl IRepository for AsReadonly {
    fn entity_type(&self) -> TypeId {
        self.0.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for AsReadonly {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        self.0.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        self.0.read(query).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.0.read_single_where(where_clause).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.0.read_where(where_clause).await
    }
}

pub fn as_readonly(repo: Arc<dyn ICRUDRepository>) -> Arc<dyn IReadonlyRepository> {
    Arc::new(AsReadonly(repo))
}
