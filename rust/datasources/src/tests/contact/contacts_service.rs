use crate::deep_crud_repository::DeepCRUDRepository;
use crate::query::Query;
use crate::repository::{ReadonlyRepository, Repository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ContactsService {
    inner: DeepCRUDRepository,
}

impl ContactsService {
    pub fn new(
        contacts: Arc<dyn Repository>,
        types: Arc<dyn ReadonlyRepository>,
        sources: Arc<dyn ReadonlyRepository>,
        addresses: Arc<dyn Repository>,
        phones: Arc<dyn Repository>,
    ) -> Self {
        let mut inner = DeepCRUDRepository::new(contacts);
        inner
            .add_enriched(
                "contact_type_name",
                "contact_type_id",
                types,
                "id",
                Some("name".into()),
                false,
            )
            .add_enriched(
                "contact_source_name",
                "contact_source_id",
                sources,
                "id",
                Some("name".into()),
                false,
            )
            .add_enriched_repo("addresses", "id", addresses, "contact_id", None, true)
            .add_enriched_repo("phones", "id", phones, "contact_id", None, true);
        Self { inner }
    }
}

#[async_trait]
impl ReadonlyRepository for ContactsService {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        self.inner.get_single(query).await
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        self.inner.get(query).await
    }

    async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.inner.get_single_where(where_clause).await
    }

    async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.inner.get_where(where_clause).await
    }
}

#[async_trait]
impl Repository for ContactsService {
    async fn add(&self, item: &Row) -> Result<Response> {
        self.inner.add(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }
}
