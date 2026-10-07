use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn legacy_migration_result_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "legacy_migration_result".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("legacy_migration_result".into(), "legacy_migration_results".into()),
    ])));
    options.ids = vec![];
    options.read_properties = vec!["migrated_count".into(), "skipped_count".into(), "completed_at".into()];
    CRUDRepository::new(connection, options)
}
