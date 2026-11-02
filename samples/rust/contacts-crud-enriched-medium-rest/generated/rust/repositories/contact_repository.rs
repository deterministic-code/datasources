use datasources::{ Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, IReadonlyRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result};
use super::super::Contact;
use super::super::ContactSettings;
use super::super::ContactPreferences;
use super::super::ContactSource;
use super::super::Address;
use super::super::Phone;
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Deref;

pub struct ContactRepository {
    pub repo: Arc<CRUDRepository<Contact>>,
    pub address: Arc<super::address_repository::AddressRepository>,
    pub phone: Arc<super::phone_repository::PhoneRepository>,
    pub contact_settings: Arc<CRUDRepository<ContactSettings>>,
    pub contact_preferences: Arc<CRUDRepository<ContactPreferences>>,
    pub contact_source: Arc<CRUDRepository<ContactSource>>,
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
        options.create_properties = Some(vec!["contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let address = Arc::new(super::address_repository::AddressRepository::new(Arc::clone(&connection))?);
        let phone = Arc::new(super::phone_repository::PhoneRepository::new(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contact_settings_repository::contact_settings_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contact_preferences_repository::contact_preferences_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contact_source_repository::contact_source_repository(Arc::clone(&connection))?);
        let repo = CRUDRepository::<Contact>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address.repo) as Arc<dyn IReadonlyRepository<Address>>, None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone.repo) as Arc<dyn IReadonlyRepository<Phone>>, None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings) as Arc<dyn IReadonlyRepository<ContactSettings>>, None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences) as Arc<dyn IReadonlyRepository<ContactPreferences>>, None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source) as Arc<dyn IReadonlyRepository<ContactSource>>, None);
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
        options.create_properties = Some(vec!["contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let address = Arc::new(super::address_repository::address_row_repository(Arc::clone(&connection))?);
        let phone = Arc::new(super::phone_repository::phone_row_repository(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contact_settings_repository::contact_settings_row_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contact_preferences_repository::contact_preferences_row_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contact_source_repository::contact_source_row_repository(Arc::clone(&connection))?);
        let repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address) as Arc<dyn IReadonlyRepository>, None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone) as Arc<dyn IReadonlyRepository>, None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings) as Arc<dyn IReadonlyRepository>, None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences) as Arc<dyn IReadonlyRepository>, None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source) as Arc<dyn IReadonlyRepository>, None);
        Ok(repo)
}

pub fn contact_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Contact>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "Contact".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Contact".into(), "contacts_bases".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
        ("contact_source_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ("notes".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
    ]));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_source_id".into(), "first_name".into(), "last_name".into(), "email".into(), "notes".into()];
        let address = Arc::new(super::address_repository::AddressRepository::new(Arc::clone(&connection))?);
        let phone = Arc::new(super::phone_repository::PhoneRepository::new(Arc::clone(&connection))?);
        let contact_settings = Arc::new(super::contact_settings_repository::contact_settings_repository(Arc::clone(&connection))?);
        let contact_preferences = Arc::new(super::contact_preferences_repository::contact_preferences_repository(Arc::clone(&connection))?);
        let contact_source = Arc::new(super::contact_source_repository::contact_source_repository(Arc::clone(&connection))?);
        let repo = ReadonlyRepository::<Contact>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&address.repo) as Arc<dyn IReadonlyRepository<Address>>, None);
        repo.add_enriched(Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List), Arc::clone(&phone.repo) as Arc<dyn IReadonlyRepository<Phone>>, None);
        repo.add_enriched(Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&contact_settings) as Arc<dyn IReadonlyRepository<ContactSettings>>, None);
        repo.add_enriched(Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary), Arc::clone(&contact_preferences) as Arc<dyn IReadonlyRepository<ContactPreferences>>, None);
        repo.add_enriched(Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&contact_source) as Arc<dyn IReadonlyRepository<ContactSource>>, None);
        Ok(repo)
}
