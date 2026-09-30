use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use datasources::converters::RowTypeConverter;
use datasources::{
    as_readonly, encode_rows, CRUDRepository, CRUDRepositoryOptions, Connection, FlatToNestedMapper,
    ICRUDRepository, IReadonlyRepository, Mapper, Operator, TypeConverter, Value, Where,
};
use datasources_sql_specs::{
    add_extra_contact, apply_sql, as_contacts_view, contact_lookup_ids, contact_schema,
    contact_with, contacts_join_query, contacts_parity, contacts_service_of, contacts_view,
    expect_contacts_view, from_mapped_contacts, seed_contacts_sql, statements_of,
    SEED_CONTACT_COUNT,
};
use std::sync::Arc;

fn verbose() -> bool {
    matches!(std::env::var("VERBOSE").as_deref(), Ok("1") | Ok("true"))
}

fn render(statement: &datasources::CreateTableStatement) -> String {
    crate::statement_factory::SqliteStatementFactory
        .transform(statement)
        .get_text()
}

#[tokio::test]
async fn matches_enriched_contacts_service_to_flat_mapper() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    let command = SqliteCommand::new(conn.clone());
    let mut sql = statements_of(&contact_schema("sqlite", render));
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
    assert_eq!(extra["first_name"], datasources::Value::from("Extra"));
    assert!(extra["addresses"].wire_text().contains("line1=9 Extra"));
    assert!(extra["addresses"]
        .wire_text()
        .contains(&format!("contact_id={}", extra["id"].wire_text())));
    assert!(extra["phones"].wire_text().contains("number=555-extra"));
    assert!(extra["addresses"]
        .wire_text()
        .contains("address_type_name=Home"));
    assert!(extra["phones"]
        .wire_text()
        .contains("phone_type_name=Mobile"));
}

#[tokio::test]
async fn maps_join_rows_to_nested_contacts() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    let command = SqliteCommand::new(conn.clone());
    let mut sql = statements_of(&contact_schema("sqlite", render));
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
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    let converters = [
        ("uuid", "uuid"),
        ("created", "datetime"),
        ("updated", "datetime"),
        ("version", "binary"),
    ]
    .into_iter()
    .filter_map(|(column, key)| field(key).map(|converter| (column.to_string(), converter)))
    .collect();
    options.row_converter = Some(RowTypeConverter::new(contact_columns(names)).with_fields(converters));
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
    let command = SqliteCommand::new(conn.clone());
    let sql = statements_of(&contact_schema("sqlite", render));
    apply_sql(&command, &sql).await.unwrap();

    let field = super::converters::field;
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
    let mut addresses_repo = contact_repo(conn.clone(), "address", &address, field);
    addresses_repo.add_enriched(
        datasources::Enrichment::new(
            "address_type_name",
            "address_type_id",
            "id",
            Some("name".into()),
            false,
        ),
        as_readonly(Arc::new(contact_repo(
            conn.clone(),
            "address_type",
            &named,
            field,
        ))),
        None,
    );
    let mut phones_repo = contact_repo(conn.clone(), "phone", &phone, field);
    phones_repo.add_enriched(
        datasources::Enrichment::new(
            "phone_type_name",
            "phone_type_id",
            "id",
            Some("name".into()),
            false,
        ),
        as_readonly(Arc::new(contact_repo(conn.clone(), "phone_type", &named, field))),
        None,
    );
    let mut contacts_repo = contact_repo(conn.clone(), "contacts_base", &base, field);
    contacts_repo
        .add_enriched(
            datasources::Enrichment::new(
                "contact_type_name",
                "contact_type_id",
                "id",
                Some("name".into()),
                false,
            ),
            as_readonly(Arc::new(contact_repo(
                conn.clone(),
                "contact_type",
                &named,
                field,
            ))),
            None,
        )
        .add_enriched(
            datasources::Enrichment::new(
                "contact_source_name",
                "contact_source_id",
                "id",
                Some("name".into()),
                false,
            ),
            as_readonly(Arc::new(contact_repo(
                conn.clone(),
                "contact_source",
                &named,
                field,
            ))),
            None,
        )
        .add_enriched(
            datasources::Enrichment::new("addresses", "id", "contact_id", None, true),
            Arc::new(addresses_repo),
            None,
        )
        .add_enriched(
            datasources::Enrichment::new("phones", "id", "contact_id", None, true),
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
        ("uuid", Value::Uuid(uuid::Uuid::new_v4())),
        ("created", stamp.clone()),
        ("updated", stamp.clone()),
        ("version", version.clone()),
        ("contact_source_id", Value::Int(ids.contact_source_id)),
        ("contact_type_id", Value::Int(ids.contact_type_id)),
        ("first_name", Value::from("Extra")),
        ("last_name", Value::from("Contact")),
        (
            "addresses",
            encode_rows(&[contact_row(&[
                ("uuid", Value::Uuid(uuid::Uuid::new_v4())),
                ("created", stamp.clone()),
                ("updated", stamp.clone()),
                ("version", version.clone()),
                ("address_type_id", Value::Int(ids.address_type_id)),
                ("line1", Value::from("9 Extra")),
                ("city", Value::from("Oslo")),
            ])]),
        ),
        (
            "phones",
            encode_rows(&[contact_row(&[
                ("uuid", Value::Uuid(uuid::Uuid::new_v4())),
                ("created", stamp.clone()),
                ("updated", stamp),
                ("version", version),
                ("phone_type_id", Value::Int(ids.phone_type_id)),
                ("number", Value::from("555-extra")),
            ])]),
        ),
    ]);
    let created = contacts_repo.create(&mut created).await.unwrap();
    let Value::Int(id) = created.get("id").cloned().unwrap_or(Value::Null) else {
        panic!("expected insert id");
    };
    let loaded = contacts_repo
        .read_single(
            Some(&Where::new(Operator::equals("id", Value::Int(id)))),
            None,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(loaded.get("first_name"), Some(&Value::from("Extra")));
    assert_eq!(loaded.get("contact_type_name"), Some(&Value::from("Person")));
    assert_eq!(loaded.get("contact_source_name"), Some(&Value::from("Web")));
    assert!(loaded
        .get("addresses")
        .unwrap()
        .wire_text()
        .contains("line1=9 Extra"));
    assert!(loaded
        .get("addresses")
        .unwrap()
        .wire_text()
        .contains("address_type_name=Home"));
    assert!(loaded
        .get("phones")
        .unwrap()
        .wire_text()
        .contains("number=555-extra"));
    assert!(loaded
        .get("phones")
        .unwrap()
        .wire_text()
        .contains("phone_type_name=Mobile"));
}
