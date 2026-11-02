use datasources::{ Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result};
use super::super::ContactGroupMember;
use std::collections::HashMap;
use std::sync::Arc;
pub fn contact_group_member_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<ContactGroupMember>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroupMember".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroupMember".into(), "contact_group_members".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroupMember".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroupMember".into(), "contact_group_members".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroupMember".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroupMember".into(), "contact_group_members".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        options.create_properties = Some(vec!["contact_id".into(), "contact_group_id".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroupMember".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroupMember".into(), "contact_group_members".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        options.create_properties = Some(vec!["contact_id".into(), "contact_group_id".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

    CRUDRepository::<ContactGroupMember>::new(connection, options)
}

pub fn contact_group_member_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroupMember".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroupMember".into(), "contact_group_members".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroupMember".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroupMember".into(), "contact_group_members".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroupMember".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroupMember".into(), "contact_group_members".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        options.create_properties = Some(vec!["contact_id".into(), "contact_group_id".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroupMember".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroupMember".into(), "contact_group_members".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        options.create_properties = Some(vec!["contact_id".into(), "contact_group_id".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

    CRUDRepository::<datasources::Row>::new(connection, options)
}

pub fn contact_group_member_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactGroupMember>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactGroupMember".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactGroupMember".into(), "contact_group_members".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("contact_group_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
    ]));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        let repo = ReadonlyRepository::<ContactGroupMember>::new(connection, options)?;
        Ok(repo)
}
