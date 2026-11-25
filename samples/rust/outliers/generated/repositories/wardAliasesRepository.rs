use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::WardAlias;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct WardAliasesRepository {
    pub(crate) inner: Arc<CRUDRepository<WardAlias>>,
}

impl WardAliasesRepository {
    pub fn options() -> Result<CRUDRepositoryOptions> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "WardAlias".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("WardAlias".into(), "ward_aliases".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("parent_region_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("parent_ward_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "parent_region_id".into(), "parent_ward_id".into(), "name".into()];
    options.create_properties = Some(vec!["parent_region_id".into(), "parent_ward_id".into(), "name".into()]);
    options.update_properties = Some(vec!["parent_region_id".into(), "parent_ward_id".into(), "name".into()]);

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = Self::options()?;
        let repo = CRUDRepository::<WardAlias>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<WardAlias>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "WardAlias".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("WardAlias".into(), "ward_aliases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("parent_region_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("parent_ward_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "parent_region_id".into(), "parent_ward_id".into(), "name".into()];
        let repo = ReadonlyRepository::<WardAlias>::new(connection, options.clone())?;
        Ok(repo)
    }
}

impl Deref for WardAliasesRepository {
    type Target = CRUDRepository<WardAlias>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<WardAlias> for &WardAliasesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<WardAlias>>,
        Option<Arc<dyn ICRUDRepository<WardAlias>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<WardAlias>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<WardAlias>>),
        )
    }
}

impl IRepository for WardAliasesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<WardAlias> for WardAliasesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<WardAlias>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<WardAlias>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<WardAlias>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<WardAlias>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<WardAlias>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<WardAlias> for WardAliasesRepository {
    async fn create(&self, item: WardAlias) -> Result<WardAlias> {
        self.inner.create(item).await
    }

    async fn update(&self, item: WardAlias, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: WardAlias, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<WardAlias>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn ward_aliases_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let options = WardAliasesRepository::options()?;
    CRUDRepository::<datasources::Row>::new(connection, options)
}
