use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct CRUDRepositoryOptions {
    pub ids: Vec<String>,
    pub write: Option<Arc<dyn WriteEnrichmentStrategy>>,
    pub read: ReadonlyRepositoryMappingOptions,
}

impl Default for CRUDRepositoryOptions {
    fn default() -> Self {
        Self {
            ids: vec!["id".into()],
            write: None,
            read: ReadonlyRepositoryMappingOptions::default(),
        }
    }
}

impl Deref for CRUDRepositoryOptions {
    type Target = ReadonlyRepositoryMappingOptions;

    fn deref(&self) -> &Self::Target {
        &self.read
    }
}

impl DerefMut for CRUDRepositoryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.read
    }
}
