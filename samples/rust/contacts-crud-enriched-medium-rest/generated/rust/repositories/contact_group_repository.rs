use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn contact_group_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactGroup".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactGroup".into(), "contact_groups".into()),
    ])));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
    let contact = super::contact_repository::contact_repository(Arc::clone(&connection))?;
    let contact_group_member = super::contact_group_member_repository::contact_group_member_repository(Arc::clone(&connection))?;
    let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;
    repo.add_linked(Enrichment::new("members", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("contact_group_id", "contact_id")), Arc::new(contact), Arc::new(contact_group_member) as Arc<dyn ICRUDRepository>, None);
    Ok(repo)
}
