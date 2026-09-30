use crate::contact_columns::{columns, snake};
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
    options.ids = vec!["Id".into()];
    options.entity_mapper = Box::new(snake());
    options.column_mappings = Box::new(columns());
    options.row_converter = Some(RowTypeConverter::new(vec!["Id".into(), "Name".into()]));
    let repo = CRUDRepository::new(connection, options)?;
    let row = repo
        .read_single(
            Some(&Where::new(Operator::equals("Name", Value::from(name)))),
            None,
        )
        .await?
        .ok_or_else(|| DatasourceError::message(format!("{entity} '{name}' missing")))?;
    match row.get("Id") {
        Some(&Value::Int(id)) => Ok(id),
        _ => Err(DatasourceError::message(format!(
            "{entity} '{name}' has no id"
        ))),
    }
}

pub async fn contact_lookup_ids(connection: Arc<dyn Connection>) -> Result<ContactLookups> {
    Ok(ContactLookups {
        contact_source_id: id_named(connection.clone(), "ContactSource", "Web").await?,
        contact_type_id: id_named(connection.clone(), "ContactType", "Person").await?,
        address_type_id: id_named(connection.clone(), "AddressType", "Home").await?,
        phone_type_id: id_named(connection, "PhoneType", "Mobile").await?,
    })
}
