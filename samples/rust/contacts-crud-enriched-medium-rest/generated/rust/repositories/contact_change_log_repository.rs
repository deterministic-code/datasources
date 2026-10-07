use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn contact_change_log_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactChangeLog".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactChangeLog".into(), "contact_change_logs".into()),
    ])));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
    CRUDRepository::new(connection, options)
}
