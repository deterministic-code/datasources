use crate::contact_lookups::ContactLookups;
use datasources::converters::RowTypeConverter;
use datasources::{
    Connection, DatasourceError, CRUDRepository, IReadonlyRepository, RepositoryBuilderOptions, Result,
    Operator, Value, WhereClause,
};
use std::sync::Arc;

async fn id_named(connection: Arc<dyn Connection>, entity: &str, name: &str) -> Result<i64> {
    let mut options = RepositoryBuilderOptions::with_factory(connection);
    options.entity = entity.into();
    options.mapping = Some(RowTypeConverter::new(vec!["id".into(), "name".into()]));
    let repo = CRUDRepository::new(options)?;
    let row = repo
        .read_single_where(&WhereClause::new(Operator::equals("name", Value::from(name))))
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
