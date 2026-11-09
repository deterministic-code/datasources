use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::Contact;
use super::addressesRepository::AddressesRepository;
use super::emailsRepository::EmailsRepository;
use super::phonesRepository::PhonesRepository;
use super::contactAttributesRepository::ContactAttributesRepository;
use super::labelsRepository::LabelsRepository;
use super::contactLabelsRepository::ContactLabelsRepository;
use super::gendersRepository::GendersRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct ContactsRepository {
    pub(crate) inner: Arc<CRUDRepository<Contact>>,
    pub address: AddressesRepository,
    pub email: EmailsRepository,
    pub phone: PhonesRepository,
    pub contact_attribute: ContactAttributesRepository,
    pub label: LabelsRepository,
    pub contact_label: ContactLabelsRepository,
    pub gender: GendersRepository,
}

impl ContactsRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
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
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("given_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("additional_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("pronunciation".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("family_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("nick_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("job_title".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("organization".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("is_starred".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
        ("is_favorite".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
        ("is_deleted".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
        ("gender_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "given_name".into(), "additional_name".into(), "pronunciation".into(), "family_name".into(), "nick_name".into(), "job_title".into(), "organization".into(), "is_starred".into(), "is_favorite".into(), "is_deleted".into(), "gender_type_id".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "given_name".into(), "additional_name".into(), "pronunciation".into(), "family_name".into(), "nick_name".into(), "job_title".into(), "organization".into(), "is_starred".into(), "is_favorite".into(), "is_deleted".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "given_name".into(), "additional_name".into(), "pronunciation".into(), "family_name".into(), "nick_name".into(), "job_title".into(), "organization".into(), "is_starred".into(), "is_favorite".into(), "is_deleted".into()]);

        let address = AddressesRepository::new(Arc::clone(&connection))?;
        let email = EmailsRepository::new(Arc::clone(&connection))?;
        let phone = PhonesRepository::new(Arc::clone(&connection))?;
        let contact_attribute = ContactAttributesRepository::new(Arc::clone(&connection))?;
        let label = LabelsRepository::new(Arc::clone(&connection))?;
        let contact_label = ContactLabelsRepository::new(Arc::clone(&connection))?;
        let gender = GendersRepository::new(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<Contact>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), &address, None);
        repo.add_enriched(Enrichment::new("emails", "id", "contact_id", None, EnrichmentPropertyType::List), &email, None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), &phone, None);
        repo.add_enriched(Enrichment::new("attributes", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &contact_attribute, None);
        repo.add_linked(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("contact_id", "label_id")), &label, Arc::clone(&contact_label.inner) as Arc<dyn ICRUDRepository<_>>, None);
        repo.add_enriched(Enrichment::new("gender_type_name", "gender_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &gender, None);
        Ok(Self { inner: Arc::new(repo), address, email, phone, contact_attribute, label, contact_label, gender })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Contact>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
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
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("given_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("additional_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("pronunciation".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("family_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("nick_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("job_title".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("organization".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("is_starred".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
            ("is_favorite".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
            ("is_deleted".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
            ("gender_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "given_name".into(), "additional_name".into(), "pronunciation".into(), "family_name".into(), "nick_name".into(), "job_title".into(), "organization".into(), "is_starred".into(), "is_favorite".into(), "is_deleted".into(), "gender_type_id".into()];
        let address = AddressesRepository::new(Arc::clone(&connection))?;
        let email = EmailsRepository::new(Arc::clone(&connection))?;
        let phone = PhonesRepository::new(Arc::clone(&connection))?;
        let contact_attribute = ContactAttributesRepository::new(Arc::clone(&connection))?;
        let label = LabelsRepository::new(Arc::clone(&connection))?;
        let gender = GendersRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<Contact>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), &address, None);
        repo.add_enriched(Enrichment::new("emails", "id", "contact_id", None, EnrichmentPropertyType::List), &email, None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), &phone, None);
        repo.add_enriched(Enrichment::new("attributes", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &contact_attribute, None);
        repo.add_enriched(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List), &label, None);
        repo.add_enriched(Enrichment::new("gender_type_name", "gender_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &gender, None);
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
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("given_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("additional_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("pronunciation".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("family_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("nick_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("job_title".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("organization".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("is_starred".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
        ("is_favorite".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
        ("is_deleted".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
        ("gender_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "given_name".into(), "additional_name".into(), "pronunciation".into(), "family_name".into(), "nick_name".into(), "job_title".into(), "organization".into(), "is_starred".into(), "is_favorite".into(), "is_deleted".into(), "gender_type_id".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "given_name".into(), "additional_name".into(), "pronunciation".into(), "family_name".into(), "nick_name".into(), "job_title".into(), "organization".into(), "is_starred".into(), "is_favorite".into(), "is_deleted".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "given_name".into(), "additional_name".into(), "pronunciation".into(), "family_name".into(), "nick_name".into(), "job_title".into(), "organization".into(), "is_starred".into(), "is_favorite".into(), "is_deleted".into()]);

        let address = Arc::new(super::addressesRepository::addresses_row_repository(Arc::clone(&connection))?);
        let email = Arc::new(super::emailsRepository::emails_row_repository(Arc::clone(&connection))?);
        let phone = Arc::new(super::phonesRepository::phones_row_repository(Arc::clone(&connection))?);
        let contact_attribute = Arc::new(super::contactAttributesRepository::contact_attributes_row_repository(Arc::clone(&connection))?);
        let label = Arc::new(super::labelsRepository::labels_row_repository(Arc::clone(&connection))?);
        let contact_label = Arc::new(super::contactLabelsRepository::contact_labels_row_repository(Arc::clone(&connection))?);
        let gender = Arc::new(super::gendersRepository::genders_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address), None);
        repo.add_enriched(Enrichment::new("emails", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&email), None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone), None);
        repo.add_enriched(Enrichment::new("attributes", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_attribute), None);
        repo.add_linked(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("contact_id", "label_id")), Arc::clone(&label), Arc::clone(&contact_label) as Arc<dyn ICRUDRepository<_>>, None);
        repo.add_enriched(Enrichment::new("gender_type_name", "gender_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&gender), None);
        Ok(repo)
}
