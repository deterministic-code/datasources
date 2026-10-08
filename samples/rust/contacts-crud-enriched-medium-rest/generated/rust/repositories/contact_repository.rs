use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;
use std::ops::Deref;

pub struct ContactRepository {
    pub repo: Arc<CRUDRepository>,
    pub address: Arc<super::address_repository::AddressRepository>,
    pub phone: Arc<super::phone_repository::PhoneRepository>,
    pub contact_settings: Arc<CRUDRepository>,
    pub contact_preferences: Arc<CRUDRepository>,
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
        let address = Arc::new(super::address_repository::AddressRepository::new(Arc::clone(&connection))?);
        let phone = Arc::new(super::phone_repository::PhoneRepository::new(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contact_settings_repository::contact_settings_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contact_preferences_repository::contact_preferences_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contact_source_repository::contact_source_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address.repo), None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone.repo), None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings), None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences), None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source), None);
        Ok(Self { repo: Arc::new(repo), address, phone, contact_settings, contact_preferences, contact_source })
    }
}

impl Deref for ContactRepository {
    type Target = CRUDRepository;
    fn deref(&self) -> &Self::Target {
        self.repo.as_ref()
    }
}

pub fn contact_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    Arc::try_unwrap(ContactRepository::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))
}

pub fn contact_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "Contact".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Contact".into(), "contacts".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
        let address = Arc::new(super::address_repository::AddressRepository::new(Arc::clone(&connection))?);
        let phone = Arc::new(super::phone_repository::PhoneRepository::new(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contact_settings_repository::contact_settings_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contact_preferences_repository::contact_preferences_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contact_source_repository::contact_source_repository(Arc::clone(&connection))?);
        let mut repo = ReadonlyRepository::new(connection, options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address.repo), None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone.repo), None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings), None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences), None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source), None);
        Ok(repo)
}
