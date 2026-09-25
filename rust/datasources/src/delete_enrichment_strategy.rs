use crate::delete_nested::DeleteNested;
use crate::enrichment::Enrichment;
use crate::repository::ICRUDRepository;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use crate::{Operator, Where};
use async_trait::async_trait;

#[async_trait]
pub trait DeleteEnrichmentStrategy: Send + Sync {
    async fn delete_owned(
        &self,
        node: &RepositoryTreeNode,
        enrichment: &Enrichment,
        parent: &Row,
        delete_nested: &dyn DeleteNested,
    ) -> Result<()>;

    async fn delete_linked(
        &self,
        node: &RepositoryTreeNode,
        enrichment: &Enrichment,
        parent: &Row,
    ) -> Result<()>;
}

pub(crate) async fn load(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &Row,
) -> Result<Vec<Row>> {
    let Some(key) = parent.get(&enrichment.from).cloned() else {
        return Ok(Vec::new());
    };
    writer
        .read(
            Some(&Where::new(Operator::equals(
                enrichment.by.clone(),
                key,
            ))),
            None,
        )
        .await
}
