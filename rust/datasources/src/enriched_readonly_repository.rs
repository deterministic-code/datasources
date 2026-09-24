use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enriched::Enriched;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::nest::Nest;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
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
        via: impl EnrichmentVia,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl Nest,
    ) -> &mut Self {
        nest.apply(self.tree.parent.include(via, into, from, by, select, many));
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
    async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>> {
        Ok(self.read(query).await?.into_iter().next())
    }

    async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>> {
        self.strategy
            .read(self.inner.as_ref(), &self.tree, query)
            .await
    }
}
