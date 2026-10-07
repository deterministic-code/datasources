use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn legacy_contact_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "LegacyContact".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("LegacyContact".into(), "OldContactsTbl".into()),
    ])));
    options.property_mappings = Some(Arc::new(DictionaryMapper::new([
        ("key".into(), "CntID".into()),
        ("first_name".into(), "FirstNm".into()),
        ("last_name".into(), "LastNm".into()),
        ("email".into(), "EmailAddr".into()),
        ("imported_at".into(), "ImpDate".into()),
    ])));
    options.ids = vec!["key".into()];
    options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
    CRUDRepository::new(connection, options)
}
