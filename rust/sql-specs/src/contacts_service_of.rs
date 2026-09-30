use crate::contact_columns::{columns, contact_entity, references, snake};
use datasources::converters::{MetaDataRowTypeConverter, RowTypeConverter};
use datasources::{
    as_readonly, CRUDRepository, CRUDRepositoryOptions, CommandMiddleware, Connection,
    ICRUDRepository, TypeConverter, Value,
};
use std::collections::HashMap;
use std::sync::Arc;

fn name_list(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn stamp_fields(
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    [
        ("Uuid", "uuid"),
        ("Created", "datetime"),
        ("Updated", "datetime"),
        ("Version", "binary"),
    ]
    .into_iter()
    .filter_map(|(column, key)| field(key).map(|converter| (column.to_string(), converter)))
    .collect()
}

fn open(
    connection: Arc<dyn Connection>,
    entity: &str,
    names: &[&str],
    contact: bool,
    middleware: Option<Arc<dyn CommandMiddleware>>,
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = vec!["Id".into()];
    options.entity_mapper = if contact {
        Box::new(contact_entity())
    } else {
        Box::new(snake())
    };
    options.column_mappings = Box::new(columns());
    options.row_converter =
        Some(RowTypeConverter::new(name_list(names)).with_fields(stamp_fields(field)));
    options.object_converter = Some(Box::new(
        MetaDataRowTypeConverter::new(Vec::new()).with_references(references(), Box::new(columns())),
    ));
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
    Arc::new(open(connection, entity, names, false, middleware, field))
}

pub fn contacts_service_of(
    connection: Arc<dyn Connection>,
    middleware: Option<Arc<dyn CommandMiddleware>>,
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> CRUDRepository {
    let base = [
        "Id", "Uuid", "Created", "Updated", "Version", "ContactSource", "ContactType", "FirstName",
        "LastName", "Email", "Notes",
    ];
    let named = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"];
    let address = [
        "Id", "Uuid", "Created", "Updated", "Version", "Contact", "AddressType", "Line1", "Line2",
        "City", "Region", "PostalCode", "Country",
    ];
    let phone = [
        "Id", "Uuid", "Created", "Updated", "Version", "Contact", "PhoneType", "Number",
    ];
    let address_types = as_readonly(repo(connection.clone(), "AddressType", &named, middleware.clone(), field));
    let phone_types = as_readonly(repo(connection.clone(), "PhoneType", &named, middleware.clone(), field));
    let mut addresses_repo = open(connection.clone(), "Address", &address, false, middleware.clone(), field);
    addresses_repo.add_enriched(
        datasources::Enrichment::new("AddressType", "AddressType", "Id", None, false),
        address_types,
        None,
    );
    let mut phones_repo = open(connection.clone(), "Phone", &phone, false, middleware.clone(), field);
    phones_repo.add_enriched(
        datasources::Enrichment::new("PhoneType", "PhoneType", "Id", None, false),
        phone_types,
        None,
    );
    let mut contacts = open(connection.clone(), "Contact", &base, true, middleware.clone(), field);
    contacts
        .add_enriched(
            datasources::Enrichment::new("ContactType", "ContactType", "Id", None, false),
            as_readonly(repo(connection.clone(), "ContactType", &named, middleware.clone(), field)),
            None,
        )
        .add_enriched(
            datasources::Enrichment::new("ContactSource", "ContactSource", "Id", None, false),
            as_readonly(repo(connection.clone(), "ContactSource", &named, middleware.clone(), field)),
            None,
        )
        .add_enriched(
            datasources::Enrichment::new("Addresses", "Id", "Contact", None, true),
            Arc::new(addresses_repo),
            None,
        )
        .add_enriched(
            datasources::Enrichment::new("Phones", "Id", "Contact", None, true),
            Arc::new(phones_repo),
            None,
        );
    contacts
}
