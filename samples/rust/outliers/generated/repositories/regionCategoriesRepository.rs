use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::RegionCategory;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct RegionCategoriesRepository {
    pub(crate) inner: Arc<CRUDRepository<RegionCategory>>,
}

impl RegionCategoriesRepository {
    pub fn options() -> Result<CRUDRepositoryOptions> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "RegionCategory".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("RegionCategory".into(), "region_categories".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("region_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("category_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("parent_region_id".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
        ("parent_category_id".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["region_id".into(), "category_id".into()];
    options.read_properties = vec!["region_id".into(), "category_id".into(), "parent_region_id".into(), "parent_category_id".into(), "name".into()];
    options.create_properties = Some(vec!["region_id".into(), "category_id".into(), "parent_region_id".into(), "parent_category_id".into(), "name".into()]);
    options.update_properties = Some(vec!["region_id".into(), "category_id".into(), "parent_region_id".into(), "parent_category_id".into(), "name".into()]);

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = Self::options()?;
        let repo = CRUDRepository::<RegionCategory>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<RegionCategory>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "RegionCategory".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("RegionCategory".into(), "region_categories".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("region_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("category_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("parent_region_id".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("parent_category_id".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["region_id".into(), "category_id".into(), "parent_region_id".into(), "parent_category_id".into(), "name".into()];
        let repo = ReadonlyRepository::<RegionCategory>::new(connection, options)?;
        Ok(repo)
    }
}

impl Deref for RegionCategoriesRepository {
    type Target = CRUDRepository<RegionCategory>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<RegionCategory> for &RegionCategoriesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<RegionCategory>>,
        Option<Arc<dyn ICRUDRepository<RegionCategory>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<RegionCategory>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<RegionCategory>>),
        )
    }
}

impl IRepository for RegionCategoriesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<RegionCategory> for RegionCategoriesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<RegionCategory>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<RegionCategory>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<RegionCategory>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<RegionCategory>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<RegionCategory>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<RegionCategory> for RegionCategoriesRepository {
    async fn create(&self, item: RegionCategory) -> Result<RegionCategory> {
        self.inner.create(item).await
    }

    async fn update(&self, item: RegionCategory, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: RegionCategory, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<RegionCategory>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn region_categories_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let options = RegionCategoriesRepository::options()?;
    CRUDRepository::<datasources::Row>::new(connection, options)
}
