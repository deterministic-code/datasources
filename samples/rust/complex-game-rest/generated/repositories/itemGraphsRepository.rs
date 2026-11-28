use datasources::{Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DefaultWriteEnrichmentStrategy, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, GraphCreateEnrichmentStrategy, GraphReadonlyEnrichmentStrategy, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::ItemGraph;
use super::itemLinksRepository::ItemLinksRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct ItemGraphsRepository {
    pub(crate) inner: Arc<CRUDRepository<ItemGraph>>,
    pub item_links: ItemLinksRepository,
}

impl ItemGraphsRepository {
    pub fn options(connection: Arc<dyn Connection>) -> Result<CRUDRepositoryOptions> {
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
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into()];
        options.create_properties = Some(vec!["name".into()]);
        options.update_properties = Some(vec!["uuid".into(), "name".into()]);
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
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "name".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "name".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let item_links = ItemLinksRepository::new(Arc::clone(&connection))?;
        let options = Self::options(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<ItemGraph>::new(Arc::clone(&connection), options)?.with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(None, None, Some(Arc::new(GraphCreateEnrichmentStrategy::new(Arc::clone(&item_links.inner) as Arc<dyn ICRUDRepository<_>>))), false)));
        let via_edges = Arc::new(CRUDRepository::<ItemGraph>::new(Arc::clone(&connection), Self::options(Arc::clone(&connection))?)?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_item_id", "to_item_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&item_links))));
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
        let mut repo = ReadonlyRepository::<ItemGraph>::new(Arc::clone(&connection), options.clone())?;
        let via_edges = Arc::new(ReadonlyRepository::<ItemGraph>::new(Arc::clone(&connection), options.clone())?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_item_id", "to_item_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&item_links))));
        Ok(repo)
    }
}

impl Deref for ItemGraphsRepository {
    type Target = CRUDRepository<ItemGraph>;
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
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<ItemGraph>>),
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
        Some(self)
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

#[async_trait]
impl ICRUDRepository<ItemGraph> for ItemGraphsRepository {
    async fn create(&self, item: ItemGraph) -> Result<ItemGraph> {
        self.inner.create(item).await
    }

    async fn update(&self, item: ItemGraph, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: ItemGraph, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<ItemGraph>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn item_graphs_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let item_links = Arc::new(super::itemLinksRepository::item_links_row_repository(Arc::clone(&connection))?);
        let options = ItemGraphsRepository::options(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?.with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(None, None, Some(Arc::new(GraphCreateEnrichmentStrategy::new(Arc::clone(&item_links) as Arc<dyn ICRUDRepository<_>>))), false)));
        let via_edges = Arc::new(CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), ItemGraphsRepository::options(Arc::clone(&connection))?)?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_item_id", "to_item_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(Arc::clone(&item_links)))));
        Ok(repo)
}
