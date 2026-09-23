use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enriched::Enriched;
use crate::i_repository::IRepository;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use crate::statement::Statement;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct EnrichedReadonlyRepository {
    inner: Arc<dyn IReadonlyRepository>,
    pub tree: RepositoryTree,
    strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
}

impl EnrichedReadonlyRepository {
    pub fn new(inner: Arc<dyn IReadonlyRepository>) -> Self {
        Self::with_strategy(inner, Arc::new(DefaultReadonlyEnrichmentStrategy))
    }

    pub fn with_strategy(
        inner: Arc<dyn IReadonlyRepository>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> Self {
        Self {
            tree: RepositoryTree::new(RepositoryTreeNode::with_strategy(
                inner.clone(),
                "",
                strategy.clone(),
            )),
            inner,
            strategy,
        }
    }

    pub fn add_enriched(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn IReadonlyRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> &mut Self {
        self.tree.parent.include(via, into, from, by, select, many);
        self
    }

    pub fn add_enriched_with(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn IReadonlyRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl FnOnce(&mut RepositoryTreeNode),
    ) -> &mut Self {
        nest(self.tree.parent.include(via, into, from, by, select, many));
        self
    }

    pub fn add_enriched_repo(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn crate::repository::ICRUDRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> &mut Self {
        self.tree
            .parent
            .include_repo(via, into, from, by, select, many);
        self
    }

    pub fn add_enriched_repo_with(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn crate::repository::ICRUDRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl FnOnce(&mut RepositoryTreeNode),
    ) -> &mut Self {
        nest(
            self.tree
                .parent
                .include_repo(via, into, from, by, select, many),
        );
        self
    }
}

#[async_trait]
impl Enriched for EnrichedReadonlyRepository {
    async fn enrich(&self, items: &mut [Row]) -> Result<()> {
        self.strategy.enrich(&self.tree, items).await
    }
}

impl IRepository for EnrichedReadonlyRepository {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for EnrichedReadonlyRepository {
    async fn read_single(&self, query: Option<&dyn Statement>) -> Result<Option<Row>> {
        Ok(self.read(query).await?.into_iter().next())
    }

    async fn read(&self, query: Option<&dyn Statement>) -> Result<Vec<Row>> {
        self.strategy
            .read(self.inner.as_ref(), &self.tree, query)
            .await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        Ok(self.read_where(where_clause).await?.into_iter().next())
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.strategy
            .read_where(self.inner.as_ref(), &self.tree, where_clause)
            .await
    }
}
