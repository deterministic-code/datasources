use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;

pub fn contact_group_member_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "ContactGroupMember".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("ContactGroupMember".into(), "contact_group_members".into()),
            ])));
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
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
            options.create_properties = Some(vec!["contact_id".into(), "contact_group_id".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "contact_id".into(), "contact_group_id".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::new(connection, options)
}

pub fn contact_group_member_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactGroupMember".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactGroupMember".into(), "contact_group_members".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "contact_group_id".into()];
        let repo = ReadonlyRepository::new(connection, options)?;
        Ok(repo)
}
