use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DefaultWriteEnrichmentStrategy, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, GraphCreateEnrichmentStrategy, GraphReadonlyEnrichmentStrategy, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::NodeGraph;
use super::nodeEdgesRepository::NodeEdgesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct NodeGraphsRepository {
    pub(crate) inner: Arc<CRUDRepository<NodeGraph>>,
    pub node_edge: NodeEdgesRepository,
}

impl NodeGraphsRepository {
    pub fn options() -> Result<CRUDRepositoryOptions> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "NodeGraph".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("NodeGraph".into(), "nodes".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("value".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "value".into()];
    options.create_properties = Some(vec!["value".into()]);
    options.update_properties = Some(vec!["value".into()]);

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let node_edge = NodeEdgesRepository::new(Arc::clone(&connection))?;
        let options = Self::options()?;
        let mut repo = CRUDRepository::<NodeGraph>::new(Arc::clone(&connection), options)?.with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(None, None, Some(Arc::new(GraphCreateEnrichmentStrategy::new(Arc::clone(&node_edge.inner) as Arc<dyn ICRUDRepository<_>>))), false)));
        let via_edges = Arc::new(CRUDRepository::<NodeGraph>::new(Arc::clone(&connection), Self::options()?)?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_node_id", "to_node_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&node_edge))));
        Ok(Self { inner: Arc::new(repo), node_edge })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<NodeGraph>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "NodeGraph".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("NodeGraph".into(), "nodes".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("value".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "value".into()];
        let node_edge = NodeEdgesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<NodeGraph>::new(Arc::clone(&connection), options)?;
        let via_edges = Arc::new(ReadonlyRepository::<NodeGraph>::new(Arc::clone(&connection), options.clone())?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_node_id", "to_node_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&node_edge))));
        Ok(repo)
    }
}

impl Deref for NodeGraphsRepository {
    type Target = CRUDRepository<NodeGraph>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<NodeGraph> for &NodeGraphsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<NodeGraph>>,
        Option<Arc<dyn ICRUDRepository<NodeGraph>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<NodeGraph>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<NodeGraph>>),
        )
    }
}

impl IRepository for NodeGraphsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<NodeGraph> for NodeGraphsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<NodeGraph>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<NodeGraph>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<NodeGraph>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<NodeGraph>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<NodeGraph>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<NodeGraph> for NodeGraphsRepository {
    async fn create(&self, item: NodeGraph) -> Result<NodeGraph> {
        self.inner.create(item).await
    }

    async fn update(&self, item: NodeGraph, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: NodeGraph, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<NodeGraph>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn node_graphs_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let node_edge = Arc::new(super::nodeEdgesRepository::node_edges_row_repository(Arc::clone(&connection))?);
        let options = NodeGraphsRepository::options()?;
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?.with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(None, None, Some(Arc::new(GraphCreateEnrichmentStrategy::new(Arc::clone(&node_edge) as Arc<dyn ICRUDRepository<_>>))), false)));
        let via_edges = Arc::new(CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), NodeGraphsRepository::options()?)?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_node_id", "to_node_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&node_edge))));
        Ok(repo)
}
