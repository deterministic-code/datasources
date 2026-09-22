use crate::enriched_crud_repository::EnrichedCRUDRepository;
use crate::i_repository::IRepository;
use crate::query::Query;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct ContactsService {
    inner: EnrichedCRUDRepository,
}

impl ContactsService {
    pub fn new(
        contacts: Arc<dyn ICRUDRepository>,
        types: Arc<dyn IReadonlyRepository>,
        sources: Arc<dyn IReadonlyRepository>,
        address_types: Arc<dyn IReadonlyRepository>,
        phone_types: Arc<dyn IReadonlyRepository>,
        addresses: Arc<dyn ICRUDRepository>,
        phones: Arc<dyn ICRUDRepository>,
    ) -> Self {
        let mut inner = EnrichedCRUDRepository::new(contacts);
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

impl IRepository for ContactsService {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for ContactsService {
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
impl ICRUDRepository for ContactsService {
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
