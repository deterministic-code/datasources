use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct CRUDRepositoryOptions {
    pub ids: Vec<String>,
    pub write: Option<Arc<dyn WriteEnrichmentStrategy>>,
    pub read: ReadonlyRepositoryOptions,
}

impl Default for CRUDRepositoryOptions {
    fn default() -> Self {
        Self {
            ids: vec!["id".into()],
            write: None,
            read: ReadonlyRepositoryOptions::default(),
        }
    }
}

impl Deref for CRUDRepositoryOptions {
    type Target = ReadonlyRepositoryOptions;

    fn deref(&self) -> &Self::Target {
        &self.read
    }
}

impl DerefMut for CRUDRepositoryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.read
    }
}
