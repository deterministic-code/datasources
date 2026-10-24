use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::datasource_error::DatasourceError;
use crate::result::Result;
use std::collections::HashMap;

#[derive(Default)]
pub struct RepositoryBuilderOptionsFactory {
    fallback: Option<CRUDRepositoryOptions>,
    items: HashMap<String, CRUDRepositoryOptions>,
}

impl RepositoryBuilderOptionsFactory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, options: CRUDRepositoryOptions) -> &mut Self {
        self.fallback = Some(options);
        self
    }

    pub fn add_for(&mut self, dialect: impl Into<String>, options: CRUDRepositoryOptions) -> &mut Self {
        self.items.insert(dialect.into(), options);
        self
    }

    pub fn for_provider(&self, provider: &str) -> Result<CRUDRepositoryOptions> {
        self.items
            .get(provider)
            .cloned()
            .or_else(|| self.fallback.clone())
            .ok_or_else(|| DatasourceError::message("repository options required"))
    }
}
