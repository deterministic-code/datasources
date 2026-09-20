use crate::deep_crud_repository::DeepCRUDRepository;
use crate::query::Query;
use crate::repository::{CRUDRepository, ReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct ContactsService {
    inner: DeepCRUDRepository,
}

impl ContactsService {
    pub fn new(
        contacts: Arc<dyn CRUDRepository>,
        types: Arc<dyn ReadonlyRepository>,
        sources: Arc<dyn ReadonlyRepository>,
        address_types: Arc<dyn ReadonlyRepository>,
        phone_types: Arc<dyn ReadonlyRepository>,
        addresses: Arc<dyn CRUDRepository>,
        phones: Arc<dyn CRUDRepository>,
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
            .add_enriched_repo_with(
                "addresses",
                "id",
                addresses,
                "contact_id",
                None,
                true,
                |node| {
                    node.include(
                        address_types,
                        "address_type_name",
                        "address_type_id",
                        "id",
                        Some("name".into()),
                        false,
                    );
                },
            )
            .add_enriched_repo_with("phones", "id", phones, "contact_id", None, true, |node| {
                node.include(
                    phone_types,
                    "phone_type_name",
                    "phone_type_id",
                    "id",
                    Some("name".into()),
                    false,
                );
            });
        Self { inner }
    }
}

#[async_trait]
impl ReadonlyRepository for ContactsService {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }

    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        self.inner.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        self.inner.read(query).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.inner.read_single_where(where_clause).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.inner.read_where(where_clause).await
    }
}

#[async_trait]
impl CRUDRepository for ContactsService {
    async fn create(&self, item: &Row) -> Result<Response> {
        self.inner.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }
}
