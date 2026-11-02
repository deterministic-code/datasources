use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result};
use super::super::ContactChangeLog;
use std::collections::HashMap;
use std::sync::Arc;
pub fn contact_change_log_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<ContactChangeLog>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactChangeLog".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactChangeLog".into(), "contact_change_logs".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("occurred_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: false }),
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("changed_by".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("change_summary".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
    options.create_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);
    options.update_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);

    CRUDRepository::<ContactChangeLog>::new(connection, options)
}

pub fn contact_change_log_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactChangeLog".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactChangeLog".into(), "contact_change_logs".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("occurred_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: false }),
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("changed_by".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("change_summary".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
    options.create_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);
    options.update_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);

    CRUDRepository::<datasources::Row>::new(connection, options)
}

pub fn contact_change_log_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactChangeLog>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactChangeLog".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactChangeLog".into(), "contact_change_logs".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("occurred_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: false }),
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("changed_by".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("change_summary".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
    ]));
    options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
        let mut repo = ReadonlyRepository::<ContactChangeLog>::new(connection, options)?;
        Ok(repo)
}
