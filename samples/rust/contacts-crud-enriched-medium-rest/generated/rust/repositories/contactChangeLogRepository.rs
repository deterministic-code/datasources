use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
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
            ("id".into(), "integer".into()),
            ("occurred_at".into(), "datetime".into()),
            ("contact_id".into(), "integer".into()),
        ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
    options.create_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);
    options.update_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);
    CRUDRepository::<ContactChangeLog>::new(connection, options).map(|repo| repo.with_create_zeros([
        ("changed_by".into(), Value::from("")),
        ("change_summary".into(), Value::from("")),
    ].into_iter().collect()))
}

pub fn contact_change_log_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactChangeLog".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactChangeLog".into(), "contact_change_logs".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
            ("id".into(), "integer".into()),
            ("occurred_at".into(), "datetime".into()),
            ("contact_id".into(), "integer".into()),
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
            ("id".into(), "integer".into()),
            ("occurred_at".into(), "datetime".into()),
            ("contact_id".into(), "integer".into()),
        ]));
        options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
        let repo = ReadonlyRepository::<ContactChangeLog>::new(connection, options)?;
        Ok(repo)
}
