use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use datasources::converters::RowTypeConverter;
use datasources::{
    CRUDRepository, Connection, FlatToNestedMapper, IReadonlyRepository, Mapper, RepositoryOptions,
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

#[tokio::test]
async fn matches_enriched_contacts_service_to_flat_mapper() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    let command = SqliteCommand::new(conn.clone());
    let mut sql = statements_of(&contact_schema("sqlite"));
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
    let mut sql = statements_of(&contact_schema("sqlite"));
    sql.extend(seed_contacts_sql(SEED_CONTACT_COUNT, "X''"));
    apply_sql(&command, &sql).await.unwrap();

    let mut cells = RepositoryOptions::default();
    cells.entity = "contacts_base".into();
    cells.mapping = Some(RowTypeConverter::new(Vec::new()));
    let join_repo = CRUDRepository::new(conn, cells).expect("join repo");
    let contacts = from_mapped_contacts(&Mapper::map(
        &FlatToNestedMapper::new(contacts_view()),
        join_repo.read(Some(&contacts_join_query())).await.unwrap(),
    ));
    let mapped = contact_with(&contacts, 2, 1);
    assert_eq!(mapped.addresses.len(), 2);
    assert!(!mapped.contact_source_name.is_empty());
}
