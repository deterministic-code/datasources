use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn sample() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo() -> PathBuf {
    sample().join("../../..").canonicalize().expect("repo")
}

fn run(cmd: &mut Command, label: &str) {
    let output = cmd.output().unwrap_or_else(|error| panic!("{label}: {error}"));
    assert!(
        output.status.success(),
        "{label}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn migrate_up(project: &Path, db: &Path) {
    let dir = project.join("sql/sqlite/migrations");
    let mut ups: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|error| panic!("migrations {}: {error}", dir.display()))
        .map(|entry| entry.expect("entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with("_up.sql"))
        })
        .collect();
    ups.sort();
    let connection = rusqlite::Connection::open(db).expect("sqlite");
    for path in ups {
        for batch in fs::read_to_string(&path)
            .expect("sql")
            .split("\nGO\n")
            .map(str::trim)
            .filter(|batch| !batch.is_empty() && *batch != "GO")
        {
            connection
                .execute_batch(batch)
                .unwrap_or_else(|error| panic!("{}: {error}\n{batch}", path.display()));
        }
    }
}

#[test]
fn generates_and_registers_contacts() {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let project = std::env::temp_dir().join(format!("contacts-gen-rs-{stamp}"));
    let db = project.join("app.sqlite");
    let url = format!("sqlite:///{}", db.display());
    let rust = repo().join("rust");
    fs::create_dir_all(project.join("src")).expect("src");
    fs::write(
        project.join("Cargo.toml"),
        format!(
            r#"[package]
name = "contacts-generate-e2e"
version = "0.1.0"
edition = "2021"

[dependencies]
ctor = "0.4"
datasources = {{ path = "{}" }}
datasources-sqlite = {{ path = "{}" }}
tokio = {{ version = "1", features = ["macros", "rt-multi-thread"] }}
"#,
            rust.join("datasources").display(),
            rust.join("providers/sqlite").display()
        ),
    )
    .expect("Cargo.toml");
    fs::write(
        project.join(".env"),
        format!("INSTALLED_PROVIDERS=sqlite\nPROVIDER=sqlite\nDATASOURCE_URL={url}\n"),
    )
    .expect(".env");
    fs::write(
        project.join("src/datasources_providers.rs"),
        "pub fn register() {\n    datasources::register(\"sqlite\", std::sync::Arc::new(datasources_sqlite::SqliteDriver));\n}\n",
    )
    .expect("providers");
    fs::copy(sample().join("e2e/main.rs"), project.join("src/main.rs")).expect("main");
    let spec = repo().join("samples/contacts-crud-enriched-medium-rest-shared/deterministic");
    run(
        Command::new("node")
            .arg(repo().join("cli/bin/datasources.mjs"))
            .args([
                "generate",
                "all",
                "--dialect",
                "rust",
                "--cwd",
                project.to_str().expect("project"),
                "--input",
                spec.to_str().expect("spec"),
                "--output",
                "generated",
                "--no-auto-migrate-up",
            ])
            .env("PROVIDER", "sqlite")
            .env("DATASOURCE_URL", &url)
            .env("INSTALLED_PROVIDERS", "sqlite"),
        "generate",
    );
    migrate_up(&project, &db);
    run(
        Command::new("cargo")
            .args(["run", "--manifest-path"])
            .arg(project.join("Cargo.toml"))
            .env("DATASOURCE_URL", &url)
            .env("PROVIDER", "sqlite")
            .env("INSTALLED_PROVIDERS", "sqlite")
            .env("CARGO_TARGET_DIR", project.join("target")),
        "cargo run",
    );
    let _ = fs::remove_dir_all(&project);
}
