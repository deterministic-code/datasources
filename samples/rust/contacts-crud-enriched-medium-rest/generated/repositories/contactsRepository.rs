use datasources::{Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::Contact;
use super::addressesRepository::AddressesRepository;
use super::phonesRepository::PhonesRepository;
use super::contactSettingsRepository::ContactSettingsRepository;
use super::contactPreferencesRepository::ContactPreferencesRepository;
use super::contactSourcesRepository::ContactSourcesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct ContactsRepository {
    pub(crate) inner: Arc<CRUDRepository<Contact>>,
    pub address: AddressesRepository,
    pub phone: PhonesRepository,
    pub contact_settings: ContactSettingsRepository,
    pub contact_preferences: ContactPreferencesRepository,
    pub contact_source: ContactSourcesRepository,
}

impl ContactsRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        options.create_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        options.create_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let address = AddressesRepository::new(Arc::clone(&connection))?;
        let phone = PhonesRepository::new(Arc::clone(&connection))?;
        let contact_settings = ContactSettingsRepository::new(Arc::clone(&connection))?;
        let contact_preferences = ContactPreferencesRepository::new(Arc::clone(&connection))?;
        let contact_source = ContactSourcesRepository::new(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<Contact>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), &address, None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), &phone, None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &contact_settings, None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), &contact_preferences, None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &contact_source, None);
        repo.add_enriched(Enrichment::new("description", "contact_source_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &contact_source, None);
        Ok(Self { inner: Arc::new(repo), address, phone, contact_settings, contact_preferences, contact_source })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Contact>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        let address = AddressesRepository::new(Arc::clone(&connection))?;
        let phone = PhonesRepository::new(Arc::clone(&connection))?;
        let contact_settings = ContactSettingsRepository::new(Arc::clone(&connection))?;
        let contact_preferences = ContactPreferencesRepository::new(Arc::clone(&connection))?;
        let contact_source = ContactSourcesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<Contact>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), &address, None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), &phone, None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &contact_settings, None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), &contact_preferences, None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &contact_source, None);
        repo.add_enriched(Enrichment::new("description", "contact_source_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &contact_source, None);
        Ok(repo)
    }
}

impl Deref for ContactsRepository {
    type Target = CRUDRepository<Contact>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<Contact> for &ContactsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<Contact>>,
        Option<Arc<dyn ICRUDRepository<Contact>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<Contact>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<Contact>>),
        )
    }
}

impl IRepository for ContactsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }
}

#[async_trait]
impl IReadonlyRepository<Contact> for ContactsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<Contact>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Contact>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Contact>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Contact>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Contact>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<Contact> for ContactsRepository {
    async fn create(&self, item: Contact) -> Result<Contact> {
        self.inner.create(item).await
    }

    async fn update(&self, item: Contact, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: Contact, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Contact>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn contacts_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        options.create_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_source_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into(), "contact_source_id".into()];
        options.create_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let address = Arc::new(super::addressesRepository::addresses_row_repository(Arc::clone(&connection))?);
        let phone = Arc::new(super::phonesRepository::phones_row_repository(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contactSettingsRepository::contact_settings_row_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contactPreferencesRepository::contact_preferences_row_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contactSourcesRepository::contact_sources_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address), None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone), None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings), None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences), None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source), None);
        repo.add_enriched(Enrichment::new("description", "contact_source_id", "id", Some("description".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source), None);
        Ok(repo)
}
