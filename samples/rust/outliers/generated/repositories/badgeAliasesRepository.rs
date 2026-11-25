use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::BadgeAlias;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct BadgeAliasesRepository {
    pub(crate) inner: Arc<CRUDRepository<BadgeAlias>>,
}

impl BadgeAliasesRepository {
    pub fn options() -> Result<CRUDRepositoryOptions> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "BadgeAlias".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("BadgeAlias".into(), "badge_aliases".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("parent_badge_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: false, is_id: false }),
        ("parent_code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "parent_badge_id".into(), "parent_code".into(), "name".into()];
    options.create_properties = Some(vec!["parent_badge_id".into(), "parent_code".into(), "name".into()]);
    options.update_properties = Some(vec!["parent_badge_id".into(), "parent_code".into(), "name".into()]);

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = Self::options()?;
        let repo = CRUDRepository::<BadgeAlias>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<BadgeAlias>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "BadgeAlias".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("BadgeAlias".into(), "badge_aliases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("parent_badge_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: false, is_id: false }),
            ("parent_code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "parent_badge_id".into(), "parent_code".into(), "name".into()];
        let repo = ReadonlyRepository::<BadgeAlias>::new(connection, options.clone())?;
        Ok(repo)
    }
}

impl Deref for BadgeAliasesRepository {
    type Target = CRUDRepository<BadgeAlias>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<BadgeAlias> for &BadgeAliasesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<BadgeAlias>>,
        Option<Arc<dyn ICRUDRepository<BadgeAlias>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<BadgeAlias>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<BadgeAlias>>),
        )
    }
}

impl IRepository for BadgeAliasesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<BadgeAlias> for BadgeAliasesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<BadgeAlias>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<BadgeAlias>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<BadgeAlias>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<BadgeAlias>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<BadgeAlias>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<BadgeAlias> for BadgeAliasesRepository {
    async fn create(&self, item: BadgeAlias) -> Result<BadgeAlias> {
        self.inner.create(item).await
    }

    async fn update(&self, item: BadgeAlias, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: BadgeAlias, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<BadgeAlias>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn badge_aliases_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let options = BadgeAliasesRepository::options()?;
    CRUDRepository::<datasources::Row>::new(connection, options)
}
