use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::NodeEdge;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct NodeEdgesRepository {
    pub(crate) inner: Arc<CRUDRepository<NodeEdge>>,
}

impl NodeEdgesRepository {
    pub fn options() -> Result<CRUDRepositoryOptions> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "NodeEdge".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("NodeEdge".into(), "node_edges".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("from_node_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("to_node_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec![];
    options.read_properties = vec!["from_node_id".into(), "to_node_id".into()];
    options.create_properties = Some(vec!["from_node_id".into(), "to_node_id".into()]);
    options.update_properties = Some(vec!["from_node_id".into(), "to_node_id".into()]);

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = Self::options()?;
        let repo = CRUDRepository::<NodeEdge>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<NodeEdge>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "NodeEdge".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("NodeEdge".into(), "node_edges".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("from_node_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("to_node_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["from_node_id".into(), "to_node_id".into()];
        let repo = ReadonlyRepository::<NodeEdge>::new(connection, options.clone())?;
        Ok(repo)
    }
}

impl Deref for NodeEdgesRepository {
    type Target = CRUDRepository<NodeEdge>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<NodeEdge> for &NodeEdgesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<NodeEdge>>,
        Option<Arc<dyn ICRUDRepository<NodeEdge>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<NodeEdge>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<NodeEdge>>),
        )
    }
}

impl IRepository for NodeEdgesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<NodeEdge> for NodeEdgesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<NodeEdge>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<NodeEdge>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<NodeEdge>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<NodeEdge>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<NodeEdge>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<NodeEdge> for NodeEdgesRepository {
    async fn create(&self, item: NodeEdge) -> Result<NodeEdge> {
        self.inner.create(item).await
    }

    async fn update(&self, item: NodeEdge, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: NodeEdge, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<NodeEdge>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn node_edges_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let options = NodeEdgesRepository::options()?;
    CRUDRepository::<datasources::Row>::new(connection, options)
}
