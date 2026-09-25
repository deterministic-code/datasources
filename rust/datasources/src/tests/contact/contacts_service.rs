use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::tests::list_repository::ListRepository;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct ContactsService {
    contacts: ListRepository,
}

impl ContactsService {
    pub fn new(
        mut contacts: ListRepository,
        types: Arc<dyn IReadonlyRepository>,
        sources: Arc<dyn IReadonlyRepository>,
        address_types: Arc<dyn IReadonlyRepository>,
        phone_types: Arc<dyn IReadonlyRepository>,
        mut addresses: ListRepository,
        mut phones: ListRepository,
    ) -> Self {
        addresses.add_enriched(
            "address_type_name",
            "address_type_id",
            address_types,
            "id",
            Some("name".into()),
            false,
            None,
        );
        phones.add_enriched(
            "phone_type_name",
            "phone_type_id",
            phone_types,
            "id",
            Some("name".into()),
            false,
            None,
        );
        contacts
            .add_enriched(
                "contact_type_name",
                "contact_type_id",
                types,
                "id",
                Some("name".into()),
                false,
                None,
            )
            .add_enriched(
                "contact_source_name",
                "contact_source_id",
                sources,
                "id",
                Some("name".into()),
                false,
                None,
            )
            .add_enriched(
                "addresses",
                "id",
                Arc::new(addresses),
                "contact_id",
                None,
                true,
                None,
            )
            .add_enriched(
                "phones",
                "id",
                Arc::new(phones),
                "contact_id",
                None,
                true,
                None,
            );
        Self { contacts }
    }
}

impl IRepository for ContactsService {
    fn entity_type(&self) -> TypeId {
        self.contacts.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for ContactsService {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>> {
        self.contacts.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>> {
        self.contacts.read(query).await
    }
}

#[async_trait]
impl ICRUDRepository for ContactsService {
    async fn create(&self, item: &mut Row) -> Result<Row> {
        self.contacts.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.contacts.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.contacts.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        self.contacts.read_single_by_id(id).await
    }
}
