use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

#[derive(Deserialize)]
struct CliSpec {
    now: String,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct Step {
    argv: Vec<String>,
    expect: Expect,
}

#[derive(Deserialize)]
struct Expect {
    exit: i32,
    stdout: String,
    stderr: String,
}

fn sample() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo() -> PathBuf {
    sample().join("../../..").canonicalize().expect("repo")
}

fn spec() -> CliSpec {
    serde_yaml::from_str(
        &fs::read_to_string(sample().join("../../user-roles-simple-cli-shared/cli.yaml")).expect("spec"),
    )
    .expect("yaml")
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

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default.to_string())
}

fn ensure_compose(service: &str) {
    if std::env::var("SKIP_DOCKER")
        .ok()
        .filter(|value| !value.is_empty())
        .is_some()
    {
        return;
    }
    let status = Command::new("docker")
        .args(["compose", "up", "-d", "--wait", service])
        .current_dir(repo())
        .status()
        .expect("docker");
    assert!(status.success(), "docker compose up {service}");
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

fn build() -> PathBuf {
    let status = Command::new("cargo")
        .args(["build", "--quiet"])
        .current_dir(sample())
        .status()
        .expect("build");
    assert!(status.success());
    std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| sample().join("target"))
        .join("debug/user-roles-simple-cli")
}

fn cli(bin: &Path, args: &[String], url: &str, schema: &str, now: &str) -> (i32, String, String) {
    let output = Command::new(bin)
        .args(args)
        .env("DATASOURCE_URL", url)
        .env("DATABASE_SCHEMA", schema)
        .env("CLI_NOW", now)
        .current_dir(sample())
        .output()
        .expect("cli");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn verbose() -> bool {
    matches!(std::env::var("VERBOSE").as_deref(), Ok("1") | Ok("true"))
}

fn script(url: &str, schema: &str, provider: &str) {
    let spec = spec();
    let bin = build();
    for step in &spec.steps {
        let (exit, stdout, stderr) = cli(&bin, &step.argv, url, schema, &spec.now);
        let cmd = step.argv.join(" ");
        if verbose() {
            println!("{provider}: {cmd}");
            println!("  assert exit {exit} == {}", step.expect.exit);
            println!("  assert stdout [{stdout}] == [{}]", step.expect.stdout);
            println!("  assert stderr [{stderr}] == [{}]", step.expect.stderr);
        }
        if exit != step.expect.exit || stdout != step.expect.stdout || stderr != step.expect.stderr {
            panic!(
                "{provider}: {cmd}: first mismatch\nexpected exit {} stdout [{}] stderr [{}]\nactual exit {exit} stdout [{stdout}] stderr [{stderr}]",
                step.expect.exit,
                step.expect.stdout,
                step.expect.stderr
            );
        }
    }
}

#[test]
fn sqlite() {
    let _guard = LOCK.lock().unwrap_or_else(|error| error.into_inner());
    let dir = std::env::temp_dir().join(format!("user-roles-{}", uuid()));
    fs::create_dir_all(&dir).expect("tmp");
    let url = format!("sqlite:///{}", dir.join("app.db").display());
    write_env(&url, "");
    setup();
    script(&url, "", "sqlite");
}

#[tokio::test]
async fn mysql() {
    let _guard = LOCK.lock().unwrap_or_else(|error| error.into_inner());
    ensure_compose("mysql");
    let url = env_or("MYSQL_URL", "mysql://ds:ds@127.0.0.1:3306/datasources");
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
    script(&isolated, "", "mysql");
    let mut conn = pool.get_conn().await.expect("mysql");
    mysql_async::prelude::Queryable::query_drop(&mut conn, format!("DROP DATABASE `{name}`"))
        .await
        .expect("drop");
}

#[tokio::test]
async fn postgres() {
    let _guard = LOCK.lock().unwrap_or_else(|error| error.into_inner());
    ensure_compose("postgres");
    let url = env_or("POSTGRES_URL", "postgres://ds:ds@127.0.0.1:5432/datasources");
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
    script(&url, &schema, "postgres");
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
