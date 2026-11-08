use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::PhoneAttribute;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct PhoneAttributesRepository {
    pub(crate) inner: Arc<CRUDRepository<PhoneAttribute>>,
}

impl PhoneAttributesRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "PhoneAttribute".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("PhoneAttribute".into(), "phone_attributes".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
        ("phone_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("value".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "phone_id".into(), "key".into(), "value".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "phone_id".into(), "key".into(), "value".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "phone_id".into(), "key".into(), "value".into()]);

        let repo = CRUDRepository::<PhoneAttribute>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<PhoneAttribute>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "PhoneAttribute".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("PhoneAttribute".into(), "phone_attributes".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("phone_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("value".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "phone_id".into(), "key".into(), "value".into()];
        let repo = ReadonlyRepository::<PhoneAttribute>::new(connection, options)?;
        Ok(repo)
    }
}

impl Deref for PhoneAttributesRepository {
    type Target = CRUDRepository<PhoneAttribute>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<PhoneAttribute> for &PhoneAttributesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<PhoneAttribute>>,
        Option<Arc<dyn ICRUDRepository<PhoneAttribute>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<PhoneAttribute>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<PhoneAttribute>>),
        )
    }
}

impl IRepository for PhoneAttributesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }
}

#[async_trait]
impl IReadonlyRepository<PhoneAttribute> for PhoneAttributesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<PhoneAttribute>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<PhoneAttribute>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<PhoneAttribute>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<PhoneAttribute>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<PhoneAttribute>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<PhoneAttribute> for PhoneAttributesRepository {
    async fn create(&self, item: PhoneAttribute) -> Result<PhoneAttribute> {
        self.inner.create(item).await
    }

    async fn update(&self, item: PhoneAttribute, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: PhoneAttribute, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<PhoneAttribute>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn phone_attributes_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "PhoneAttribute".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("PhoneAttribute".into(), "phone_attributes".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
        ("phone_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("value".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "phone_id".into(), "key".into(), "value".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "phone_id".into(), "key".into(), "value".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "phone_id".into(), "key".into(), "value".into()]);

    CRUDRepository::<datasources::Row>::new(connection, options)
}
