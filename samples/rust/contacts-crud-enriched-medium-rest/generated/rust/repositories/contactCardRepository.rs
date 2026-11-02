use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::ContactCard;
use std::collections::HashMap;
use std::sync::Arc;

pub fn contact_card_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<ContactCard>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactCard".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactCard".into(), "contact_cards".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
    
        ]));
    options.ids = vec![];
    options.read_properties = vec!["display_name".into()];
    options.create_properties = Some(vec!["display_name".into()]);
    options.update_properties = Some(vec!["display_name".into()]);
    CRUDRepository::<ContactCard>::new(connection, options).map(|repo| repo.with_create_zeros([
        ("display_name".into(), Value::from("")),
    ].into_iter().collect()))
}

pub fn contact_card_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactCard".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactCard".into(), "contact_cards".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
    
        ]));
    options.ids = vec![];
    options.read_properties = vec!["display_name".into()];
    options.create_properties = Some(vec!["display_name".into()]);
    options.update_properties = Some(vec!["display_name".into()]);
    CRUDRepository::<datasources::Row>::new(connection, options)
}

pub fn contact_card_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactCard>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "ContactCard".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactCard".into(), "contact_cards".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
    
        ]));
        options.read_properties = vec!["display_name".into()];
        let repo = ReadonlyRepository::<ContactCard>::new(connection, options)?;
        Ok(repo)
}
