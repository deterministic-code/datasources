use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::ContactPreferences;
use std::collections::HashMap;
use std::sync::Arc;

pub fn contact_preferences_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<ContactPreferences>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactPreferences".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactPreferences".into(), "contact_preferences".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactPreferences".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactPreferences".into(), "contact_preferences".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactPreferences".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactPreferences".into(), "contact_preferences".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
            options.create_properties = Some(vec!["contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactPreferences".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactPreferences".into(), "contact_preferences".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
            options.create_properties = Some(vec!["contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::<ContactPreferences>::new(connection, options).map(|repo| repo.with_create_zeros([
        ("key".into(), Value::from("")),
        ("locale".into(), Value::from("")),
        ("timezone".into(), Value::from("")),
    ].into_iter().collect()))
}

pub fn contact_preferences_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactPreferences".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactPreferences".into(), "contact_preferences".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactPreferences".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactPreferences".into(), "contact_preferences".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactPreferences".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactPreferences".into(), "contact_preferences".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
            options.create_properties = Some(vec!["contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactPreferences".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactPreferences".into(), "contact_preferences".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
            options.create_properties = Some(vec!["contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::<datasources::Row>::new(connection, options)
}

pub fn contact_preferences_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactPreferences>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactPreferences".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactPreferences".into(), "contact_preferences".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
            ("id".into(), "integer".into()),
            ("uuid".into(), "uuid".into()),
            ("created".into(), "datetime".into()),
            ("updated".into(), "datetime".into()),
            ("version".into(), "binary".into()),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "key".into(), "locale".into(), "timezone".into()];
        let repo = ReadonlyRepository::<ContactPreferences>::new(connection, options)?;
        Ok(repo)
}
