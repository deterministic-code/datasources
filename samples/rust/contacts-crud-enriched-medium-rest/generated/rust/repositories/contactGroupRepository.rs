use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result};
use super::super::ContactGroup;
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Deref;

pub struct ContactGroupRepository {
    pub repo: Arc<CRUDRepository<ContactGroup>>,
    pub contact: Arc<CRUDRepository>,
    pub contact_group_member: Arc<CRUDRepository>,
}

impl ContactGroupRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroup".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroup".into(), "contact_groups_bases".into()),
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
        options.entity = "ContactGroup".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroup".into(), "contact_groups".into()),
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
        options.entity = "ContactGroup".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroup".into(), "contact_groups".into()),
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
        options.entity = "ContactGroup".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroup".into(), "contact_groups".into()),
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

        let contact = Arc::new(super::contactRepository::contact_row_repository(Arc::clone(&connection))?);
        let contact_group_member = Arc::new(super::contactGroupMemberRepository::contact_group_member_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<ContactGroup>::new(Arc::clone(&connection), options)?;
        repo.add_linked(Enrichment::new("members", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("contact_group_id", "contact_id")), Arc::clone(&contact), Arc::clone(&contact_group_member) as Arc<dyn ICRUDRepository>, None);
        Ok(Self { repo: Arc::new(repo), contact, contact_group_member })
    }
}

impl Deref for ContactGroupRepository {
    type Target = CRUDRepository<ContactGroup>;
    fn deref(&self) -> &Self::Target {
        self.repo.as_ref()
    }
}

pub fn contact_group_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<ContactGroup>> {
    Arc::try_unwrap(ContactGroupRepository::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))
}

pub fn contact_group_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "ContactGroup".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroup".into(), "contact_groups_bases".into()),
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
        options.entity = "ContactGroup".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroup".into(), "contact_groups".into()),
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
        options.entity = "ContactGroup".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroup".into(), "contact_groups".into()),
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
        options.entity = "ContactGroup".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactGroup".into(), "contact_groups".into()),
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

        let contact = Arc::new(super::contactRepository::contact_row_repository(Arc::clone(&connection))?);
        let contact_group_member = Arc::new(super::contactGroupMemberRepository::contact_group_member_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_linked(Enrichment::new("members", "id", "id", None, EnrichmentPropertyType::List).linked(Association::new("contact_group_id", "contact_id")), Arc::clone(&contact), Arc::clone(&contact_group_member) as Arc<dyn ICRUDRepository>, None);
        Ok(repo)
}

pub fn contact_group_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactGroup>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactGroup".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactGroup".into(), "contact_groups_bases".into()),
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
        let contact = Arc::new(super::contactRepository::contact_row_repository(Arc::clone(&connection))?);
        let mut repo = ReadonlyRepository::<ContactGroup>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("members", "id", "id", None, EnrichmentPropertyType::List), Arc::clone(&contact), None);
        Ok(repo)
}
