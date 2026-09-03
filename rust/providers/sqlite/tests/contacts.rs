use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use super::rusqlite_driver::RusqliteDriver;
use datasources::Connection;
use datasources_sql_specs::{
    add_extra_contact, apply_sql, as_contacts_view, contact_schema, contacts_parity,
    contacts_service_of, expect_contacts_view, seed_contacts_sql, statements_of, SEED_CONTACT_COUNT,
};
use std::sync::Arc;

fn verbose() -> bool {
    matches!(std::env::var("VERBOSE").as_deref(), Ok("1") | Ok("true"))
}

#[tokio::test]
async fn matches_enriched_contacts_service_to_flat_mapper() {
    let conn = Arc::new(SqliteConnection::new(
        Some(":memory:".into()),
        Arc::new(RusqliteDriver::new()),
    ));
    conn.open().await.unwrap();
    let command = SqliteCommand::new(conn.clone());
    let mut sql = statements_of(&contact_schema("sqlite"));
    sql.extend(seed_contacts_sql(SEED_CONTACT_COUNT, "X''"));
    apply_sql(&command, &sql).await.unwrap();

    let (eager, enriched) = contacts_parity(conn, verbose()).await.unwrap();
    expect_contacts_view(&eager, SEED_CONTACT_COUNT);
    expect_contacts_view(&enriched, SEED_CONTACT_COUNT);
    let mapped = eager.iter().find(|contact| contact.id == 10).unwrap();
    assert!(!mapped.contact_source_name.is_empty());
    assert!(!mapped.contact_type_name.is_empty());
    assert_eq!(mapped.addresses.len(), 2);
    assert_eq!(as_contacts_view(&enriched), as_contacts_view(&eager));

    let extra = add_extra_contact(&contacts_service_of(conn, None))
        .await
        .unwrap();
    assert_eq!(extra["first_name"], datasources::Value::from("Extra"));
    assert!(extra["addresses"].wire_text().contains("line1=9 Extra"));
    assert!(extra["addresses"]
        .wire_text()
        .contains(&format!("contact_id={}", extra["id"].wire_text())));
    assert!(extra["phones"].wire_text().contains("number=555-extra"));
}
