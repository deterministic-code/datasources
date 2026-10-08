use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;
use std::ops::Deref;

pub struct AddressRepository {
    pub repo: Arc<CRUDRepository>,
    pub address_type: Arc<CRUDRepository>,
}

impl AddressRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "addresses".into()),
        ])));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
        let address_type = Arc::new(super::address_type_repository::address_type_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        Ok(Self { repo: Arc::new(repo), address_type })
    }
}

impl Deref for AddressRepository {
    type Target = CRUDRepository;
    fn deref(&self) -> &Self::Target {
        self.repo.as_ref()
    }
}

pub fn address_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    Arc::try_unwrap(AddressRepository::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))
}

pub fn address_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "Address".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Address".into(), "addresses".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
        let address_type = Arc::new(super::address_type_repository::address_type_repository(Arc::clone(&connection))?);
        let mut repo = ReadonlyRepository::new(connection, options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        Ok(repo)
}
