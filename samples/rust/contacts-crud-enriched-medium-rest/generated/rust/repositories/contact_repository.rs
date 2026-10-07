use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

pub struct ContactRepository {
    repo: CRUDRepository,
    pub address: Arc<CRUDRepository>,
    pub phone: Arc<CRUDRepository>,
    pub contact_source: Arc<CRUDRepository>,
}

impl ContactRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Contact".into(), "contacts".into()),
        ])));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
        let address = Arc::new(super::address_repository::address_repository(Arc::clone(&connection))?);
        let phone = Arc::new(super::phone_repository::phone_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contact_source_repository::contact_source_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address), None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone), None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source), None);
        Ok(Self { repo, address, phone, contact_source })
    }
}

impl Deref for ContactRepository {
    type Target = CRUDRepository;
    fn deref(&self) -> &Self::Target {
        &self.repo
    }
}

impl DerefMut for ContactRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.repo
    }
}

pub fn contact_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    Ok(ContactRepository::new(connection)?.repo)
}
