use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn sample() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo() -> PathBuf {
    sample().join("../../..").canonicalize().expect("repo")
}

fn generate(cwd: &Path) {
    let status = Command::new("node")
        .args([
            "--import",
            "tsx",
            repo()
                .join("cli/src/main.ts")
                .to_str()
                .expect("cli"),
            "generate",
            "all",
            "--dialect",
            "rust",
            "--input",
            repo()
                .join("samples/contacts-crud-enriched-medium-rest-shared/deterministic")
                .to_str()
                .expect("spec"),
            "--output",
            "generated",
            "--cwd",
            cwd.to_str().expect("cwd"),
        ])
        .current_dir(repo().join("cli"))
        .status()
        .expect("generate");
    assert!(status.success(), "generate");
}

#[test]
fn generates_and_registers_contacts() {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let tmp = std::env::temp_dir().join(format!("contacts-gen-rs-{stamp}"));
    fs::create_dir_all(tmp.join("src")).expect("tmp");
    generate(&tmp);
    fs::copy(sample().join("src/schema.rs"), tmp.join("src/schema.rs")).expect("schema");
    fs::copy(sample().join("e2e/main.rs"), tmp.join("src/main.rs")).expect("main");
    let datasources = repo().join("rust/datasources");
    let sqlite = repo().join("rust/providers/sqlite");
    fs::write(
        tmp.join("Cargo.toml"),
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

[lints.rust]
dead_code = "allow"
unused_imports = "allow"
"#,
            datasources.display(),
            sqlite.display()
        ),
    )
    .expect("manifest");
    let run = Command::new("cargo")
        .args(["run", "--manifest-path"])
        .arg(tmp.join("Cargo.toml"))
        .output()
        .expect("cargo");
    let _ = fs::remove_dir_all(&tmp);
    assert!(
        run.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
}

#[test]
fn drops_extra_fields_and_tables_across_three_versions() {
    let tmp = std::env::temp_dir().join(format!(
        "contacts-mig-{}",
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
                .join("samples/contacts-crud-enriched-medium-rest-shared/deterministic")
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
        .execute_batch(
            &contacts_crud_enriched_medium_rest::schema::statements(
                &fs::read_to_string(&files[0]).expect("0001"),
            )
            .join("\n"),
        )
        .expect("0001");
    let names = table_names(&connection);
    assert!(names.iter().any(|name| name == "tags"));
    assert!(names.iter().any(|name| name == "note_pads"));
    assert!(columns(&connection, "contacts").iter().any(|name| name == "middle_name"));
    assert!(columns(&connection, "addresses").iter().any(|name| name == "line3"));
    connection
        .execute_batch(&fs::read_to_string(&files[1]).expect("0002"))
        .expect("0002");
    assert!(table_names(&connection).iter().all(|name| name != "tags"));
    assert!(columns(&connection, "contacts").iter().all(|name| name != "middle_name"));
    connection
        .execute_batch(&fs::read_to_string(&files[2]).expect("0003"))
        .expect("0003");
    assert!(table_names(&connection).iter().all(|name| name != "note_pads"));
    assert!(columns(&connection, "addresses").iter().all(|name| name != "line3"));
    assert!(table_names(&connection).iter().any(|name| name == "contacts"));
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
