use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use datasources::converters::{MetaDataRowTypeConverter, RowTypeConverter};
use datasources::{
    as_readonly, encode_rows, CRUDRepository, CRUDRepositoryOptions, Connection, FlatToNestedMapper,
    ICRUDRepository, IReadonlyRepository, Mapper, Operator, TypeConverter, Value, Where,
};
use datasources_sql_specs::{
    add_extra_contact, apply_schema, apply_sql, as_contacts_view, columns, contact_entity,
    contact_lookup_ids, contact_seed, contact_with, contacts_join_query, contacts_parity,
    contacts_service_of, contacts_view, expect_contacts_view, from_mapped_contacts, references,
    seed_contacts_sql, snake, statements_of, SEED_CONTACT_COUNT,
};
use std::sync::Arc;

fn verbose() -> bool {
    matches!(std::env::var("VERBOSE").as_deref(), Ok("1") | Ok("true"))
}

#[tokio::test]
async fn matches_enriched_contacts_service_to_flat_mapper() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    apply_schema(conn.as_ref(), "contacts").await.unwrap();
    let command = SqliteCommand::new(conn.clone());
    let mut sql = statements_of(&contact_seed("sqlite"));
    sql.extend(seed_contacts_sql(SEED_CONTACT_COUNT, "X''"));
    apply_sql(&command, &sql).await.unwrap();

    let (eager, enriched) = contacts_parity(conn.clone(), verbose(), super::converters::field)
        .await
        .unwrap();
    expect_contacts_view(&eager, SEED_CONTACT_COUNT);
    expect_contacts_view(&enriched, SEED_CONTACT_COUNT);
    let mapped = contact_with(&eager, 2, 1);
    assert!(!mapped.contact_source_name.is_empty());
    assert!(!mapped.contact_type_name.is_empty());
    assert_eq!(mapped.addresses.len(), 2);
    assert_eq!(as_contacts_view(&enriched), as_contacts_view(&eager));

    let ids = contact_lookup_ids(conn.clone()).await.unwrap();
    let extra = add_extra_contact(
        &contacts_service_of(conn, None, super::converters::field),
        &ids,
    )
    .await
    .unwrap();
    assert_eq!(extra["FirstName"], datasources::Value::from("Extra"));
    assert!(extra["Addresses"].wire_text().contains("Line1=9 Extra"));
    assert!(extra["Addresses"]
        .wire_text()
        .contains(&format!("Contact={}", extra["Id"].wire_text())));
    assert!(extra["Phones"].wire_text().contains("Number=555-extra"));
    assert!(extra["Addresses"].wire_text().contains("Name=Home")
        || extra["Addresses"].wire_text().contains("Name%3DHome"));
    assert!(extra["Phones"].wire_text().contains("Name=Mobile")
        || extra["Phones"].wire_text().contains("Name%3DMobile"));
}

#[tokio::test]
async fn maps_join_rows_to_nested_contacts() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    apply_schema(conn.as_ref(), "contacts").await.unwrap();
    let command = SqliteCommand::new(conn.clone());
    let mut sql = statements_of(&contact_seed("sqlite"));
    sql.extend(seed_contacts_sql(SEED_CONTACT_COUNT, "X''"));
    apply_sql(&command, &sql).await.unwrap();

    let mut cells = CRUDRepositoryOptions::default();
    cells.entity = "contacts_base".into();
    cells.row_converter = Some(RowTypeConverter::new(Vec::new()));
    let join_repo = CRUDRepository::new(conn, cells).expect("join repo");
    let contacts = from_mapped_contacts(&Mapper::map(
        &FlatToNestedMapper::new(contacts_view()),
        join_repo
            .read(Some(&contacts_join_query()), None)
            .await
            .unwrap(),
    ));
    let mapped = contact_with(&contacts, 2, 1);
    assert_eq!(mapped.addresses.len(), 2);
    assert!(!mapped.contact_source_name.is_empty());
}

