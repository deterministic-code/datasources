use std::path::{Path, PathBuf};
use std::process::Command;

fn sample() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo() -> PathBuf {
    sample().join("../../..").canonicalize().expect("repo")
}

fn ts() -> PathBuf {
    sample().join("../../typescript/contacts-crud-enriched-medium-rest")
}

fn datasources(args: &[&str], url: &str) {
    let status = Command::new("node")
        .arg(repo().join("cli/bin/datasources.mjs"))
        .args(args)
        .current_dir(ts())
        .env("PROVIDER", "sqlite")
        .env("DATASOURCE_URL", url)
        .status()
        .expect("datasources");
    assert!(status.success(), "datasources {}", args.join(" "));
}

fn migrate_up(db: &Path) {
    let setup = Command::new("npx")
        .args(["migrate-setup", "--provider", "sqlite", "--connection"])
        .arg(db)
        .current_dir(sample())
        .status()
        .expect("migrate-setup");
    assert!(setup.success(), "migrate-setup");
    let status = Command::new("npx")
        .args(["migrate-up", "--provider", "sqlite", "--connection"])
        .arg(db)
        .args(["--migrations-root", "sql"])
        .current_dir(sample())
        .status()
        .expect("migrate-up");
    assert!(status.success(), "migrate-up");
}

#[test]
fn generates_and_registers_contacts() {
    let project = sample();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let db = std::env::temp_dir().join(format!("contacts-gen-rs-{stamp}.sqlite"));
    let url = format!("sqlite:///{}", db.display());
    let spec = repo().join("samples/contacts-crud-enriched-medium-rest-shared/deterministic");
    datasources(
        &[
            "generate",
            "all",
            "--dialect",
            "rust",
            "--cwd",
            project.to_str().expect("sample"),
            "--input",
            spec.to_str().expect("spec"),
            "--output",
            "generated",
            "--no-migrate",
        ],
        &url,
    );
    migrate_up(&db);
    let run = Command::new("cargo")
        .args(["run", "--manifest-path"])
        .arg(project.join("Cargo.toml"))
        .args(["--bin", "generate-e2e"])
        .env("DATASOURCE_URL", &url)
        .output()
        .expect("cargo");
    let _ = std::fs::remove_file(&db);
    assert!(
        run.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
}
