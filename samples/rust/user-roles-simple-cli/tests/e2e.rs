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

fn remove_compose(service: &str) {
    docker_ok(&["compose", "rm", "-sf", "-v", service]);
    docker_ok(&["rm", "-f", &format!("datasources-{service}-1")]);
}

fn fresh_compose(service: &str) {
    remove_compose(service);
    docker(&["compose", "up", "-d", "--force-recreate", "--wait", service]);
    if service == "mysql" {
        docker(&[
            "compose",
            "exec",
            "-T",
            "mysql",
            "mysql",
            "-uroot",
            "-proot",
            "-e",
            "GRANT ALL PRIVILEGES ON *.* TO 'ds'@'%'; FLUSH PRIVILEGES;",
        ]);
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
                if text.to_ascii_uppercase().starts_with("SELECT") {
                    unquote(&text)
                } else {
                    text
                }
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
        .then(|| signed.strip_prefix(prefix).and_then(|rest| rest.strip_suffix(']')))
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
    write_env(&url, "");
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
    fresh_compose("postgres");
    let _stop = StopCompose("postgres");
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
