use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn import_contacts_result_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "import_contacts_result".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("import_contacts_result".into(), "import_contacts_results".into()),
    ])));
    options.ids = vec![];
    options.read_properties = vec!["imported_count".into(), "skipped_count".into(), "failed_count".into()];
    CRUDRepository::new(connection, options)
}
