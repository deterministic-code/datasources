use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;
use std::ops::Deref;

pub struct PhoneRepository {
    pub repo: Arc<CRUDRepository>,
    pub phone_type: Arc<CRUDRepository>,
}

impl PhoneRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Phone".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Phone".into(), "phones".into()),
        ])));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
        let phone_type = Arc::new(super::phone_type_repository::phone_type_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&phone_type), None);
        Ok(Self { repo: Arc::new(repo), phone_type })
    }
}

impl Deref for PhoneRepository {
    type Target = CRUDRepository;
    fn deref(&self) -> &Self::Target {
        self.repo.as_ref()
    }
}

pub fn phone_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    Arc::try_unwrap(PhoneRepository::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))
}

pub fn phone_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "Phone".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Phone".into(), "phones".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
        let phone_type = Arc::new(super::phone_type_repository::phone_type_repository(Arc::clone(&connection))?);
        let mut repo = ReadonlyRepository::new(connection, options)?;
        repo.add_enriched(Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&phone_type), None);
        Ok(repo)
}
