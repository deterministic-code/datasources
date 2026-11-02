use datasources::{ Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result};
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
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
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
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
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
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
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
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["key".into()];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

    CRUDRepository::<LegacyContact>::new(connection, options)
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
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
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
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
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
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
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
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
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
        ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
    ]));
    options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        let repo = ReadonlyRepository::<LegacyContact>::new(connection, options)?;
        Ok(repo)
}
