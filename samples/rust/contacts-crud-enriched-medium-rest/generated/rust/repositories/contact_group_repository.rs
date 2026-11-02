use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;
use std::ops::Deref;

pub struct ContactGroupRepository {
    pub repo: Arc<CRUDRepository>,
    pub contact: Arc<super::contact_repository::ContactRepository>,
    pub contact_group_member: Arc<CRUDRepository>,
}

impl ContactGroupRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = CRUDRepositoryBuilder::new()
            .add({
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "ContactGroup".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("ContactGroup".into(), "contact_groups".into()),
                ])));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
                options.create_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
                options
            })
            .add_for("mysql", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "ContactGroup".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("ContactGroup".into(), "contact_groups".into()),
                ])));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
                options.create_properties = Some(vec!["name".into(), "description".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
                options
            })
            .add_for("postgres", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "ContactGroup".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("ContactGroup".into(), "contact_groups".into()),
                ])));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
                options.create_properties = Some(vec!["name".into(), "description".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
                options
            })
            .for_connection(connection.as_ref())?;
        let contact = Arc::new(super::contact_repository::ContactRepository::new(Arc::clone(&connection))?);
        let contact_group_member = Arc::new(super::contact_group_member_repository::contact_group_member_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;
        repo.add_linked(Enrichment::new("members", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("contact_group_id", "contact_id")), Arc::clone(&contact.repo), Arc::clone(&contact_group_member) as Arc<dyn ICRUDRepository>, None);
        Ok(Self { repo: Arc::new(repo), contact, contact_group_member })
    }
}

impl Deref for ContactGroupRepository {
    type Target = CRUDRepository;
    fn deref(&self) -> &Self::Target {
        self.repo.as_ref()
    }
}

pub fn contact_group_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    Arc::try_unwrap(ContactGroupRepository::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))
}

pub fn contact_group_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactGroup".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactGroup".into(), "contact_groups".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        let contact = Arc::new(super::contact_repository::ContactRepository::new(Arc::clone(&connection))?);
        let mut repo = ReadonlyRepository::new(connection, options)?;
        repo.add_enriched(Enrichment::new("members", "id", "id", None, EnrichmentPropertyType::List), Arc::clone(&contact.repo), None);
        Ok(repo)
}
