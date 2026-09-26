use crate::mappers::TransformMapper;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct CRUDRepositoryOptions {
    pub ids: Vec<String>,
    pub insert: Option<Arc<TransformMapper<String, String>>>,
    pub update: Option<Arc<TransformMapper<String, String>>>,
    pub delete: Option<Arc<TransformMapper<String, String>>>,
    pub return_: Option<Arc<TransformMapper<String, String>>>,
    pub write: Option<Arc<dyn WriteEnrichmentStrategy>>,
    pub read: ReadonlyRepositoryOptions,
}

impl Default for CRUDRepositoryOptions {
    fn default() -> Self {
        Self {
            ids: vec!["id".into()],
            insert: None,
            update: None,
            delete: None,
            return_: None,
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
