use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn address_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "addresses_base".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("addresses_base".into(), "addresses".into()),
    ])));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
    CRUDRepository::new(connection, options)
}
