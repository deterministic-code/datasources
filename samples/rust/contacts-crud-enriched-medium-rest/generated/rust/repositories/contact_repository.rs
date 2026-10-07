use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;

pub fn contact_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "contacts_base".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("contacts_base".into(), "contacts".into()),
    ])));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
    let address = super::address_repository::address_repository(Arc::clone(&connection))?;
    let phone = super::phone_repository::phone_repository(Arc::clone(&connection))?;
    let contact_source = super::contact_source_repository::contact_source_repository(Arc::clone(&connection))?;
    let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;
    repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::new(address), None);
    repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::new(phone), None);
    repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::new(contact_source), None);
    Ok(repo)
}
