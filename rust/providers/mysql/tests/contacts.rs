use super::command::MysqlCommand;
use super::connection::MysqlConnection;
use super::isolate;
use datasources::Connection;
use datasources_tests::{
    add_extra_contact, apply_sql, as_contacts_view, contact_lookup_ids, contact_schema,
    contact_with, contacts_parity, contacts_service_of, expect_contacts_view, seed_contacts_sql,
    seed_identity_sql, SEED_CONTACT_COUNT,
};
use std::sync::Arc;

fn verbose() -> bool {
    matches!(std::env::var("VERBOSE").as_deref(), Ok("1") | Ok("true"))
}

fn register_sql() {
    datasources_tests::register_contact_dialect("mysql", Arc::new(crate::MysqlDriver));
}

#[tokio::test]
async fn matches_enriched_contacts_service_to_flat_mapper() {
    register_sql();
    let isolated = isolate::create().await;
    let conn = Arc::new(MysqlConnection::new(Some(isolated.url.clone())));
    conn.open().await.unwrap();
    let command = MysqlCommand::new(conn.clone());
    let mut sql = contact_schema("mysql");
    sql.extend(seed_contacts_sql(SEED_CONTACT_COUNT, "X''"));
    sql.extend(seed_identity_sql("mysql"));
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
        &contacts_service_of(conn.clone(), None, super::converters::field),
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

    conn.close().await.ok();
    isolated.drop().await;
}
