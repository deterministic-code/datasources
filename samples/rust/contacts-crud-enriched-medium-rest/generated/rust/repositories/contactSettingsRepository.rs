use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::ContactSettings;
use std::collections::HashMap;
use std::sync::Arc;

pub fn contact_settings_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<ContactSettings>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSettings".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSettings".into(), "contact_settings".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSettings".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSettings".into(), "contact_settings".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSettings".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSettings".into(), "contact_settings".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
            options.create_properties = Some(vec!["contact_id".into(), "key".into(), "value".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSettings".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSettings".into(), "contact_settings".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
            options.create_properties = Some(vec!["contact_id".into(), "key".into(), "value".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::<ContactSettings>::new(connection, options).map(|repo| repo.with_create_zeros([
        ("key".into(), Value::from("")),
        ("value".into(), Value::from("")),
    ].into_iter().collect()))
}

pub fn contact_settings_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSettings".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSettings".into(), "contact_settings".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSettings".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSettings".into(), "contact_settings".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSettings".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSettings".into(), "contact_settings".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
            options.create_properties = Some(vec!["contact_id".into(), "key".into(), "value".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSettings".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSettings".into(), "contact_settings".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
            options.create_properties = Some(vec!["contact_id".into(), "key".into(), "value".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::<datasources::Row>::new(connection, options)
}

pub fn contact_settings_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactSettings>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactSettings".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactSettings".into(), "contact_settings".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
            ("id".into(), "integer".into()),
            ("uuid".into(), "uuid".into()),
            ("created".into(), "datetime".into()),
            ("updated".into(), "datetime".into()),
            ("version".into(), "binary".into()),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "value".into()];
        let repo = ReadonlyRepository::<ContactSettings>::new(connection, options)?;
        Ok(repo)
}