fn contact_columns(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn contact_repo(
    connection: Arc<dyn Connection>,
    entity: &str,
    names: &[&str],
    contact: bool,
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
    let converters = [
        ("Uuid", "uuid"),
        ("Created", "datetime"),
        ("Updated", "datetime"),
        ("Version", "binary"),
    ]
    .into_iter()
    .filter_map(|(column, key)| field(key).map(|converter| (column.to_string(), converter)))
    .collect();
    options.row_converter = Some(RowTypeConverter::new(contact_columns(names)).with_fields(converters));
    options.object_converter = Some(Box::new(
        MetaDataRowTypeConverter::new(Vec::new()).with_references(references(), Box::new(columns())),
    ));
    CRUDRepository::new(connection, options).expect("contacts repo")
}

fn contact_row(pairs: &[(&str, Value)]) -> datasources::Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

#[tokio::test]
async fn writes_enriched_contact() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    apply_schema(conn.as_ref(), "contacts").await.unwrap();
    let command = SqliteCommand::new(conn.clone());
    let sql = statements_of(&contact_seed("sqlite"));
    apply_sql(&command, &sql).await.unwrap();

    let field = super::converters::field;
    let named = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"];
    let address = [
        "Id", "Uuid", "Created", "Updated", "Version", "Contact", "AddressType", "Line1", "Line2",
        "City", "Region", "PostalCode", "Country",
    ];
    let phone = [
        "Id", "Uuid", "Created", "Updated", "Version", "Contact", "PhoneType", "Number",
    ];
    let base = [
        "Id", "Uuid", "Created", "Updated", "Version", "ContactSource", "ContactType", "FirstName",
        "LastName", "Email", "Notes",
    ];
    let mut addresses_repo = contact_repo(conn.clone(), "Address", &address, false, field);
    addresses_repo.add_enriched(
        datasources::Enrichment::new("AddressType", "AddressType", "Id", None, false),
        as_readonly(Arc::new(contact_repo(conn.clone(), "AddressType", &named, false, field))),
        None,
    );
    let mut phones_repo = contact_repo(conn.clone(), "Phone", &phone, false, field);
    phones_repo.add_enriched(
        datasources::Enrichment::new("PhoneType", "PhoneType", "Id", None, false),
        as_readonly(Arc::new(contact_repo(conn.clone(), "PhoneType", &named, false, field))),
        None,
    );
    let mut contacts_repo = contact_repo(conn.clone(), "Contact", &base, true, field);
    contacts_repo
        .add_enriched(
            datasources::Enrichment::new("ContactType", "ContactType", "Id", None, false),
            as_readonly(Arc::new(contact_repo(conn.clone(), "ContactType", &named, false, field))),
            None,
        )
        .add_enriched(
            datasources::Enrichment::new("ContactSource", "ContactSource", "Id", None, false),
            as_readonly(Arc::new(contact_repo(conn.clone(), "ContactSource", &named, false, field))),
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

    let ids = contact_lookup_ids(conn).await.unwrap();
    let stamp = Value::DateTime(
        chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .expect("stamp")
            .with_timezone(&chrono::Utc),
    );
    let version = Value::Binary(vec![]);
    let mut created = contact_row(&[
        ("Uuid", Value::Uuid(uuid::Uuid::new_v4())),
        ("Created", stamp.clone()),
        ("Updated", stamp.clone()),
        ("Version", version.clone()),
        ("ContactSource", Value::Int(ids.contact_source_id)),
        ("ContactType", Value::Int(ids.contact_type_id)),
        ("FirstName", Value::from("Extra")),
        ("LastName", Value::from("Contact")),
        (
            "Addresses",
            encode_rows(&[contact_row(&[
                ("Uuid", Value::Uuid(uuid::Uuid::new_v4())),
                ("Created", stamp.clone()),
                ("Updated", stamp.clone()),
                ("Version", version.clone()),
                ("AddressType", Value::Int(ids.address_type_id)),
                ("Line1", Value::from("9 Extra")),
                ("City", Value::from("Oslo")),
            ])]),
        ),
        (
            "Phones",
            encode_rows(&[contact_row(&[
                ("Uuid", Value::Uuid(uuid::Uuid::new_v4())),
                ("Created", stamp.clone()),
                ("Updated", stamp),
                ("Version", version),
                ("PhoneType", Value::Int(ids.phone_type_id)),
                ("Number", Value::from("555-extra")),
            ])]),
        ),
    ]);
    let created = contacts_repo.create(&mut created).await.unwrap();
    let Value::Int(id) = created.get("Id").cloned().unwrap_or(Value::Null) else {
        panic!("expected insert id");
    };
    let loaded = contacts_repo
        .read_single(
            Some(&Where::new(Operator::equals("Id", Value::Int(id)))),
            None,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(loaded.get("FirstName"), Some(&Value::from("Extra")));
    assert!(loaded
        .get("ContactType")
        .unwrap()
        .wire_text()
        .contains("Name=Person")
        || loaded.get("ContactType").unwrap().wire_text().contains("Name%3DPerson"));
    assert!(loaded
        .get("ContactSource")
        .unwrap()
        .wire_text()
        .contains("Name=Web")
        || loaded.get("ContactSource").unwrap().wire_text().contains("Name%3DWeb"));
    assert!(loaded
        .get("Addresses")
        .unwrap()
        .wire_text()
        .contains("Line1=9 Extra"));
    assert!(loaded
        .get("Addresses")
        .unwrap()
        .wire_text()
        .contains("Name=Home")
        || loaded.get("Addresses").unwrap().wire_text().contains("Name%3DHome"));
    assert!(loaded
        .get("Phones")
        .unwrap()
        .wire_text()
        .contains("Number=555-extra"));
    assert!(loaded
        .get("Phones")
        .unwrap()
        .wire_text()
        .contains("Name=Mobile")
        || loaded.get("Phones").unwrap().wire_text().contains("Name%3DMobile"));
}
