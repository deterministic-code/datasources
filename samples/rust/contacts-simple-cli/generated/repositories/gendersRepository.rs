use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::Gender;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct GendersRepository {
    pub(crate) inner: Arc<CRUDRepository<Gender>>,
}

impl GendersRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Gender".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Gender".into(), "genders".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);

        let repo = CRUDRepository::<Gender>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Gender>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "Gender".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Gender".into(), "genders".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        let repo = ReadonlyRepository::<Gender>::new(connection, options)?;
        Ok(repo)
    }
}

impl Deref for GendersRepository {
    type Target = CRUDRepository<Gender>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<Gender> for &GendersRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<Gender>>,
        Option<Arc<dyn ICRUDRepository<Gender>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<Gender>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<Gender>>),
        )
    }
}

impl IRepository for GendersRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<Gender> for GendersRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<Gender>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Gender>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Gender>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Gender>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Gender>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<Gender> for GendersRepository {
    async fn create(&self, item: Gender) -> Result<Gender> {
        self.inner.create(item).await
    }

    async fn update(&self, item: Gender, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: Gender, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Gender>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn genders_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Gender".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Gender".into(), "genders".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);

    CRUDRepository::<datasources::Row>::new(connection, options)
}
