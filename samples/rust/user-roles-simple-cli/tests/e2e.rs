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
    #[serde(default)]
    queries: Vec<String>,
    #[serde(default)]
    inserts: Vec<String>,
    #[serde(default)]
    calls: Vec<String>,
}

fn sample() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo() -> PathBuf {
    sample().join("../../..").canonicalize().expect("repo")
}

fn spec() -> CliSpec {
    serde_yaml::from_str(
        &fs::read_to_string(sample().join("../../user-roles-simple-cli-shared/cli.yaml"))
            .expect("spec"),
    )
    .expect("yaml")
}

fn with_schema(url: &str, schema: &str) -> String {
    if schema.is_empty() {
        url.to_string()
    } else if url.contains('?') {
        format!("{url}&schema={schema}")
    } else {
        format!("{url}?schema={schema}")
    }
}

fn write_env(url: &str, schema: &str, provider: &str) {
    let url = with_schema(url, schema);
    fs::write(
        sample().join(".env"),
        format!(
            "INSTALLED_PROVIDERS=sqlite,mysql,postgres\nPROVIDER={provider}\nDATASOURCE_URL={url}\n"
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

fn skip_docker() -> bool {
    std::env::var("SKIP_DOCKER")
        .ok()
        .filter(|value| !value.is_empty())
        .is_some()
}

fn docker(args: &[&str]) {
    if skip_docker() {
        return;
    }
    let status = Command::new("docker")
        .args(args)
        .current_dir(repo())
        .status()
        .expect("docker");
    assert!(status.success(), "docker {}", args.join(" "));
}

fn docker_ok(args: &[&str]) {
    if skip_docker() {
        return;
    }
    let _ = Command::new("docker")
        .args(args)
        .current_dir(repo())
        .status();
}

fn compose_args<'a>(args: &'a [&str]) -> Vec<&'a str> {
    let mut out = vec!["compose", "-p", "datasources", "-f", "docker/compose.yml"];
    out.extend(args.iter().copied());
    out
}

fn remove_compose(service: &str) {
    docker_ok(&compose_args(&["rm", "-sf", "-v", service]));
    docker_ok(&["rm", "-f", &format!("datasources-{service}-1")]);
}

fn fresh_compose(service: &str) {
    remove_compose(service);
    docker(&compose_args(&["up", "-d", "--wait", service]));
    if service == "mysql" {
        docker(&compose_args(&[
            "exec",
            "-T",
            "mysql",
            "mysql",
            "-uroot",
            "-proot",
            "-e",
            "GRANT ALL PRIVILEGES ON *.* TO 'ds'@'%'; FLUSH PRIVILEGES;",
        ]));
    }
}

fn stop_compose(service: &str) {
    remove_compose(service);
}

struct StopCompose(&'static str);

impl Drop for StopCompose {
    fn drop(&mut self) {
        stop_compose(self.0);
    }
}

struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
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

fn cli(
    bin: &Path,
    args: &[String],
    url: &str,
    schema: &str,
    provider: &str,
    now: &str,
) -> (i32, String, String) {
    let output = Command::new(bin)
        .args(args)
        .env("PROVIDER", provider)
        .env("DATASOURCE_URL", with_schema(url, schema))
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
        let (exit, stdout, stderr) = cli(&bin, &step.argv, url, schema, provider, &spec.now);
        let cmd = step.argv.join(" ");
        let (leftover, sql) = commands(&stderr);
        let writes = if provider == "mysql" {
            &step.expect.calls
        } else {
            &step.expect.inserts
        };
        let expected_sql: Vec<String> = step
            .expect
            .queries
            .iter()
            .chain(writes.iter())
            .cloned()
            .collect();
        let actual_sql: Vec<String> = sql
            .into_iter()
            .map(|text| {
                let text = if text.to_ascii_uppercase().starts_with("SELECT") {
                    unquote(&text)
                } else {
                    text
                };
                regex::Regex::new(r"::[A-Z][A-Z0-9_]*")
                    .expect("cast")
                    .replace_all(&text, "")
                    .into_owned()
            })
            .collect();
        if verbose() {
            println!("{provider}: {cmd}");
            println!("  assert exit {exit} == {}", step.expect.exit);
            println!("  assert stdout [{stdout}] == [{}]", step.expect.stdout);
            println!("  assert stderr [{leftover}] == [{}]", step.expect.stderr);
            println!(
                "  assert commands [{}] == [{}]",
                actual_sql.join(" | "),
                expected_sql.join(" | ")
            );
        }
        if exit != step.expect.exit
            || stdout != step.expect.stdout
            || leftover != step.expect.stderr
            || actual_sql != expected_sql
        {
            panic!(
                "{provider}: {cmd}: first mismatch\nexpected exit {} stdout [{}] stderr [{}] commands [{}]\nactual exit {exit} stdout [{stdout}] stderr [{leftover}] commands [{}]",
                step.expect.exit,
                step.expect.stdout,
                step.expect.stderr,
                expected_sql.join(" | "),
                actual_sql.join(" | ")
            );
        }
    }
}

fn commands(stderr: &str) -> (String, Vec<String>) {
    let mut leftover = Vec::new();
    let mut sql = Vec::new();
    for line in stderr.replace("\r\n", "\n").split('\n') {
        if line.contains("[datasource][Start]") || line.contains("[datasource][Finish]") {
            if let Some(text) = start_sql(line) {
                sql.push(text);
            }
        } else if !line.is_empty() {
            leftover.push(line.to_string());
        }
    }
    (leftover.join("\n"), sql)
}

fn signature(line: &str) -> String {
    let body = line
        .find("]-")
        .map(|index| &line[index + 2..])
        .unwrap_or(line);
    if let Some(duration) = body.rfind("] ") {
        if body.ends_with("ms") || body.ends_with("μs") {
            return body[..=duration].to_string();
        }
    }
    body.to_string()
}

fn start_sql(line: &str) -> Option<String> {
    let signed = signature(line);
    let prefix = "[datasource][Start]-[";
    signed
        .starts_with(prefix)
        .then(|| {
            signed
                .strip_prefix(prefix)
                .and_then(|rest| rest.strip_suffix(']'))
        })
        .flatten()
        .map(str::to_string)
}

fn unquote(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '`' || ch == '"' {
            for inner in chars.by_ref() {
                if inner == ch {
                    break;
                }
                out.push(inner);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

#[test]
fn sqlite() {
    let _guard = LOCK.lock().unwrap_or_else(|error| error.into_inner());
    let dir = TempDir(std::env::temp_dir().join(format!("user-roles-{}", uuid())));
    fs::create_dir_all(&dir.0).expect("tmp");
    let url = format!("sqlite:///{}", dir.0.join("app.db").display());
    write_env(&url, "", "sqlite");
    setup();
    script(&url, "", "sqlite");
}

#[tokio::test]
async fn mysql() {
    let _guard = LOCK.lock().unwrap_or_else(|error| error.into_inner());
    fresh_compose("mysql");
    let _stop = StopCompose("mysql");
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
    write_env(&isolated, "", "mysql");
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
    fresh_compose("postgres");
    let _stop = StopCompose("postgres");
    let url = env_or(
        "POSTGRES_URL",
        "postgres://ds:ds@127.0.0.1:5432/datasources",
    );
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
    write_env(&url, &schema, "postgres");
    setup();
    script(&url, &schema, "postgres");
    client
        .execute(&format!("DROP SCHEMA {schema} CASCADE"), &[])
        .await
        .expect("drop");
}

#[derive(Deserialize)]
struct TranscriptSpec {
    now: String,
    steps: Vec<TranscriptStep>,
}

#[derive(Deserialize)]
struct TranscriptStep {
    argv: Vec<String>,
}

async fn console_transcript(
    file: &str,
    procedures: bool,
    connection: std::sync::Arc<dyn datasources::Connection>,
) {
    let shared = sample().join("../../user-roles-simple-cli-shared");
    let spec: TranscriptSpec =
        serde_yaml::from_str(&fs::read_to_string(shared.join("cli.yaml")).expect("spec"))
            .expect("yaml");
    let expected = fs::read_to_string(shared.join(file))
        .expect("log")
        .lines()
        .filter(|line| !line.is_empty())
        .map(user_roles_simple_cli::console_log::normalize_console_line)
        .collect();
    connection.open().await.expect("open");
    let command = connection.create_command();
    user_roles_simple_cli::migrate::apply(command.as_ref(), connection.provider())
        .await
        .expect("migrate");
    let clock = spec.now.clone();
    let middleware = std::sync::Arc::new(datasources::ExpectConsoleMiddleware::with_normalize(
        expected,
        move || clock.clone(),
        user_roles_simple_cli::console_log::normalize_console_line,
    ));
    command.add_middleware(middleware.clone());
    let stamp = spec.now.clone();
    let app = user_roles_simple_cli::app::App::new(command.as_ref(), connection.provider())
        .procedures(procedures)
        .with_clock(
            move || stamp.clone(),
            |_| Ok("hash".into()),
            || vec![1, 2, 3],
        );
    for step in spec.steps {
        app.run(user_roles_simple_cli::parse::args(&step.argv).expect("parse"))
            .await
            .expect("step");
    }
    middleware.complete().expect("transcript");
}

#[tokio::test]
async fn console_transcript_mysql() {
    use datasources::Driver;
    let _guard = LOCK.lock().unwrap_or_else(|error| error.into_inner());
    fresh_compose("mysql");
    let _stop = StopCompose("mysql");
    let url = env_or("MYSQL_URL", "mysql://ds:ds@127.0.0.1:3306/datasources");
    let name = format!("ds_{}", &uuid()[..16]);
    let procedures = format!("ds_{}", &uuid()[..16]);
    let pool = mysql_async::Pool::new(url.as_str());
    {
        let mut conn = pool.get_conn().await.expect("mysql");
        mysql_async::prelude::Queryable::query_drop(&mut conn, format!("CREATE DATABASE `{name}`"))
            .await
            .expect("create");
        mysql_async::prelude::Queryable::query_drop(
            &mut conn,
            format!("CREATE DATABASE `{procedures}`"),
        )
        .await
        .expect("procedures");
    }
    let mut options = std::collections::HashMap::new();
    options.insert("DATASOURCE_URL".into(), replace_mysql_db(&url, &name));
    console_transcript(
        "mysql.log",
        false,
        datasources_mysql::MysqlDriver.connection(Some(&options)),
    )
    .await;
    let mut procedure_options = std::collections::HashMap::new();
    procedure_options.insert("DATASOURCE_URL".into(), replace_mysql_db(&url, &procedures));
    console_transcript(
        "mysql-procedures.log",
        true,
        datasources_mysql::MysqlDriver.connection(Some(&procedure_options)),
    )
    .await;
    let mut conn = pool.get_conn().await.expect("mysql");
    mysql_async::prelude::Queryable::query_drop(&mut conn, format!("DROP DATABASE `{name}`"))
        .await
        .expect("drop");
    mysql_async::prelude::Queryable::query_drop(&mut conn, format!("DROP DATABASE `{procedures}`"))
        .await
        .expect("drop procedures");
}

#[tokio::test]
async fn console_transcript_postgres() {
    use datasources::Driver;
    let _guard = LOCK.lock().unwrap_or_else(|error| error.into_inner());
    fresh_compose("postgres");
    let _stop = StopCompose("postgres");
    let url = env_or(
        "POSTGRES_URL",
        "postgres://ds:ds@127.0.0.1:5432/datasources",
    );
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
    let mut options = std::collections::HashMap::new();
    options.insert("DATASOURCE_URL".into(), with_schema(&url, &schema));
    console_transcript(
        "postgres.log",
        false,
        datasources_postgres::PostgresDriver.connection(Some(&options)),
    )
    .await;
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

#[test]
fn drops_extra_fields_and_tables_across_three_versions() {
    let tmp = std::env::temp_dir().join(format!(
        "user-roles-mig-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    fs::create_dir_all(&tmp).expect("tmp");
    let status = Command::new("node")
        .args([
            "--import",
            "tsx",
            repo().join("cli/src/main.ts").to_str().expect("cli"),
            "migrations",
            "create",
            "--input",
            repo()
                .join("samples/user-roles-simple-cli-shared/deterministic")
                .to_str()
                .expect("spec"),
            "--migrations-root",
            tmp.join("sql").to_str().expect("sql"),
        ])
        .current_dir(repo().join("cli"))
        .status()
        .expect("generate");
    assert!(status.success(), "migrations create");
    let dir = tmp.join("sql/sqlite/migrations");
    let mut files: Vec<_> = fs::read_dir(&dir)
        .expect("migrations")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.len() == 11 && name.ends_with("_up.sql"))
        })
        .collect();
    files.sort();
    assert_eq!(
        files
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        ["0001_up.sql", "0002_up.sql", "0003_up.sql"]
    );
    let connection = rusqlite::Connection::open_in_memory().expect("sqlite");
    connection.execute_batch("PRAGMA foreign_keys = ON").expect("pragma");
    connection
        .execute_batch(&fs::read_to_string(&files[0]).expect("0001"))
        .expect("0001");
    let names = table_names(&connection);
    assert!(names.iter().any(|name| name == "sessions"));
    assert!(names.iter().any(|name| name == "audit_logs"));
    assert!(columns(&connection, "users").iter().any(|name| name == "nickname"));
    assert!(columns(&connection, "roles").iter().any(|name| name == "description"));
    connection
        .execute_batch(&fs::read_to_string(&files[1]).expect("0002"))
        .expect("0002");
    let names = table_names(&connection);
    assert!(names.iter().all(|name| name != "sessions"));
    assert!(columns(&connection, "users").iter().all(|name| name != "nickname"));
    connection
        .execute_batch(&fs::read_to_string(&files[2]).expect("0003"))
        .expect("0003");
    let names = table_names(&connection);
    assert!(names.iter().all(|name| name != "audit_logs"));
    assert!(columns(&connection, "roles").iter().all(|name| name != "description"));
    assert!(names.iter().any(|name| name == "users"));
    assert!(names.iter().any(|name| name == "roles"));
    assert!(names.iter().any(|name| name == "groups"));
    let _ = fs::remove_dir_all(&tmp);
}

fn table_names(connection: &rusqlite::Connection) -> Vec<String> {
    let mut statement = connection
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
        .expect("tables");
    statement
        .query_map([], |row| row.get(0))
        .expect("query")
        .map(|name| name.expect("name"))
        .collect()
}

fn columns(connection: &rusqlite::Connection, table: &str) -> Vec<String> {
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info(\"{table}\")"))
        .expect("columns");
    statement
        .query_map([], |row| row.get(1))
        .expect("query")
        .map(|name| name.expect("name"))
        .collect()
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    let ticks = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    format!("{ticks:x}{:x}", N.fetch_add(1, Ordering::Relaxed))
}
