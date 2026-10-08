use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;

pub fn contact_source_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactSource".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactSource".into(), "contact_sources".into()),
    ])));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
    CRUDRepository::new(connection, options)
}

pub fn contact_source_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactSource".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactSource".into(), "contact_sources".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        let repo = ReadonlyRepository::new(connection, options)?;
        Ok(repo)
}
