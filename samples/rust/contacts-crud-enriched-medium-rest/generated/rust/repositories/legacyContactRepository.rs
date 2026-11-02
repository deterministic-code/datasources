use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::LegacyContact;
use std::collections::HashMap;
use std::sync::Arc;

pub fn legacy_contact_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<LegacyContact>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "LegacyContact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("LegacyContact".into(), "legacy_contacts".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("imported_at".into(), "datetime".into()),
                ]));
            options.ids = vec![];
            options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
            options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options.update_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "LegacyContact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("LegacyContact".into(), "OldContactsTbl".into()),
            ])));
            options.property_mappings = Some(Arc::new(DictionaryMapper::new([
                ("key".into(), "CntID".into()),
                ("first_name".into(), "FirstNm".into()),
                ("last_name".into(), "LastNm".into()),
                ("email".into(), "EmailAddr".into()),
                ("imported_at".into(), "ImpDate".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("imported_at".into(), "datetime".into()),
                ]));
            options.ids = vec!["key".into()];
            options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
            options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "LegacyContact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("LegacyContact".into(), "OldContactsTbl".into()),
            ])));
            options.property_mappings = Some(Arc::new(DictionaryMapper::new([
                ("key".into(), "CntID".into()),
                ("first_name".into(), "FirstNm".into()),
                ("last_name".into(), "LastNm".into()),
                ("email".into(), "EmailAddr".into()),
                ("imported_at".into(), "ImpDate".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("imported_at".into(), "datetime".into()),
                ]));
            options.ids = vec!["key".into()];
            options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
            options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "LegacyContact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("LegacyContact".into(), "OldContactsTbl".into()),
            ])));
            options.property_mappings = Some(Arc::new(DictionaryMapper::new([
                ("key".into(), "CntID".into()),
                ("first_name".into(), "FirstNm".into()),
                ("last_name".into(), "LastNm".into()),
                ("email".into(), "EmailAddr".into()),
                ("imported_at".into(), "ImpDate".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("imported_at".into(), "datetime".into()),
                ]));
            options.ids = vec!["key".into()];
            options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
            options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::<LegacyContact>::new(connection, options).map(|repo| repo.with_create_zeros([
        ("key".into(), Value::from("")),
        ("first_name".into(), Value::from("")),
        ("last_name".into(), Value::from("")),
    ].into_iter().collect()))
}

pub fn legacy_contact_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "LegacyContact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("LegacyContact".into(), "legacy_contacts".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("imported_at".into(), "datetime".into()),
                ]));
            options.ids = vec![];
            options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
            options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options.update_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "LegacyContact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("LegacyContact".into(), "OldContactsTbl".into()),
            ])));
            options.property_mappings = Some(Arc::new(DictionaryMapper::new([
                ("key".into(), "CntID".into()),
                ("first_name".into(), "FirstNm".into()),
                ("last_name".into(), "LastNm".into()),
                ("email".into(), "EmailAddr".into()),
                ("imported_at".into(), "ImpDate".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("imported_at".into(), "datetime".into()),
                ]));
            options.ids = vec!["key".into()];
            options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
            options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "LegacyContact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("LegacyContact".into(), "OldContactsTbl".into()),
            ])));
            options.property_mappings = Some(Arc::new(DictionaryMapper::new([
                ("key".into(), "CntID".into()),
                ("first_name".into(), "FirstNm".into()),
                ("last_name".into(), "LastNm".into()),
                ("email".into(), "EmailAddr".into()),
                ("imported_at".into(), "ImpDate".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("imported_at".into(), "datetime".into()),
                ]));
            options.ids = vec!["key".into()];
            options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
            options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "LegacyContact".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("LegacyContact".into(), "OldContactsTbl".into()),
            ])));
            options.property_mappings = Some(Arc::new(DictionaryMapper::new([
                ("key".into(), "CntID".into()),
                ("first_name".into(), "FirstNm".into()),
                ("last_name".into(), "LastNm".into()),
                ("email".into(), "EmailAddr".into()),
                ("imported_at".into(), "ImpDate".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("imported_at".into(), "datetime".into()),
                ]));
            options.ids = vec!["key".into()];
            options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
            options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::<datasources::Row>::new(connection, options)
}

pub fn legacy_contact_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<LegacyContact>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "LegacyContact".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("LegacyContact".into(), "legacy_contacts".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
            ("imported_at".into(), "datetime".into()),
        ]));
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        let repo = ReadonlyRepository::<LegacyContact>::new(connection, options)?;
        Ok(repo)
}
