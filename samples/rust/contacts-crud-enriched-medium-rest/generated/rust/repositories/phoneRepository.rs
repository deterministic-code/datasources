use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::Phone;
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Deref;

pub struct PhoneRepository {
    pub repo: Arc<CRUDRepository<Phone>>,
    pub phone_type: Arc<CRUDRepository>,
}

impl PhoneRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = CRUDRepositoryBuilder::new()
            .add({
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Phone".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Phone".into(), "phone_bases".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
                options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
                options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
                options
            })
            .add_for("sqlite", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Phone".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Phone".into(), "phones".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
                options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
                options
            })
            .add_for("mysql", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Phone".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Phone".into(), "phones".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
                options.create_properties = Some(vec!["contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
                options
            })
            .add_for("postgres", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Phone".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Phone".into(), "phones".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
                options.create_properties = Some(vec!["contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
                options
            })
            .for_connection(connection.as_ref())?;
        let phone_type = Arc::new(super::phoneTypeRepository::phone_type_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<Phone>::new(Arc::clone(&connection), options).map(|repo| repo.with_create_zeros([
        ("number".into(), Value::from("")),
    ].into_iter().collect()))?;
        repo.add_enriched(Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&phone_type), None);
        Ok(Self { repo: Arc::new(repo), phone_type })
    }
}

impl Deref for PhoneRepository {
    type Target = CRUDRepository<Phone>;
    fn deref(&self) -> &Self::Target {
        self.repo.as_ref()
    }
}

pub fn phone_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<Phone>> {
    Arc::try_unwrap(PhoneRepository::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))
}

pub fn phone_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Phone".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Phone".into(), "phone_bases".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Phone".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Phone".into(), "phones".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Phone".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Phone".into(), "phones".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
            options.create_properties = Some(vec!["contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Phone".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Phone".into(), "phones".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
            options.create_properties = Some(vec!["contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
        let phone_type = Arc::new(super::phoneTypeRepository::phone_type_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&phone_type), None);
        Ok(repo)
}

pub fn phone_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Phone>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "Phone".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Phone".into(), "phone_bases".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
            ("id".into(), "integer".into()),
            ("uuid".into(), "uuid".into()),
            ("created".into(), "datetime".into()),
            ("updated".into(), "datetime".into()),
            ("version".into(), "binary".into()),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "phone_type_id".into(), "number".into(), "label".into()];
        let phone_type = Arc::new(super::phoneTypeRepository::phone_type_row_repository(Arc::clone(&connection))?);
        let mut repo = ReadonlyRepository::<Phone>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&phone_type), None);
        Ok(repo)
}
