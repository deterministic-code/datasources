use crate::repository::IReadonlyRepository;
use crate::stored_enrichment::StoredEnrichment;
use std::sync::{Arc, Mutex};

pub struct RepositoryTreeNode {
    pub value: Arc<dyn IReadonlyRepository>,
    pub property_name: String,
    pub(crate) enrichments: Arc<Mutex<Vec<StoredEnrichment>>>,
}

impl RepositoryTreeNode {
    pub(crate) fn new(
        value: Arc<dyn IReadonlyRepository>,
        property_name: impl Into<String>,
        enrichments: Arc<Mutex<Vec<StoredEnrichment>>>,
    ) -> Self {
        Self {
            value,
            property_name: property_name.into(),
            enrichments,
        }
    }
}
