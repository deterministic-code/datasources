use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::Contact;
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Deref;

pub struct ContactRepository {
    pub repo: Arc<CRUDRepository<Contact>>,
    pub address: Arc<CRUDRepository>,
    pub phone: Arc<CRUDRepository>,
    pub contact_settings: Arc<CRUDRepository>,
    pub contact_preferences: Arc<CRUDRepository>,
    pub contact_source: Arc<CRUDRepository>,
}

impl ContactRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = CRUDRepositoryBuilder::new()
            .add({
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Contact".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Contact".into(), "contacts_bases".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
                options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
                options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
                options
            })
            .add_for("sqlite", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Contact".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Contact".into(), "contacts".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
                options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
                options
            })
            .add_for("mysql", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Contact".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Contact".into(), "contacts".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
                options.create_properties = Some(vec!["contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
                options
            })
            .add_for("postgres", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Contact".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Contact".into(), "contacts".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
                options.create_properties = Some(vec!["contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
                options
            })
            .for_connection(connection.as_ref())?;
        let address = Arc::new(super::addressRepository::address_row_repository(Arc::clone(&connection))?);
        let phone = Arc::new(super::phoneRepository::phone_row_repository(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contactSettingsRepository::contact_settings_row_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contactPreferencesRepository::contact_preferences_row_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contactSourceRepository::contact_source_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<Contact>::new(Arc::clone(&connection), options).map(|repo| repo.with_create_zeros([
        ("first_name".into(), Value::from("")),
        ("last_name".into(), Value::from("")),
    ].into_iter().collect()))?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address), None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone), None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings), None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences), None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source), None);
        Ok(Self { repo: Arc::new(repo), address, phone, contact_settings, contact_preferences, contact_source })
    }
}

impl Deref for ContactRepository {
    type Target = CRUDRepository<Contact>;
    fn deref(&self) -> &Self::Target {
        self.repo.as_ref()
    }
}

pub fn contact_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<Contact>> {
    Arc::try_unwrap(ContactRepository::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))
}

pub fn contact_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Contact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Contact".into(), "contacts_bases".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Contact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Contact".into(), "contacts".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Contact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Contact".into(), "contacts".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
            options.create_properties = Some(vec!["contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Contact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Contact".into(), "contacts".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
            options.create_properties = Some(vec!["contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
        let address = Arc::new(super::addressRepository::address_row_repository(Arc::clone(&connection))?);
        let phone = Arc::new(super::phoneRepository::phone_row_repository(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contactSettingsRepository::contact_settings_row_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contactPreferencesRepository::contact_preferences_row_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contactSourceRepository::contact_source_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address), None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone), None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings), None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences), None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source), None);
        Ok(repo)
}

pub fn contact_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Contact>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "Contact".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Contact".into(), "contacts_bases".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
            ("id".into(), "integer".into()),
            ("uuid".into(), "uuid".into()),
            ("created".into(), "datetime".into()),
            ("updated".into(), "datetime".into()),
            ("version".into(), "binary".into()),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
        let address = Arc::new(super::addressRepository::address_row_repository(Arc::clone(&connection))?);
        let phone = Arc::new(super::phoneRepository::phone_row_repository(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contactSettingsRepository::contact_settings_row_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contactPreferencesRepository::contact_preferences_row_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contactSourceRepository::contact_source_row_repository(Arc::clone(&connection))?);
        let mut repo = ReadonlyRepository::<Contact>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address), None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone), None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings), None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences), None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source), None);
        Ok(repo)
}
