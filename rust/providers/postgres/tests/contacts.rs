use super::command::PostgresCommand;
use super::connection::PostgresConnection;
use super::tokio_postgres_driver::TokioPostgresDriver;
use datasources::Connection;
use datasources_sql_specs::{
    add_extra_contact, apply_sql, as_contacts_view, contact_lookup_ids, contact_schema,
    contact_with, contacts_parity, contacts_service_of, expect_contacts_view, seed_contacts_sql,
    statements_of, SEED_CONTACT_COUNT,
};
use std::sync::Arc;
use std::time::SystemTime;
use tokio_postgres::NoTls;

fn verbose() -> bool {
    matches!(std::env::var("VERBOSE").as_deref(), Ok("1") | Ok("true"))
}

#[tokio::test]
async fn matches_enriched_contacts_service_to_flat_mapper() {
    let Ok(url) = std::env::var("POSTGRES_URL") else {
        return;
    };
    let schema = format!(
        "ds_{:x}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    let (client, connection) = tokio_postgres::connect(&url, NoTls).await.unwrap();
    tokio::spawn(async move {
        let _ = connection.await;
    });
    client
        .batch_execute(&format!("CREATE SCHEMA {schema}"))
        .await
        .unwrap();
    let conn = Arc::new(PostgresConnection::new(
        Some(url),
        Some(schema.clone()),
        Arc::new(TokioPostgresDriver::new(Some(schema.clone()))),
    ));
    conn.open().await.unwrap();
    let command = PostgresCommand::new(conn.clone());
    let mut sql = statements_of(&contact_schema("postgres"));
    sql.extend(seed_contacts_sql(SEED_CONTACT_COUNT, r#"'\x'"#));
    apply_sql(&command, &sql).await.unwrap();

    let (eager, enriched) = contacts_parity(conn.clone(), verbose()).await.unwrap();
    expect_contacts_view(&eager, SEED_CONTACT_COUNT);
    expect_contacts_view(&enriched, SEED_CONTACT_COUNT);
    let mapped = contact_with(&eager, 2, 1);
    assert!(!mapped.contact_source_name.is_empty());
    assert!(!mapped.contact_type_name.is_empty());
    assert_eq!(mapped.addresses.len(), 2);
    assert_eq!(as_contacts_view(&enriched), as_contacts_view(&eager));

    let ids = contact_lookup_ids(conn.clone()).await.unwrap();
    let extra = add_extra_contact(&contacts_service_of(conn, None), &ids)
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

    client
        .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
        .await
        .ok();
}
