use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn import_contacts_request_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ImportContactsRequest".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ImportContactsRequest".into(), "import_contacts_requests".into()),
    ])));
    options.ids = vec![];
    options.read_properties = vec!["source".into(), "payload".into()];
    CRUDRepository::new(connection, options)
}
