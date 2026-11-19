use datasources::{Connection, DictionaryMapper, Enrichment, EnrichmentPropertyType, TreeReadonlyEnrichmentStrategy, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, Stream};
use super::super::CategoryTree;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct CategoryTreesRepository {
    pub(crate) inner: Arc<ReadonlyRepository<CategoryTree>>,
}

impl CategoryTreesRepository {
    pub fn options() -> Result<ReadonlyRepositoryMappingOptions> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "CategoryTree".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("CategoryTree".into(), "categories".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("parent_category_id".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.read_properties = vec!["id".into(), "parent_category_id".into(), "name".into()];

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = Self::options()?;
        let mut repo = ReadonlyRepository::<CategoryTree>::new(Arc::clone(&connection), options)?;
        let via_children = Arc::new(ReadonlyRepository::<CategoryTree>::new(Arc::clone(&connection), Self::options()?)?); repo.add_enriched(Enrichment::new("children", "id", "parent_category_id", None, EnrichmentPropertyType::List), via_children, Some(Arc::new(TreeReadonlyEnrichmentStrategy)));
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<CategoryTree>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "CategoryTree".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("CategoryTree".into(), "categories".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("parent_category_id".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "parent_category_id".into(), "name".into()];
        let mut repo = ReadonlyRepository::<CategoryTree>::new(Arc::clone(&connection), options)?;
        let via_children = Arc::new(ReadonlyRepository::<CategoryTree>::new(Arc::clone(&connection), Self::options()?)?); repo.add_enriched(Enrichment::new("children", "id", "parent_category_id", None, EnrichmentPropertyType::List), via_children, Some(Arc::new(TreeReadonlyEnrichmentStrategy)));
        Ok(repo)
    }
}

impl Deref for CategoryTreesRepository {
    type Target = ReadonlyRepository<CategoryTree>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<CategoryTree> for &CategoryTreesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<CategoryTree>>,
        Option<Arc<dyn ICRUDRepository<CategoryTree>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<CategoryTree>>,
            None,
        )
    }
}

impl IRepository for CategoryTreesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<CategoryTree> for CategoryTreesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<CategoryTree>>> {
        None
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<CategoryTree>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<CategoryTree>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<CategoryTree>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<CategoryTree>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}


pub fn category_trees_row_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<datasources::Row>> {
        let options = CategoryTreesRepository::options()?;
        let mut repo = ReadonlyRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("children", "id", "parent_category_id", None, EnrichmentPropertyType::List), Arc::clone(&category_tree), None);
        Ok(repo)
}
