use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct CRUDRepositoryOptions {
    pub ids: Vec<String>,
    pub create_properties: Option<Vec<String>>,
    pub update_properties: Option<Vec<String>>,
    pub write_enrichment: Option<Arc<dyn WriteEnrichmentStrategy>>,
    pub read: ReadonlyRepositoryMappingOptions,
}

impl Clone for CRUDRepositoryOptions {
    fn clone(&self) -> Self {
        Self {
            ids: self.ids.clone(),
            create_properties: self.create_properties.clone(),
            update_properties: self.update_properties.clone(),
            write_enrichment: self.write_enrichment.clone(),
            read: self.read.clone(),
        }
    }
}

impl Default for CRUDRepositoryOptions {
    fn default() -> Self {
        Self {
            ids: vec!["id".into()],
            create_properties: None,
            update_properties: None,
            write_enrichment: None,
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
