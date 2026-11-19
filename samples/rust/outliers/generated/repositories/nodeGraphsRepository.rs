use datasources::{Connection, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, GraphReadonlyEnrichmentStrategy, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, Stream};
use super::super::NodeGraph;
use super::nodeEdgesRepository::NodeEdgesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct NodeGraphsRepository {
    pub(crate) inner: Arc<ReadonlyRepository<NodeGraph>>,
    pub node_edge: NodeEdgesRepository,
}

impl NodeGraphsRepository {
    pub fn options() -> Result<ReadonlyRepositoryMappingOptions> {
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

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = Self::options()?;
        let node_edge = NodeEdgesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<NodeGraph>::new(Arc::clone(&connection), options)?;
        let via_edges = Arc::new(ReadonlyRepository::<NodeGraph>::new(Arc::clone(&connection), Self::options()?)?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_node_id", "to_node_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&node_edge))));
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
        let via_edges = Arc::new(ReadonlyRepository::<NodeGraph>::new(Arc::clone(&connection), Self::options()?)?); repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("from_node_id", "to_node_id")), via_edges, Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&node_edge))));
        Ok(repo)
    }
}

impl Deref for NodeGraphsRepository {
    type Target = ReadonlyRepository<NodeGraph>;
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
            None,
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
        None
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


pub fn node_graphs_row_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<datasources::Row>> {
        let options = NodeGraphsRepository::options()?;
        let mut repo = ReadonlyRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("edges", "id", "id", None, EnrichmentPropertyType::List), Arc::clone(&node_graph), None);
        Ok(repo)
}
