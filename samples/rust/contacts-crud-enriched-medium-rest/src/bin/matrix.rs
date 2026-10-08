use contacts_crud_enriched_medium_rest::api::Api;
use contacts_crud_enriched_medium_rest::clock::Clock;
use contacts_crud_enriched_medium_rest::repos;
use contacts_crud_enriched_medium_rest::schema;
use datasources::{ConsoleCommandMiddleware, Driver, Native};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct Spec {
    now: String,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct Step {
    method: String,
    path: String,
    body: Option<Value>,
    #[serde(rename = "saveId", default)]
    save_id: bool,
}

#[tokio::main]
async fn main() {
    let provider = std::env::var("PROVIDER").unwrap_or_else(|_| "sqlite".into());
    let url = std::env::var("DATASOURCE_URL").unwrap_or_else(|_| "sqlite:///:memory:".into());
    let log = std::env::var("E2E_MATRIX_LOG").expect("E2E_MATRIX_LOG");
    let spec: Spec = serde_yaml::from_str(
        &std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../contacts-crud-enriched-medium-rest-shared/round-trip.yaml"),
        )
        .expect("spec"),
    )
    .expect("yaml");
    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), url);
    let driver: Box<dyn Driver> = match provider.as_str() {
        "sqlite" => Box::new(datasources_sqlite::SqliteDriver),
        "mysql" => Box::new(datasources_mysql::MysqlDriver),
        "postgres" => Box::new(datasources_postgres::PostgresDriver),
        name => panic!("{name}"),
    };
    let connection = driver.connection(Some(&options));
    connection.open().await.expect("open");
    let command = connection.create_command();
    if provider == "sqlite" {
        command
            .execute_native(&Native::new("PRAGMA foreign_keys = ON"), None)
            .await
            .expect("pragma");
    }
    let sql = std::env::var("E2E_MATRIX_SQL")
        .ok()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .unwrap_or_else(|| schema::GENERATED_SQL.to_string());
    for statement in schema::statements(&sql) {
        command
            .execute_native(&Native::new(statement), None)
            .await
            .expect("migrate");
    }
    let n = Arc::new(Mutex::new(0u32));
    let clock = Clock::new();
    let stamp = spec.now.clone();
    clock.set_now(move || stamp.clone());
    {
        let n = Arc::clone(&n);
        clock.set_uuid(move || {
            let mut guard = n.lock().expect("n");
            *guard += 1;
            format!("00000000-0000-0000-0000-{:012}", *guard)
        });
    }
    let lines = Arc::new(Mutex::new(Vec::<String>::new()));
    let now = spec.now.clone();
    let middleware = Arc::new(
        ConsoleCommandMiddleware::with_sink(
            {
                let lines = Arc::clone(&lines);
                move |line| lines.lock().expect("log").push(line.to_string())
            },
            move || now.clone(),
        )
        .options(HashMap::from([("isVerbose".into(), "true".into())])),
    );
    let (contacts, legacy) = repos::open(connection, Some(middleware)).expect("repos");
    let api = Api::new(contacts, legacy, clock);
    let mut id = String::new();
    let mut error = None;
    for step in spec.steps {
        let path = step.path.replace("$id", &id);
        let body = step.body.as_ref().map(|value| value.to_string());
        match api.handle(&step.method, &path, body.as_deref()).await {
            Ok(result) => {
                if step.save_id {
                    id = serde_json::from_str::<Value>(&result.body)
                        .ok()
                        .and_then(|value| value.get("id").cloned())
                        .map(|value| value.to_string())
                        .unwrap_or_default();
                }
            }
            Err(err) => {
                error = Some(err);
                break;
            }
        }
    }
    if let Some(dir) = std::path::Path::new(&log).parent() {
        std::fs::create_dir_all(dir).ok();
    }
    std::fs::write(&log, format!("{}\n", lines.lock().expect("log").join("\n"))).expect("write");
    if let Some(err) = error {
        panic!("{err}");
    }
}
