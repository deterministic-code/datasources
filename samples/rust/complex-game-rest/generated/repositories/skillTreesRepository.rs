use datasources::{Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DefaultWriteEnrichmentStrategy, DictionaryMapper, Enrichment, EnrichmentPropertyType, TreeCreateEnrichmentStrategy, TreeReadonlyEnrichmentStrategy, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::SkillTree;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct SkillTreesRepository {
    pub(crate) inner: Arc<CRUDRepository<SkillTree>>,
}

impl SkillTreesRepository {
    pub fn options(connection: Arc<dyn Connection>) -> Result<CRUDRepositoryOptions> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "SkillTree".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("SkillTree".into(), "skills".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("parent_skill_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: true, has_default: false, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "parent_skill_id".into(), "name".into()];
        options.create_properties = Some(vec!["parent_skill_id".into(), "name".into()]);
        options.update_properties = Some(vec!["uuid".into(), "parent_skill_id".into(), "name".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "SkillTree".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("SkillTree".into(), "skills".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("parent_skill_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: true, has_default: false, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "parent_skill_id".into(), "name".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "parent_skill_id".into(), "name".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "parent_skill_id".into(), "name".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = Self::options(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<SkillTree>::new(Arc::clone(&connection), options)?.with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(None, None, Some(Arc::new(TreeCreateEnrichmentStrategy::new())), false)));
        let via_children = Arc::new(CRUDRepository::<SkillTree>::new(Arc::clone(&connection), Self::options(Arc::clone(&connection))?)?); repo.add_enriched(Enrichment::new("children", "id", "parent_skill_id", None, EnrichmentPropertyType::List), via_children, Some(Arc::new(TreeReadonlyEnrichmentStrategy)));
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<SkillTree>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "SkillTree".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("SkillTree".into(), "skills".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("parent_skill_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: true, has_default: false, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "parent_skill_id".into(), "name".into()];
        let mut repo = ReadonlyRepository::<SkillTree>::new(Arc::clone(&connection), options.clone())?;
        let via_children = Arc::new(ReadonlyRepository::<SkillTree>::new(Arc::clone(&connection), options.clone())?); repo.add_enriched(Enrichment::new("children", "id", "parent_skill_id", None, EnrichmentPropertyType::List), via_children, Some(Arc::new(TreeReadonlyEnrichmentStrategy)));
        Ok(repo)
    }
}

impl Deref for SkillTreesRepository {
    type Target = CRUDRepository<SkillTree>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<SkillTree> for &SkillTreesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<SkillTree>>,
        Option<Arc<dyn ICRUDRepository<SkillTree>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<SkillTree>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<SkillTree>>),
        )
    }
}

impl IRepository for SkillTreesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<SkillTree> for SkillTreesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<SkillTree>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<SkillTree>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<SkillTree>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<SkillTree>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<SkillTree>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<SkillTree> for SkillTreesRepository {
    async fn create(&self, item: SkillTree) -> Result<SkillTree> {
        self.inner.create(item).await
    }

    async fn update(&self, item: SkillTree, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: SkillTree, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<SkillTree>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn skill_trees_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let options = SkillTreesRepository::options(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?.with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(None, None, Some(Arc::new(TreeCreateEnrichmentStrategy::new())), false)));
        let via_children = Arc::new(CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), SkillTreesRepository::options(Arc::clone(&connection))?)?); repo.add_enriched(Enrichment::new("children", "id", "parent_skill_id", None, EnrichmentPropertyType::List), via_children, Some(Arc::new(TreeReadonlyEnrichmentStrategy)));
        Ok(repo)
}
