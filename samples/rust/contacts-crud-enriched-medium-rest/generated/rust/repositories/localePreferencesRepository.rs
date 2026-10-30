use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::LocalePreferences;
use std::collections::HashMap;
use std::sync::Arc;

pub fn locale_preferences_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<LocalePreferences>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "LocalePreferences".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("LocalePreferences".into(), "locale_preferences".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
    
        ]));
    options.ids = vec![];
    options.read_properties = vec!["locale".into(), "timezone".into()];
    options.create_properties = Some(vec!["locale".into(), "timezone".into()]);
    options.update_properties = Some(vec!["locale".into(), "timezone".into()]);
    CRUDRepository::<LocalePreferences>::new(connection, options).map(|repo| repo.with_create_zeros([
        ("locale".into(), Value::from("")),
        ("timezone".into(), Value::from("")),
    ].into_iter().collect()))
}

pub fn locale_preferences_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "LocalePreferences".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("LocalePreferences".into(), "locale_preferences".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
    
        ]));
    options.ids = vec![];
    options.read_properties = vec!["locale".into(), "timezone".into()];
    options.create_properties = Some(vec!["locale".into(), "timezone".into()]);
    options.update_properties = Some(vec!["locale".into(), "timezone".into()]);
    CRUDRepository::<datasources::Row>::new(connection, options)
}

pub fn locale_preferences_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<LocalePreferences>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "LocalePreferences".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("LocalePreferences".into(), "locale_preferences".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
    
        ]));
        options.read_properties = vec!["locale".into(), "timezone".into()];
        let repo = ReadonlyRepository::<LocalePreferences>::new(connection, options)?;
        Ok(repo)
}
