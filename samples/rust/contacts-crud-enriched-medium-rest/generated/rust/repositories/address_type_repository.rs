use datasources::{ Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result};
use super::super::AddressType;
use std::collections::HashMap;
use std::sync::Arc;
pub fn address_type_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<AddressType>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "AddressType".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("AddressType".into(), "address_types".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "AddressType".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("AddressType".into(), "address_types".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        options.create_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "AddressType".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("AddressType".into(), "address_types".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        options.create_properties = Some(vec!["name".into(), "description".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "AddressType".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("AddressType".into(), "address_types".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        options.create_properties = Some(vec!["name".into(), "description".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

    CRUDRepository::<AddressType>::new(connection, options)
}

pub fn address_type_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "AddressType".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("AddressType".into(), "address_types".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "AddressType".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("AddressType".into(), "address_types".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        options.create_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "AddressType".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("AddressType".into(), "address_types".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        options.create_properties = Some(vec!["name".into(), "description".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "AddressType".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("AddressType".into(), "address_types".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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

pub fn address_type_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<AddressType>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "AddressType".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("AddressType".into(), "address_types".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
    ]));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        let repo = ReadonlyRepository::<AddressType>::new(connection, options)?;
        Ok(repo)
}
