use datasources::{Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result, async_trait, CommandMiddleware, IdKey, IWhere, ReadInput, Response, Stream};
use super::super::Phone;
use super::phoneTypesRepository::PhoneTypesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct PhonesRepository {
    pub(crate) inner: Arc<CRUDRepository<Phone>>,
    pub phone_type: PhoneTypesRepository,
}

impl PhonesRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Phone".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Phone".into(), "phone_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: false }),
            ("number".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("label".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("phone_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()];
        options.create_properties = Some(vec!["contact_id".into(), "number".into(), "label".into()]);
        options.update_properties = Some(vec!["uuid".into(), "contact_id".into(), "number".into(), "label".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Phone".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Phone".into(), "phone_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: false }),
            ("number".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("label".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("phone_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let phone_type = PhoneTypesRepository::new(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<Phone>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &phone_type, None);
        repo.add_enriched(Enrichment::new("description", "phone_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &phone_type, None);
        Ok(Self { inner: Arc::new(repo), phone_type })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Phone>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "Phone".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Phone".into(), "phone_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: false }),
            ("number".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("label".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("phone_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()];
        let phone_type = PhoneTypesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<Phone>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &phone_type, None);
        repo.add_enriched(Enrichment::new("description", "phone_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &phone_type, None);
        Ok(repo)
    }
}

impl Deref for PhonesRepository {
    type Target = CRUDRepository<Phone>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<Phone> for &PhonesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<Phone>>,
        Option<Arc<dyn ICRUDRepository<Phone>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<Phone>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<Phone>>),
        )
    }
}

impl IRepository for PhonesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }
}

#[async_trait]
impl IReadonlyRepository<Phone> for PhonesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<Phone>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: &dyn ReadInput,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Phone>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: &'a dyn ReadInput,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Phone>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }
}

#[async_trait]
impl ICRUDRepository<Phone> for PhonesRepository {
    async fn create(&self, item: Phone) -> Result<Phone> {
        self.inner.create(item).await
    }

    async fn update(&self, item: Phone, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: Phone, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Phone>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn phones_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Phone".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Phone".into(), "phone_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: false }),
            ("number".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("label".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("phone_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()];
        options.create_properties = Some(vec!["contact_id".into(), "number".into(), "label".into()]);
        options.update_properties = Some(vec!["uuid".into(), "contact_id".into(), "number".into(), "label".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Phone".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Phone".into(), "phone_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: false }),
            ("number".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("label".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("phone_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "contact_id".into(), "number".into(), "label".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let phone_type = Arc::new(super::phoneTypesRepository::phone_types_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&phone_type), None);
        repo.add_enriched(Enrichment::new("description", "phone_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), Arc::clone(&phone_type), None);
        Ok(repo)
}
