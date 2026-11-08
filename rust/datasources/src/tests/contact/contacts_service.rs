use crate::crud_repository::CRUDRepository;
use crate::i_repository::IRepository;
use crate::id_key::IdKey;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::statement::Statement;
use crate::IWhere;
use async_trait::async_trait;
use futures_core::Stream;
use std::any::TypeId;
use std::pin::Pin;
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
        query: Option<&dyn Statement>,
        options: Option<&crate::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        self.contacts.read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a crate::ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        self.contacts.read(query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&crate::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        self.contacts.read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a crate::ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        self.contacts.read_where(clause, options)
    }
}

#[async_trait]
impl ICRUDRepository for ContactsService {
    async fn create(&self, item: Row) -> Result<Row> {
        self.contacts.create(item).await
    }

    async fn update(&self, item: Row, by_where: Option<&dyn crate::IWhere>) -> Result<Response> {
        ICRUDRepository::update(&self.contacts, item, by_where).await
    }

    async fn delete(&self, item: Row, by_where: Option<&dyn crate::IWhere>) -> Result<Response> {
        ICRUDRepository::delete(&self.contacts, item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        self.contacts.read_single_by_id(id).await
    }

    fn add_middleware(&self, middleware: Arc<dyn crate::CommandMiddleware>) {
        self.contacts.add_middleware(middleware);
    }
}
