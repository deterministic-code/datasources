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
    #[serde(default)]
    save: HashMap<String, String>,
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
    let mut saved = HashMap::<String, String>::new();
    let mut error = None;
    for step in spec.steps {
        let path = substitute_text(&step.path, &saved);
        let body = step.body.as_ref().map(|value| substitute_node(value, &saved).to_string());
        match api.handle(&step.method, &path, body.as_deref()).await {
            Ok(result) => {
                let payload = serde_json::from_str::<Value>(&result.body).unwrap_or(Value::Null);
                if step.save_id {
                    if let Some(id) = payload.get("id") {
                        saved.insert("id".into(), scalar(id));
                    }
                }
                for (name, pointer) in &step.save {
                    saved.insert(name.clone(), at(&payload, pointer));
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

fn substitute_text(text: &str, saved: &HashMap<String, String>) -> String {
    let mut names: Vec<_> = saved.keys().cloned().collect();
    names.sort_by(|left, right| right.len().cmp(&left.len()));
    names.into_iter().fold(text.to_string(), |current, name| {
        current.replace(&format!("${name}"), saved.get(&name).map(String::as_str).unwrap_or(""))
    })
}

fn substitute_node(node: &Value, saved: &HashMap<String, String>) -> Value {
    match node {
        Value::Object(map) => Value::Object(map.iter().map(|(key, value)| (key.clone(), substitute_node(value, saved))).collect()),
        Value::Array(items) => Value::Array(items.iter().map(|item| substitute_node(item, saved)).collect()),
        Value::String(text) if text.starts_with('$') => saved
            .get(&text[1..])
            .map(|value| value.parse::<i64>().map(Value::from).unwrap_or_else(|_| Value::String(value.clone())))
            .unwrap_or_else(|| node.clone()),
        other => other.clone(),
    }
}

fn at(root: &Value, pointer: &str) -> String {
    let mut current = root;
    for part in pointer.split('.') {
        current = match part.parse::<usize>() {
            Ok(index) => current.get(index).unwrap_or(&Value::Null),
            Err(_) => current.get(part).unwrap_or(&Value::Null),
        };
    }
    scalar(current)
}

fn scalar(value: &Value) -> String {
    match value {
        Value::Number(number) => number.to_string(),
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}
