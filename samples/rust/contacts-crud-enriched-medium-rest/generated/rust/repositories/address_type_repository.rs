use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;

pub fn address_type_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "AddressType".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("AddressType".into(), "address_types".into()),
    ])));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
    CRUDRepository::new(connection, options)
}

pub fn address_type_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "AddressType".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("AddressType".into(), "address_types".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        let repo = ReadonlyRepository::new(connection, options)?;
        Ok(repo)
}
