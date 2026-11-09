use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::Email;
use super::emailAttributesRepository::EmailAttributesRepository;
use super::labelsRepository::LabelsRepository;
use super::emailLabelsRepository::EmailLabelsRepository;
use super::emailTypesRepository::EmailTypesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct EmailsRepository {
    pub(crate) inner: Arc<CRUDRepository<Email>>,
    pub email_attribute: EmailAttributesRepository,
    pub label: LabelsRepository,
    pub email_label: EmailLabelsRepository,
    pub email_type: EmailTypesRepository,
}

impl EmailsRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Email".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Email".into(), "emails".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("address".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("is_primary".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
        ("email_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address".into(), "display_name".into(), "is_primary".into(), "email_type_id".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address".into(), "display_name".into(), "is_primary".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address".into(), "display_name".into(), "is_primary".into()]);

        let email_attribute = EmailAttributesRepository::new(Arc::clone(&connection))?;
        let label = LabelsRepository::new(Arc::clone(&connection))?;
        let email_label = EmailLabelsRepository::new(Arc::clone(&connection))?;
        let email_type = EmailTypesRepository::new(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<Email>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("attributes", "id", "email_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &email_attribute, None);
        repo.add_linked(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("email_id", "label_id")), &label, Arc::clone(&email_label.inner) as Arc<dyn ICRUDRepository<_>>, None);
        repo.add_enriched(Enrichment::new("email_type_name", "email_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &email_type, None);
        repo.add_enriched(Enrichment::new("description", "email_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &email_type, None);
        Ok(Self { inner: Arc::new(repo), email_attribute, label, email_label, email_type })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Email>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "Email".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Email".into(), "emails".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("address".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("is_primary".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
            ("email_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address".into(), "display_name".into(), "is_primary".into(), "email_type_id".into()];
        let email_attribute = EmailAttributesRepository::new(Arc::clone(&connection))?;
        let label = LabelsRepository::new(Arc::clone(&connection))?;
        let email_type = EmailTypesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<Email>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("attributes", "id", "email_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &email_attribute, None);
        repo.add_enriched(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List), &label, None);
        repo.add_enriched(Enrichment::new("email_type_name", "email_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &email_type, None);
        repo.add_enriched(Enrichment::new("description", "email_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &email_type, None);
        Ok(repo)
    }
}

impl Deref for EmailsRepository {
    type Target = CRUDRepository<Email>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<Email> for &EmailsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<Email>>,
        Option<Arc<dyn ICRUDRepository<Email>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<Email>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<Email>>),
        )
    }
}

impl IRepository for EmailsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<Email> for EmailsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<Email>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Email>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Email>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Email>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Email>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<Email> for EmailsRepository {
    async fn create(&self, item: Email) -> Result<Email> {
        self.inner.create(item).await
    }

    async fn update(&self, item: Email, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: Email, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Email>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn emails_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Email".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Email".into(), "emails".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("address".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("is_primary".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: false, is_id: false }),
        ("email_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address".into(), "display_name".into(), "is_primary".into(), "email_type_id".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address".into(), "display_name".into(), "is_primary".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address".into(), "display_name".into(), "is_primary".into()]);

        let email_attribute = Arc::new(super::emailAttributesRepository::email_attributes_row_repository(Arc::clone(&connection))?);
        let label = Arc::new(super::labelsRepository::labels_row_repository(Arc::clone(&connection))?);
        let email_label = Arc::new(super::emailLabelsRepository::email_labels_row_repository(Arc::clone(&connection))?);
        let email_type = Arc::new(super::emailTypesRepository::email_types_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("attributes", "id", "email_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&email_attribute), None);
        repo.add_linked(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("email_id", "label_id")), Arc::clone(&label), Arc::clone(&email_label) as Arc<dyn ICRUDRepository<_>>, None);
        repo.add_enriched(Enrichment::new("email_type_name", "email_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&email_type), None);
        repo.add_enriched(Enrichment::new("description", "email_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), Arc::clone(&email_type), None);
        Ok(repo)
}
