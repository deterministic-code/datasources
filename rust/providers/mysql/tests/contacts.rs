use super::command::MysqlCommand;
use super::connection::MysqlConnection;
use super::mysql_async_driver::MysqlAsyncDriver;
use datasources::Connection;
use datasources_sql_specs::{
    add_extra_contact, apply_sql, as_contacts_view, contact_schema, contacts_parity,
    contacts_service_of, expect_contacts_view, seed_contacts_sql, statements_of, SEED_CONTACT_COUNT,
};
use mysql_async::prelude::Queryable;
use mysql_async::Conn;
use std::sync::Arc;
use std::time::SystemTime;

fn verbose() -> bool {
    matches!(std::env::var("VERBOSE").as_deref(), Ok("1") | Ok("true"))
}

fn replace_db(url: &str, name: &str) -> String {
    match url.rfind('/') {
        Some(index) if url[..index].contains("://") => {
            let prefix = &url[..=index];
            match url[index + 1..].find('?') {
                Some(q) => format!("{prefix}{name}{}", &url[index + 1 + q..]),
                None => format!("{prefix}{name}"),
            }
        }
        _ => url.to_string(),
    }
}

#[tokio::test]
async fn matches_enriched_contacts_service_to_flat_mapper() {
    let Ok(base) = std::env::var("MYSQL_URL") else {
        return;
    };
    let name = format!(
        "ds_{:x}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    let mut admin = Conn::from_url(base.clone()).await.unwrap();
    admin
        .query_drop(format!("CREATE DATABASE `{name}`"))
        .await
        .unwrap();
    drop(admin);
    let url = replace_db(&base, &name);
    let conn = Arc::new(MysqlConnection::new(
        Some(url),
        Arc::new(MysqlAsyncDriver::new()),
    ));
    conn.open().await.unwrap();
    let command = MysqlCommand::new(conn.clone());
    let mut sql = statements_of(&contact_schema("mysql"));
    sql.extend(seed_contacts_sql(SEED_CONTACT_COUNT, "X''"));
    apply_sql(&command, &sql).await.unwrap();

    let (eager, enriched) = contacts_parity(conn.clone(), verbose()).await.unwrap();
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

    let mut admin = Conn::from_url(base).await.unwrap();
    admin
        .query_drop(format!("DROP DATABASE `{name}`"))
        .await
        .ok();
}
