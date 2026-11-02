use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use crate::stubs::StubRepository;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use crate::write_mode::WriteMode;
use std::sync::{Arc, Mutex};

pub(crate) async fn after_bulk_write(
    write: Option<&Arc<dyn WriteEnrichmentStrategy>>,
    enrichments: Option<&Arc<Mutex<Vec<StoredEnrichment>>>>,
    items: &mut [Row],
    mode: WriteMode,
) -> Result<()> {
    let (Some(write), Some(enrichments)) = (write, enrichments) else {
        return Ok(());
    };
    if enrichments.lock().expect("enrichments").is_empty() {
        return Ok(());
    }
    let tree = RepositoryTree::new(RepositoryTreeNode::new(
        Arc::new(StubRepository::new("bulk")),
        "",
        Arc::clone(enrichments),
    ));
    write.persist_bulk(&tree, items, mode).await
}
