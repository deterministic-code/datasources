use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn contact_group_member_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactGroupMember".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactGroupMember".into(), "contact_group_members".into()),
    ])));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
    CRUDRepository::new(connection, options)
}
