use crate::contact_lookups::ContactLookups;
use datasources::converters::RowTypeConverter;
use datasources::{
    CRUDRepository, CRUDRepositoryOptions, Connection, DatasourceError, IReadonlyRepository,
    Operator, Result, Value, Where,
};
use std::sync::Arc;

async fn id_named(connection: Arc<dyn Connection>, entity: &str, name: &str) -> Result<i64> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.row_converter = Some(RowTypeConverter::new(vec!["id".into(), "name".into()]));
    let repo = CRUDRepository::new(connection, options)?;
    let row = repo
        .read_single(
            Some(&Where::new(Operator::equals("name", Value::from(name)))),
            None,
        )
        .await?
        .ok_or_else(|| DatasourceError::message(format!("{entity} '{name}' missing")))?;
    match row.get("id") {
        Some(&Value::Int(id)) => Ok(id),
        _ => Err(DatasourceError::message(format!(
            "{entity} '{name}' has no id"
        ))),
    }
}

pub async fn contact_lookup_ids(connection: Arc<dyn Connection>) -> Result<ContactLookups> {
    Ok(ContactLookups {
        contact_source_id: id_named(connection.clone(), "contact_source", "Web").await?,
        contact_type_id: id_named(connection.clone(), "contact_type", "Person").await?,
        address_type_id: id_named(connection.clone(), "address_type", "Home").await?,
        phone_type_id: id_named(connection, "phone_type", "Mobile").await?,
    })
}
