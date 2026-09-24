use crate::mappers::{FlatToNestedMapper, MappedRow, Mapper};
use crate::query::Query;
use crate::read_input::{ReadInput, StatementRead};
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::IReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::sync::Arc;

pub struct FlatToNestedReadonlyEnrichmentStrategy {
    source: Arc<dyn IReadonlyRepository>,
    mapper: FlatToNestedMapper,
    hydrate: Arc<dyn Fn(MappedRow) -> Row + Send + Sync>,
    command: Option<Arc<dyn Query>>,
}

impl FlatToNestedReadonlyEnrichmentStrategy {
    pub fn new(
        source: Arc<dyn IReadonlyRepository>,
        mapper: FlatToNestedMapper,
        hydrate: impl Fn(MappedRow) -> Row + Send + Sync + 'static,
        command: Option<Arc<dyn Query>>,
    ) -> Self {
        Self {
            source,
            mapper,
            hydrate: Arc::new(hydrate),
            command,
        }
    }
}

#[async_trait]
impl ReadonlyEnrichmentStrategy for FlatToNestedReadonlyEnrichmentStrategy {
    async fn read(
        &self,
        _inner: &dyn IReadonlyRepository,
        _tree: &RepositoryTree,
        query: Option<&dyn ReadInput>,
    ) -> Result<Vec<Row>> {
        let rows = match &self.command {
            Some(command) => {
                let input = StatementRead(command.as_ref());
                self.source.read(Some(&input)).await?
            }
            None => self.source.read(query).await?,
        };
        Ok(self
            .mapper
            .map(rows)
            .into_iter()
            .map(|row| (self.hydrate)(row))
            .collect())
    }

    async fn enrich(&self, _tree: &RepositoryTree, _items: &mut [Row]) -> Result<()> {
        Ok(())
    }

    async fn apply(&self, _node: &RepositoryTreeNode, _parents: &mut [Row]) -> Result<()> {
        Ok(())
    }
}
