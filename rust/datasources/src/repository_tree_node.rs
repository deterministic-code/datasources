use crate::from_schema::FromSchema;
use crate::repository::IReadonlyRepository;
use crate::row::Row;
use crate::stored_enrichment::WriteEnrichment;
use std::sync::{Arc, Mutex};

pub struct RepositoryTreeNode<T: FromSchema = Row> {
    pub value: Arc<dyn IReadonlyRepository>,
    pub property_name: String,
    pub(crate) enrichments: Arc<Mutex<Vec<WriteEnrichment<T>>>>,
}

impl<T: FromSchema> RepositoryTreeNode<T> {
    pub(crate) fn new(
        value: Arc<dyn IReadonlyRepository>,
        property_name: impl Into<String>,
        enrichments: Arc<Mutex<Vec<WriteEnrichment<T>>>>,
    ) -> Self {
        Self {
            value,
            property_name: property_name.into(),
            enrichments,
        }
    }
}
