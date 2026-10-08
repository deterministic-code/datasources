use crate::api::Api;
use crate::clock::Clock;
use crate::migrate;
use crate::repos;
use datasources::{CommandMiddleware, ConsoleCommandMiddleware, Driver, ExpectConsoleMiddleware};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const STAMP: &str = "2026-09-22T00:00:00.000Z";

fn signature(line: &str) -> String {
    match line.rfind("] ") {
        Some(duration) if line.ends_with("ms") || line.ends_with("μs") => {
            line[..=duration].to_string()
        }
        _ => line.to_string(),
    }
}

async fn connection() -> std::sync::Arc<dyn datasources::Connection> {
    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), "sqlite:///:memory:".into());
    let connection = datasources_sqlite::SqliteDriver.connection(Some(&options));
    connection.open().await.unwrap();
    connection
}

async fn open(middleware: Option<Arc<dyn CommandMiddleware>>) -> Api {
    let connection = connection().await;
    migrate::apply(connection.create_command().as_ref(), connection.provider())
        .await
        .unwrap();
    let (contacts, legacy) = repos::open(connection, middleware).unwrap();
    Api::new(contacts, legacy, Clock::new())
}

#[tokio::test]
async fn round_trips_contacts_and_legacy_contacts() {
    let shared = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../contacts-crud-enriched-medium-rest-shared");
    let log_file = shared.join("sqlite.log");
    let record = std::env::var("RECORD_CONSOLE").ok().as_deref() == Some("1");
    let n = Arc::new(Mutex::new(0u32));
    let clock = Clock::new();
    clock.set_now(|| STAMP.into());
    {
        let n = Arc::clone(&n);
        clock.set_uuid(move || {
            let mut guard = n.lock().unwrap();
            *guard += 1;
            format!("00000000-0000-0000-0000-{:012}", *guard)
        });
    }
    let lines = Arc::new(Mutex::new(Vec::new()));
    let expect = if record {
        None
    } else {
        let expected = std::fs::read_to_string(&log_file)
            .unwrap()
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect();
        Some(Arc::new(ExpectConsoleMiddleware::new(expected, || STAMP.into())))
    };
    let middleware: Arc<dyn CommandMiddleware> = match &expect {
        Some(middleware) => Arc::clone(middleware) as Arc<dyn CommandMiddleware>,
        None => {
            let lines = Arc::clone(&lines);
            Arc::new(
                ConsoleCommandMiddleware::with_sink(
                    move |line| lines.lock().unwrap().push(signature(line)),
                    || STAMP.into(),
                )
                .options(HashMap::from([("isVerbose".into(), "true".into())])),
            )
        }
    };
    let connection = connection().await;
    migrate::apply(connection.create_command().as_ref(), connection.provider())
        .await
        .unwrap();
    let (contacts, legacy) = repos::open(connection, Some(middleware)).unwrap();
    let api = Api::new(contacts, legacy, clock);
    let created = api
        .handle(
            "POST",
            "/contacts/",
            Some(
                r#"{"first_name":"Ada","last_name":"Lovelace","email":"ada@example.com","contact_source_name":"Manual","addresses":[{"line1":"12 Analytical Engine","city":"London","address_type_name":"Home"}],"phones":[{"number":"555-0100","label":"Mobile","phone_type_name":"Mobile"}],"settings":{"theme":"dark"},"prefs":{"home":{"locale":"en-GB","timezone":"Europe/London"}}}"#,
            ),
        )
        .await
        .unwrap();
    assert_eq!(created.status, 201);
    assert!(!created.body.contains("OldContactsTbl"));
    let saved: Value = serde_json::from_str(&created.body).unwrap();
    assert_eq!(saved["first_name"], "Ada");
    assert_eq!(saved["contact_source_name"], "Manual");
    assert_eq!(saved["addresses"][0]["city"], "London");
    assert_eq!(saved["phones"][0]["number"], "555-0100");
    assert_eq!(saved["settings"]["theme"], "dark");
    assert_eq!(saved["prefs"]["home"]["locale"], "en-GB");
    assert_eq!(saved["prefs"]["home"]["timezone"], "Europe/London");
    api.handle(
        "POST",
        "/contacts",
        Some(
            r#"{"first_name":"Grace","last_name":"Hopper","uuid":"22222222-2222-2222-2222-222222222222","created":"2024-01-02T00:00:00Z","contact_source_name":"Imported","addresses":[{"uuid":"33333333-3333-3333-3333-333333333333","created":"2024-01-02T00:00:00Z","line1":"1","city":"NY","address_type_name":"Home"}],"phones":[{"uuid":"44444444-4444-4444-4444-444444444444","created":"2024-01-02T00:00:00Z","number":"1","phone_type_name":"Mobile"}]}"#,
        ),
    )
    .await
    .unwrap();
    let id = saved["id"].as_i64().unwrap();
    let filtered = api
        .handle(
            "GET",
            &format!("/contacts/?first_name=Ada&last_name=Lovelace&id={id}"),
            None,
        )
        .await
        .unwrap();
    assert_eq!(serde_json::from_str::<Vec<Value>>(&filtered.body).unwrap().len(), 1);
    let updated = api
        .handle(
            "PUT",
            &format!("/contacts/?id={id}"),
            Some(
                r#"{"first_name":"Ada","last_name":"Byron","email":null,"contact_source_name":"Manual","addresses":[{"line1":"1 Strand","city":"London","address_type_name":"Home"}],"phones":[],"settings":{"density":"compact"},"prefs":{"work":{"locale":"en-US","timezone":"America/New_York"}}}"#,
            ),
        )
        .await
        .unwrap();
    assert_eq!(updated.status, 200);
    let changed: Value = serde_json::from_str(&updated.body).unwrap();
    assert_eq!(changed["last_name"], "Byron");
    assert_eq!(changed["addresses"][0]["line1"], "1 Strand");
    assert_eq!(changed["phones"].as_array().unwrap().len(), 0);
    assert_eq!(changed["settings"]["density"], "compact");
    assert!(changed["settings"].get("theme").is_none());
    assert_eq!(changed["prefs"]["work"]["locale"], "en-US");
    assert!(changed["prefs"].get("home").is_none());
    assert_eq!(
        api.handle("DELETE", &format!("/contacts/?id={id}"), None)
            .await
            .unwrap()
            .status,
        204
    );
    assert_eq!(
        serde_json::from_str::<Vec<Value>>(&api.handle("get", "/contacts/", None).await.unwrap().body)
            .unwrap()
            .len(),
        1
    );
    let legacy = api
        .handle(
            "POST",
            "/legacy_contacts/",
            Some(
                r#"{"key":"ada","first_name":"Ada","last_name":"Lovelace","email":"ada@example.com","imported_at":"2024-06-01T00:00:00Z"}"#,
            ),
        )
        .await
        .unwrap();
    assert_eq!(legacy.status, 201);
    let listed: Vec<Value> = serde_json::from_str(
        &api.handle("GET", "/legacy_contacts/?first_name=Ada", None)
            .await
            .unwrap()
            .body,
    )
    .unwrap();
    assert_eq!(listed[0]["key"], "ada");
    let legacy_put = api
        .handle(
            "PUT",
            "/legacy_contacts/?key=ada",
            Some(r#"{"first_name":"Augusta","last_name":"Lovelace","email":""}"#),
        )
        .await
        .unwrap();
    assert_eq!(legacy_put.status, 200);
    assert!(legacy_put.body.contains("Augusta"));
    assert_eq!(
        api.handle("DELETE", "/legacy_contacts/?key=ada", None)
            .await
            .unwrap()
            .status,
        204
    );
    assert_eq!(
        serde_json::from_str::<Vec<Value>>(
            &api.handle("GET", "/legacy_contacts", None).await.unwrap().body
        )
        .unwrap()
        .len(),
        0
    );
    if record {
        std::fs::create_dir_all(&shared).unwrap();
        std::fs::write(&log_file, format!("{}\n", lines.lock().unwrap().join("\n"))).unwrap();
    } else {
        expect.unwrap().complete().unwrap();
    }
}

#[tokio::test]
async fn rejects_unknown_routes_lookups_and_missing_rows() {
    let api = open(None).await;
    assert_eq!(api.handle("GET", "/nope", None).await.unwrap().status, 404);
    assert_eq!(api.handle("POST", "/contacts/", Some(" ")).await.unwrap().status, 400);
    assert_eq!(api.handle("POST", "/contacts/", Some("null")).await.unwrap().status, 400);
    assert_eq!(api.handle("POST", "/contacts/", Some("{")).await.unwrap().status, 400);
    let unknown = api
        .handle(
            "POST",
            "/contacts/",
            Some(r#"{"first_name":"Ada","last_name":"Lovelace","contact_source_name":"Nope"}"#),
        )
        .await
        .unwrap();
    assert_eq!(unknown.status, 400);
    assert!(unknown.body.contains("Unknown contact_source"));
    assert_eq!(
        api.handle(
            "POST",
            "/contacts/",
            Some(r#"{"first_name":"Ada","last_name":"Lovelace"}"#)
        )
        .await
        .unwrap()
        .status,
        400
    );
    assert_eq!(
        api.handle("PUT", "/contacts/?id=abc", Some("{}")).await.unwrap().status,
        400
    );
    assert_eq!(
        api.handle(
            "PUT",
            "/contacts/?id=99",
            Some(r#"{"first_name":"Ada","last_name":"Lovelace","contact_source_name":"Manual"}"#)
        )
        .await
        .unwrap()
        .status,
        404
    );
    assert_eq!(api.handle("DELETE", "/contacts/?id=99", None).await.unwrap().status, 404);
    assert_eq!(api.handle("DELETE", "/contacts/", None).await.unwrap().status, 400);
    assert_eq!(api.handle("POST", "/legacy_contacts/", Some("{}")).await.unwrap().status, 400);
    assert_eq!(
        api.handle(
            "PUT",
            "/legacy_contacts/?key=missing",
            Some(r#"{"first_name":"A","last_name":"B"}"#)
        )
        .await
        .unwrap()
        .status,
        404
    );
    assert_eq!(
        api.handle("DELETE", "/legacy_contacts/?key=missing", None)
            .await
            .unwrap()
            .status,
        404
    );
    assert_eq!(
        api.handle("DELETE", "/legacy_contacts/?key=", None)
            .await
            .unwrap()
            .status,
        400
    );
    assert_eq!(
        api.handle("GET", "/contacts/?id=&lonely&first_name=Ada%20Lovelace", None)
            .await
            .unwrap()
            .status,
        200
    );
    assert_eq!(
        api.handle(
            "POST",
            "/contacts/",
            Some(r#"{"first_name":"Ada","last_name":"Lovelace","contact_source_name":"Manual"}"#)
        )
        .await
        .unwrap()
        .status,
        201
    );
}

#[tokio::test]
async fn maps_datasource_errors() {
    let connection = connection().await;
    let (contacts, legacy) = repos::open(connection, None).unwrap();
    let api = Api::new(contacts, legacy, Clock::new());
    assert_eq!(api.handle("GET", "/contacts", None).await.unwrap().status, 400);
}

#[tokio::test]
async fn clock_defaults_cover_now() {
    let _ = Clock::new().now();
    let _ = Clock::default().uuid();
}
