use crate::enriched_crud_repository::EnrichedCRUDRepository;
use crate::i_repository::IRepository;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::repository_tree_node::RepositoryTreeNode;
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
                (),
            )
            .add_enriched(
                "contact_source_name",
                "contact_source_id",
                sources,
                "id",
                Some("name".into()),
                false,
                (),
            )
            .add_enriched(
                "addresses",
                "id",
                addresses,
                "contact_id",
                None,
                true,
                |node: &mut RepositoryTreeNode| {
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
            .add_enriched(
                "phones",
                "id",
                phones,
                "contact_id",
                None,
                true,
                |node: &mut RepositoryTreeNode| {
                    node.include(
                        phone_types,
                        "phone_type_name",
                        "phone_type_id",
                        "id",
                        Some("name".into()),
                        false,
                    );
                },
            );
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
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>> {
        self.inner.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>> {
        self.inner.read(query).await
    }
}

#[async_trait]
impl ICRUDRepository for ContactsService {
    async fn create(&self, item: &mut Row) -> Result<Response> {
        self.inner.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        self.inner.read_single_by_id(id).await
    }
}
