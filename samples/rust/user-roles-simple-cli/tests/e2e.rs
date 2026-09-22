use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

fn sample() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo() -> PathBuf {
    sample().join("../../..").canonicalize().expect("repo")
}

fn write_env(url: &str, schema: &str) {
    fs::write(
        sample().join(".env"),
        format!(
            "INSTALLED_PROVIDERS=sqlite,mysql,postgres\nDATASOURCE_URL={url}\nDATABASE_SCHEMA={schema}\n"
        ),
    )
    .expect("env");
}

fn setup() {
    let status = Command::new("npm")
        .args([
            "run",
            "setup",
            "--",
            "--cwd",
            sample().to_str().expect("cwd"),
            "--ecosystem",
            "rust",
            "--yes",
        ])
        .current_dir(repo())
        .status()
        .expect("setup");
    assert!(status.success());
}

fn cli(args: &[&str], url: &str, schema: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .env("DATASOURCE_URL", url)
        .env("DATABASE_SCHEMA", schema)
        .current_dir(sample())
        .output()
        .expect("cli");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn script(url: &str, schema: &str) {
    cli(&["add-role", "admin"], url, schema);
    cli(&["add-group", "staff"], url, schema);
    cli(&["add-user", "alice", "secret"], url, schema);
    cli(&["add-user-role", "alice", "admin"], url, schema);
    cli(&["add-user-group", "alice", "staff"], url, schema);
    let listed = cli(&["list-users-roles-groups"], url, schema);
    let users: serde_json::Value = serde_json::from_str(&listed).expect("json");
    assert_eq!(users[0]["username"], "alice");
    assert_eq!(users[0]["is_enabled"], true);
    assert_eq!(users[0]["roles"][0]["name"], "admin");
    assert_eq!(users[0]["groups"][0]["name"], "staff");
    assert!(!listed.contains("password"));
    assert!(!listed.contains("ugly_name"));
    cli(&["disable-user", "alice"], url, schema);
    let disabled: serde_json::Value =
        serde_json::from_str(&cli(&["list-users-roles-groups"], url, schema)).expect("json");
    assert_eq!(disabled[0]["is_enabled"], false);
}

#[test]
fn sqlite() {
    let _guard = LOCK.lock().expect("lock");
    let dir = std::env::temp_dir().join(format!("user-roles-{}", uuid()));
    fs::create_dir_all(&dir).expect("tmp");
    let url = format!("sqlite:///{}", dir.join("app.db").display());
    write_env(&url, "");
    setup();
    script(&url, "");
}

#[tokio::test]
async fn mysql() {
    let Some(url) = std::env::var("MYSQL_URL").ok().filter(|value| !value.is_empty()) else {
        return;
    };
    let _guard = LOCK.lock().expect("lock");
    let name = format!("ds_{}", &uuid()[..16]);
    let pool = mysql_async::Pool::new(url.as_str());
    {
        let mut conn = pool.get_conn().await.expect("mysql");
        mysql_async::prelude::Queryable::query_drop(&mut conn, format!("CREATE DATABASE `{name}`"))
            .await
            .expect("create");
    }
    let isolated = replace_mysql_db(&url, &name);
    write_env(&isolated, "");
    setup();
    script(&isolated, "");
    let mut conn = pool.get_conn().await.expect("mysql");
    mysql_async::prelude::Queryable::query_drop(&mut conn, format!("DROP DATABASE `{name}`"))
        .await
        .expect("drop");
}

#[tokio::test]
async fn postgres() {
    let Some(url) = std::env::var("POSTGRES_URL")
        .ok()
        .filter(|value| !value.is_empty())
    else {
        return;
    };
    let _guard = LOCK.lock().expect("lock");
    let schema = format!("ds_{}", &uuid()[..16]);
    let (client, connection) = tokio_postgres::connect(&url, tokio_postgres::NoTls)
        .await
        .expect("pg");
    tokio::spawn(async move {
        let _ = connection.await;
    });
    client
        .execute(&format!("CREATE SCHEMA {schema}"), &[])
        .await
        .expect("schema");
    write_env(&url, &schema);
    setup();
    script(&url, &schema);
    client
        .execute(&format!("DROP SCHEMA {schema} CASCADE"), &[])
        .await
        .expect("drop");
}

fn replace_mysql_db(url: &str, name: &str) -> String {
    match url.trim_end_matches('/').rfind('/') {
        Some(index) => format!("{}/{name}", &url[..index]),
        None => format!("{url}/{name}"),
    }
}

fn uuid() -> String {
    format!("{:?}", std::time::SystemTime::now())
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .collect()
}
