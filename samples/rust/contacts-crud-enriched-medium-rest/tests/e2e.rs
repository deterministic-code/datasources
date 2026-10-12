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
            "--experimental-strip-types",
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
