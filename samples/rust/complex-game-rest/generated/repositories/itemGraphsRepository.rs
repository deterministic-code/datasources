use datasources::{Connection, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, GraphReadonlyEnrichmentStrategy, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, Stream};
use super::super::ItemGraph;
use super::itemLinksRepository::ItemLinksRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct ItemGraphsRepository {
    pub(crate) inner: Arc<ReadonlyRepository<ItemGraph>>,
    pub item_links: ItemLinksRepository,
}

impl ItemGraphsRepository {
    pub fn options(connection: Arc<dyn Connection>) -> Result<ReadonlyRepositoryMappingOptions> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ItemGraph".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ItemGraph".into(), "items".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into()];
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ItemGraph".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ItemGraph".into(), "items".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into()];
        options
    })
    .for_connection(connection.as_ref())?
    .read;

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = Self::options(Arc::clone(&connection))?;
        let item_links = ItemLinksRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<ItemGraph>::new(Arc::clone(&connection), options)?;
        let via_edges = Arc::new(ReadonlyRepository::<ItemGraph>::new(Arc::clone(&connection), Self::options(Arc::clone(&connection))?)?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_item_id", "to_item_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&item_links))));
        Ok(Self { inner: Arc::new(repo), item_links })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ItemGraph>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "ItemGraph".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ItemGraph".into(), "items".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into()];
        let item_links = ItemLinksRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<ItemGraph>::new(Arc::clone(&connection), options)?;
        let via_edges = Arc::new(ReadonlyRepository::<ItemGraph>::new(Arc::clone(&connection), Self::options(Arc::clone(&connection))?)?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_item_id", "to_item_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&item_links))));
        Ok(repo)
    }
}

impl Deref for ItemGraphsRepository {
    type Target = ReadonlyRepository<ItemGraph>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<ItemGraph> for &ItemGraphsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<ItemGraph>>,
        Option<Arc<dyn ICRUDRepository<ItemGraph>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<ItemGraph>>,
            None,
        )
    }
}

impl IRepository for ItemGraphsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<ItemGraph> for ItemGraphsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<ItemGraph>>> {
        None
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<ItemGraph>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<ItemGraph>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<ItemGraph>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<ItemGraph>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}


pub fn item_graphs_row_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<datasources::Row>> {
        let options = ItemGraphsRepository::options(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List), Arc::clone(&item_graph), None);
        Ok(repo)
}
