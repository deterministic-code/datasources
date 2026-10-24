use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};
use std::sync::Arc;

pub fn phone_type_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "PhoneType".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("PhoneType".into(), "phone_types".into()),
            ])));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "PhoneType".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("PhoneType".into(), "phone_types".into()),
            ])));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "PhoneType".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("PhoneType".into(), "phone_types".into()),
            ])));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
            options.create_properties = Some(vec!["name".into(), "description".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "name".into(), "description".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
    CRUDRepository::new(connection, options)
}

pub fn phone_type_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "PhoneType".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("PhoneType".into(), "phone_types".into()),
    ])));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into()];
        let repo = ReadonlyRepository::new(connection, options)?;
        Ok(repo)
}
