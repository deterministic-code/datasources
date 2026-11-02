use crate::entity::Entity;
use crate::repository::IReadonlyRepository;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use std::sync::{Arc, Mutex};

pub struct RepositoryTreeNode<T: Entity = Row> {
    pub value: Arc<dyn IReadonlyRepository>,
    pub property_name: String,
    pub(crate) enrichments: Arc<Mutex<Vec<StoredEnrichment<T>>>>,
}

impl<T: Entity> RepositoryTreeNode<T> {
    pub(crate) fn new(
        value: Arc<dyn IReadonlyRepository>,
        property_name: impl Into<String>,
        enrichments: Arc<Mutex<Vec<StoredEnrichment<T>>>>,
    ) -> Self {
        Self {
            value,
            property_name: property_name.into(),
            enrichments,
        }
    }
}
