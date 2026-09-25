use datasources::converters::RowTypeConverter;
use datasources::{
    as_readonly, CRUDRepository, CRUDRepositoryOptions, CommandMiddleware, Connection,
    ICRUDRepository, TypeConverter, Value,
};
use std::collections::HashMap;
use std::sync::Arc;

fn columns(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn stamp_fields(
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    [
        ("uuid", "uuid"),
        ("created", "datetime"),
        ("updated", "datetime"),
        ("version", "binary"),
    ]
    .into_iter()
    .filter_map(|(column, key)| field(key).map(|converter| (column.to_string(), converter)))
    .collect()
}

fn open(
    connection: Arc<dyn Connection>,
    entity: &str,
    names: &[&str],
    middleware: Option<Arc<dyn CommandMiddleware>>,
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.row_converter =
        Some(RowTypeConverter::new(columns(names)).with_fields(stamp_fields(field)));
    let mut repo = CRUDRepository::new(connection, options).expect("contacts repo");
    if let Some(middleware) = middleware {
        repo.add_middleware(middleware);
    }
    repo
}

fn repo(
    connection: Arc<dyn Connection>,
    entity: &str,
    names: &[&str],
    middleware: Option<Arc<dyn CommandMiddleware>>,
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> Arc<dyn ICRUDRepository> {
    Arc::new(open(connection, entity, names, middleware, field))
}

pub fn contacts_service_of(
    connection: Arc<dyn Connection>,
    middleware: Option<Arc<dyn CommandMiddleware>>,
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> CRUDRepository {
    let base = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "contact_source_id",
        "contact_type_id",
        "first_name",
        "last_name",
        "email",
        "notes",
    ];
    let named = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "name",
        "description",
    ];
    let address = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "contact_id",
        "address_type_id",
        "line1",
        "line2",
        "city",
        "region",
        "postal_code",
        "country",
    ];
    let phone = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "contact_id",
        "phone_type_id",
        "number",
    ];
    let address_types = as_readonly(repo(
        connection.clone(),
        "address_type",
        &named,
        middleware.clone(),
        field,
    ));
    let phone_types = as_readonly(repo(
        connection.clone(),
        "phone_type",
        &named,
        middleware.clone(),
        field,
    ));
    let mut addresses_repo = open(
        connection.clone(),
        "address",
        &address,
        middleware.clone(),
        field,
    );
    addresses_repo.add_enriched(datasources::Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), false), address_types, None);
    let mut phones_repo = open(connection.clone(), "phone", &phone, middleware.clone(), field);
    phones_repo.add_enriched(datasources::Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), false), phone_types, None);
    let mut contacts = open(
        connection.clone(),
        "contacts_base",
        &base,
        middleware.clone(),
        field,
    );
    contacts
        .add_enriched(datasources::Enrichment::new("contact_type_name", "contact_type_id", "id", Some("name".into()), false), as_readonly(repo(
                connection.clone(),
                "contact_type",
                &named,
                middleware.clone(),
                field,
            )), None)
        .add_enriched(datasources::Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), false), as_readonly(repo(
                connection.clone(),
                "contact_source",
                &named,
                middleware.clone(),
                field,
            )), None)
        .add_enriched(datasources::Enrichment::new("addresses", "id", "contact_id", None, true), Arc::new(addresses_repo), None)
        .add_enriched(datasources::Enrichment::new("phones", "id", "contact_id", None, true), Arc::new(phones_repo), None);
    contacts
}
