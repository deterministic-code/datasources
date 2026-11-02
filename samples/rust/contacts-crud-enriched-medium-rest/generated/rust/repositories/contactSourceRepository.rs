use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::ContactSource;
use std::collections::HashMap;
use std::sync::Arc;

pub fn contact_source_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<ContactSource>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSource".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSource".into(), "contact_sources".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSource".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSource".into(), "contact_sources".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSource".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSource".into(), "contact_sources".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSource".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSource".into(), "contact_sources".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::<ContactSource>::new(connection, options).map(|repo| repo.with_create_zeros([
        ("name".into(), Value::from("")),
    ].into_iter().collect()))
}

pub fn contact_source_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSource".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSource".into(), "contact_sources".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSource".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSource".into(), "contact_sources".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSource".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSource".into(), "contact_sources".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactSource".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactSource".into(), "contact_sources".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::<datasources::Row>::new(connection, options)
}

pub fn contact_source_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactSource>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactSource".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactSource".into(), "contact_sources".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
            ("id".into(), "integer".into()),
            ("uuid".into(), "uuid".into()),
            ("created".into(), "datetime".into()),
            ("updated".into(), "datetime".into()),
            ("version".into(), "binary".into()),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        let repo = ReadonlyRepository::<ContactSource>::new(connection, options)?;
        Ok(repo)
}
