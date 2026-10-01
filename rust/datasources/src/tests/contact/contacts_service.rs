use crate::crud_repository::CRUDRepository;
use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct ContactsService {
    contacts: CRUDRepository,
}

impl ContactsService {
    pub fn new(
        mut contacts: CRUDRepository,
        types: Arc<dyn IReadonlyRepository>,
        sources: Arc<dyn IReadonlyRepository>,
        address_types: Arc<dyn IReadonlyRepository>,
        phone_types: Arc<dyn IReadonlyRepository>,
        mut addresses: CRUDRepository,
        mut phones: CRUDRepository,
    ) -> Self {
        addresses.add_enriched(
            crate::enrichment::Enrichment::new(
                "AddressType",
                "AddressType",
                "Id",
                None,
                crate::EnrichmentPropertyType::Object,
            ),
            address_types,
            None,
        );
        phones.add_enriched(
            crate::enrichment::Enrichment::new(
                "PhoneType",
                "PhoneType",
                "Id",
                None,
                crate::EnrichmentPropertyType::Object,
            ),
            phone_types,
            None,
        );
        contacts
            .add_enriched(
                crate::enrichment::Enrichment::new(
                    "ContactType",
                    "ContactType",
                    "Id",
                    None,
                    crate::EnrichmentPropertyType::Object,
                ),
                types,
                None,
            )
            .add_enriched(
                crate::enrichment::Enrichment::new(
                    "ContactSource",
                    "ContactSource",
                    "Id",
                    None,
                    crate::EnrichmentPropertyType::Object,
                ),
                sources,
                None,
            )
            .add_enriched(
                crate::enrichment::Enrichment::new(
                    "Addresses",
                    "Id",
                    "Contact",
                    None,
                    crate::EnrichmentPropertyType::List,
                ),
                Arc::new(addresses),
                None,
            )
            .add_enriched(
                crate::enrichment::Enrichment::new(
                    "Phones",
                    "Id",
                    "Contact",
                    None,
                    crate::EnrichmentPropertyType::List,
                ),
                Arc::new(phones),
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

    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Option<Row>> {
        self.contacts.read_single(query, parameters).await
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Vec<Row>> {
        self.contacts.read(query, parameters).await
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
