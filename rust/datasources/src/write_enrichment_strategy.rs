use crate::entity::Entity;
use crate::repository_tree::RepositoryTree;
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::write_mode::WriteMode;
use async_trait::async_trait;
use std::collections::HashSet;

#[async_trait]
pub trait WriteEnrichmentStrategy<T: Entity = Row>: Send + Sync {
    fn use_transaction(&self) -> bool {
        false
    }

    async fn persist(
        &self,
        tree: &RepositoryTree<T>,
        items: &mut [T],
        mode: WriteMode,
    ) -> Result<()>;

    async fn persist_bulk(
        &self,
        tree: &RepositoryTree<T>,
        items: &mut [T],
        mode: WriteMode,
    ) -> Result<()> {
        self.persist(tree, items, mode).await
    }

    async fn resolve_lookups(
        &self,
        _tree: &RepositoryTree<T>,
        _items: &mut [T],
        _record: &mut Row,
        _schema: &Schema,
    ) -> Result<()> {
        Ok(())
    }

    fn mapped_sources(
        &self,
        _tree: &RepositoryTree<T>,
        _column: &dyn Fn(&str) -> String,
    ) -> Vec<(String, String)> {
        Vec::new()
    }

    async fn lookup(
        &self,
        tree: &RepositoryTree<T>,
        items: &mut [T],
        record: &mut Row,
        schema: &Schema,
        _columns: Vec<(String, String)>,
    ) -> Result<()> {
        self.resolve_lookups(tree, items, record, schema).await
    }

    fn sources(
        &self,
        tree: &RepositoryTree<T>,
        column: &dyn Fn(&str) -> String,
    ) -> HashSet<String> {
        self.mapped_sources(tree, column)
            .into_iter()
            .flat_map(|(source, mapped)| [source, mapped])
            .collect()
    }

    fn destination(&self, _tree: &RepositoryTree<T>, _name: &str) -> bool {
        false
    }
}
