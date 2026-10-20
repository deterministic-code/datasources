use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;

pub fn contact_settings_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactSettings".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactSettings".into(), "contact_settings".into()),
    ])));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
    CRUDRepository::new(connection, options)
}

pub fn contact_settings_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactSettings".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactSettings".into(), "contact_settings".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
        let repo = ReadonlyRepository::new(connection, options)?;
        Ok(repo)
}
