use crate::mappers::{FlatToTypeMapper, FlatToTypeMapperTrait, MappedRow};
use crate::query::Query;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::ReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::sync::Arc;

pub struct FlatToTypeReadonlyEnrichmentStrategy {
    source: Arc<dyn ReadonlyRepository>,
    mapper: FlatToTypeMapper,
    hydrate: Arc<dyn Fn(MappedRow) -> Row + Send + Sync>,
    command: Option<Arc<dyn Query>>,
}

impl FlatToTypeReadonlyEnrichmentStrategy {
    pub fn new(
        source: Arc<dyn ReadonlyRepository>,
        mapper: FlatToTypeMapper,
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
impl ReadonlyEnrichmentStrategy for FlatToTypeReadonlyEnrichmentStrategy {
    async fn get(
        &self,
        _inner: &dyn ReadonlyRepository,
        _tree: &RepositoryTree,
        query: Option<&dyn Query>,
    ) -> Result<Vec<Row>> {
        let rows = match &self.command {
            Some(command) => self.source.get(Some(command.as_ref())).await?,
            None => self.source.get(query).await?,
        };
        Ok(self
            .mapper
            .collapse_rows(rows)
            .into_iter()
            .map(|row| (self.hydrate)(row))
            .collect())
    }

    async fn get_where(
        &self,
        _inner: &dyn ReadonlyRepository,
        _tree: &RepositoryTree,
        where_clause: &dyn Where,
    ) -> Result<Vec<Row>> {
        let rows = match &self.command {
            Some(command) => self.source.get(Some(command.as_ref())).await?,
            None => self.source.get_where(where_clause).await?,
        };
        Ok(self
            .mapper
            .collapse_rows(rows)
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
