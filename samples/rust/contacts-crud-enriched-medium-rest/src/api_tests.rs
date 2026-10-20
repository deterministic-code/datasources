use crate::api::Api;
use crate::clock::Clock;
use crate::migrate;
use crate::repos;
use datasources::{CommandMiddleware, ConsoleCommandMiddleware, Driver, ExpectConsoleMiddleware};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const STAMP: &str = "2026-09-22T00:00:00.000Z";

fn by<'a>(rows: &'a Value, field: &str, value: &str) -> &'a Value {
    rows.as_array()
        .unwrap()
        .iter()
        .find(|row| row[field] == value)
        .unwrap()
}

fn signature(line: &str) -> String {
    let stripped = match line.rfind("] ") {
        Some(duration) if line.ends_with("ms") || line.ends_with("μs") => {
            line[..=duration].to_string()
        }
        _ => line.to_string(),
    };
    drop_float_zeros(&replace_uuids(&replace_dates(&stripped)))
}

fn drop_float_zeros(text: &str) -> String {
    let mut out = String::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'.'
            && i + 1 < bytes.len()
            && bytes[i + 1] == b'0'
            && i > 0
            && bytes[i - 1].is_ascii_digit()
            && (i + 2 == bytes.len() || matches!(bytes[i + 2], b',' | b'}' | b']'))
        {
            i += 2;
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn replace_uuids(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut out = String::new();
    while i < bytes.len() {
        if uuid_at(bytes, i) {
            out.push_str("<uuid>");
            i += 36;
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn uuid_at(bytes: &[u8], i: usize) -> bool {
    if i + 36 > bytes.len() {
        return false;
    }
    const DASHES: [usize; 4] = [8, 13, 18, 23];
    bytes[i..i + 36].iter().enumerate().all(|(offset, byte)| {
        if DASHES.contains(&offset) {
            *byte == b'-'
        } else {
            byte.is_ascii_hexdigit()
        }
    })
}

fn replace_dates(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut out = String::new();
    while i < bytes.len() {
        if iso_at(text, i) {
            let end = text[i..].find('Z').map(|offset| i + offset + 1).unwrap_or(i + 1);
            out.push_str("<date>");
            i = end;
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn iso_at(text: &str, i: usize) -> bool {
    let Some(slice) = text.get(i..i + 20) else { return false };
    slice.as_bytes()[..4].iter().all(u8::is_ascii_digit)
        && slice.as_bytes()[4] == b'-'
        && slice.as_bytes()[10] == b'T'
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
        Some(Arc::new(ExpectConsoleMiddleware::with_normalize(
            expected,
            || STAMP.into(),
            signature,
        )))
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
                r#"{"first_name":"Ada","last_name":"Lovelace","email":"ada@example.com","contact_source_name":"Manual","addresses":[{"line1":"12 Analytical Engine","city":"London","address_type_name":"Home"},{"line1":"1 Strand","city":"London","address_type_name":"Home"}],"phones":[{"number":"555-0100","label":"Mobile","phone_type_name":"Mobile"},{"number":"555-0199","label":"Work","phone_type_name":"Mobile"}],"settings":{"theme":"dark","density":"comfortable"},"prefs":{"home":{"locale":"en-GB","timezone":"Europe/London"},"travel":{"locale":"en-US","timezone":"America/New_York"}}}"#,
            ),
        )
        .await
        .unwrap();
    assert_eq!(created.status, 201);
    assert!(!created.body.contains("OldContactsTbl"));
    let saved: Value = serde_json::from_str(&created.body).unwrap();
    assert_eq!(saved["first_name"], "Ada");
    assert_eq!(saved["contact_source_name"], "Manual");
    assert_eq!(saved["addresses"].as_array().unwrap().len(), 2);
    assert_eq!(by(&saved["addresses"], "line1", "12 Analytical Engine")["city"], "London");
    assert_eq!(by(&saved["addresses"], "line1", "1 Strand")["address_type_name"], "Home");
    assert_eq!(saved["phones"].as_array().unwrap().len(), 2);
    assert_eq!(by(&saved["phones"], "number", "555-0100")["phone_type_name"], "Mobile");
    assert_eq!(by(&saved["phones"], "number", "555-0199")["label"], "Work");
    assert_eq!(saved["settings"]["theme"], "dark");
    assert_eq!(saved["settings"]["density"], "comfortable");
    assert_eq!(saved["prefs"]["home"]["locale"], "en-GB");
    assert_eq!(saved["prefs"]["home"]["timezone"], "Europe/London");
    assert_eq!(saved["prefs"]["travel"]["locale"], "en-US");
    let home_address_id = by(&saved["addresses"], "line1", "12 Analytical Engine")["id"].as_i64().unwrap();
    let mobile_phone_id = by(&saved["phones"], "number", "555-0100")["id"].as_i64().unwrap();
    api.handle(
        "POST",
        "/contacts",
        Some(
            r#"{"first_name":"Grace","last_name":"Hopper","uuid":"22222222-2222-2222-2222-222222222222","created":"2024-01-02T00:00:00Z","contact_source_name":"Imported","addresses":[{"uuid":"33333333-3333-3333-3333-333333333333","created":"2024-01-02T00:00:00Z","line1":"1","city":"NY","address_type_name":"Home"},{"uuid":"55555555-5555-5555-5555-555555555555","created":"2024-01-02T00:00:00Z","line1":"2","city":"Boston","address_type_name":"Home"}],"phones":[{"uuid":"44444444-4444-4444-4444-444444444444","created":"2024-01-02T00:00:00Z","number":"1","phone_type_name":"Mobile"},{"uuid":"66666666-6666-6666-6666-666666666666","created":"2024-01-02T00:00:00Z","number":"2","phone_type_name":"Mobile"}],"settings":{"imported":"yes","source":"csv"}}"#,
        ),
    )
    .await
    .unwrap();
    let id = saved["id"].as_i64().unwrap();
    let listed = serde_json::from_str::<Vec<Value>>(
        &api.handle("GET", "/contacts/", None).await.unwrap().body,
    )
    .unwrap();
    assert_eq!(listed.len(), 2);
    let ada = listed.iter().find(|row| row["first_name"] == "Ada").unwrap();
    let grace = listed.iter().find(|row| row["first_name"] == "Grace").unwrap();
    assert_eq!(ada["addresses"].as_array().unwrap().len(), 2);
    assert_eq!(ada["phones"].as_array().unwrap().len(), 2);
    assert_eq!(ada["settings"]["theme"], "dark");
    assert_eq!(grace["addresses"].as_array().unwrap().len(), 2);
    assert_eq!(grace["phones"].as_array().unwrap().len(), 2);
    assert_eq!(grace["settings"]["imported"], "yes");
    assert_eq!(grace["settings"]["source"], "csv");
    let updated = api
        .handle(
            "PUT",
            &format!("/contacts/?id={id}"),
            Some(&format!(
                r#"{{"first_name":"Ada","last_name":"Byron","email":null,"contact_source_name":"Manual","addresses":[{{"id":{home_address_id},"line1":"12 Analytical Engine","city":"Cambridge","address_type_name":"Home"}},{{"line1":"10 Downing","city":"London","address_type_name":"Home"}}],"phones":[{{"id":{mobile_phone_id},"number":"555-0101","label":"Mobile","phone_type_name":"Mobile"}},{{"number":"555-0200","label":"Office","phone_type_name":"Mobile"}}],"settings":{{"theme":"light","locale":"en"}},"prefs":{{"home":{{"locale":"en-GB","timezone":"Europe/Paris"}},"work":{{"locale":"en-US","timezone":"America/New_York"}}}}}}"#
            )),
        )
        .await
        .unwrap();
    assert_eq!(updated.status, 200);
    let changed: Value = serde_json::from_str(&updated.body).unwrap();
    assert_eq!(changed["last_name"], "Byron");
    assert_eq!(changed["addresses"].as_array().unwrap().len(), 2);
    assert_eq!(by(&changed["addresses"], "line1", "12 Analytical Engine")["city"], "Cambridge");
    assert_eq!(by(&changed["addresses"], "line1", "10 Downing")["city"], "London");
    assert!(changed["addresses"].as_array().unwrap().iter().all(|row| row["line1"] != "1 Strand"));
    assert_eq!(changed["phones"].as_array().unwrap().len(), 2);
    assert_eq!(by(&changed["phones"], "number", "555-0101")["id"], mobile_phone_id);
    assert_eq!(by(&changed["phones"], "number", "555-0200")["label"], "Office");
    assert!(changed["phones"].as_array().unwrap().iter().all(|row| row["number"] != "555-0199"));
    assert_eq!(changed["settings"]["theme"], "light");
    assert_eq!(changed["settings"]["locale"], "en");
    assert!(changed["settings"].get("density").is_none());
    assert_eq!(changed["prefs"]["home"]["timezone"], "Europe/Paris");
    assert_eq!(changed["prefs"]["work"]["locale"], "en-US");
    assert!(changed["prefs"].get("travel").is_none());
    let filtered = serde_json::from_str::<Vec<Value>>(
        &api
            .handle(
                "GET",
                &format!("/contacts/?first_name=Ada&last_name=Byron&id={id}"),
                None,
            )
            .await
            .unwrap()
            .body,
    )
    .unwrap();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0]["addresses"].as_array().unwrap().len(), 2);
    assert_eq!(filtered[0]["settings"]["theme"], "light");
    assert_eq!(
        api.handle("DELETE", &format!("/contacts/?id={id}"), None)
            .await
            .unwrap()
            .status,
        204
    );
    let remaining = serde_json::from_str::<Vec<Value>>(
        &api.handle("get", "/contacts/", None).await.unwrap().body,
    )
    .unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0]["first_name"], "Grace");
    assert_eq!(remaining[0]["addresses"].as_array().unwrap().len(), 2);
    assert_eq!(remaining[0]["settings"]["imported"], "yes");
    assert_eq!(remaining[0]["settings"]["source"], "csv");
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
