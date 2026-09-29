use datasources::{
    create, default, defaults, registered, reset, DatasourceError, ProviderCatalog,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

static ENV: Mutex<()> = Mutex::new(());

fn clear_env() {
    for key in ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"] {
        std::env::remove_var(key);
    }
}

fn plugins() -> &'static Path {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let plugin_target = root.join("target/plugin-build");
        let status = Command::new("cargo")
            .args([
                "build",
                "-p",
                "datasources-json",
                "-p",
                "datasources-memory",
                "--offline",
                "--target-dir",
            ])
            .arg(&plugin_target)
            .current_dir(&root)
            .env_remove("CARGO_TARGET_DIR")
            .status()
            .expect("cargo build");
        assert!(status.success(), "json and memory plugins failed to build");
        let debug = plugin_target.join("debug");
        let dir = root.join("target/datasources-plugins-test");
        std::fs::create_dir_all(&dir).unwrap();
        for location in ["datasources_json", "datasources_memory"] {
            let file = lib_file(location);
            std::fs::copy(debug.join(&file), dir.join(&file))
                .unwrap_or_else(|error| panic!("copy {file}: {error}"));
        }
        dir
    })
}

fn lib_file(location: &str) -> String {
    match std::env::consts::OS {
        "windows" => format!("{location}.dll"),
        "macos" => format!("lib{location}.dylib"),
        _ => format!("lib{location}.so"),
    }
}

struct Plugins {
    _lock: std::sync::MutexGuard<'static, ()>,
}

impl Plugins {
    fn enter() -> Self {
        let lock = ENV.lock().expect("env");
        std::env::set_var("DATASOURCES_PLUGIN_DIR", plugins());
        reset();
        clear_env();
        Self { _lock: lock }
    }
}

impl Drop for Plugins {
    fn drop(&mut self) {
        clear_env();
        std::env::remove_var("DATASOURCES_PLUGIN_DIR");
        reset();
    }
}

fn opts(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
        .collect()
}

#[test]
fn default_stores_location_and_module() {
    let sqlite = &ProviderCatalog::default()["sqlite"];
    assert_eq!(sqlite.location, "datasources_sqlite");
    assert_eq!(sqlite.module, "datasources_sqlite_driver");
    assert_eq!(
        ProviderCatalog::default()["sqlserver"].module,
        "datasources_sqlserver_driver"
    );
    assert_eq!(
        ProviderCatalog::default()["mongodb"].module,
        "datasources_mongodb_driver"
    );
    assert_eq!(
        ProviderCatalog::default()["dynamodb"].module,
        "datasources_dynamodb_driver"
    );
}

#[test]
fn full_constructor_stores_location_and_module() {
    let catalog = ProviderCatalog::new("acme", "acme_data", "acme_data_driver");
    assert_eq!(catalog.name, "acme");
    assert_eq!(catalog.location, "acme_data");
    assert_eq!(catalog.module, "acme_data_driver");
}

#[test]
fn default_lists_monorepo_providers() {
    let mut keys: Vec<_> = ProviderCatalog::default().keys().cloned().collect();
    keys.sort();
    assert_eq!(
        keys,
        [
            "dynamodb", "json", "memory", "mongodb", "mysql", "oracle", "postgres", "sqlite",
            "sqlserver", "xml", "yaml",
        ]
    );
}

#[test]
fn defaults_registers_installed_providers() {
    let _plugins = Plugins::enter();
    defaults(opts(&[
        ("INSTALLED_PROVIDERS", "json,memory"),
        ("DATASOURCE_URL", "memory://"),
    ]))
    .unwrap();
    assert_eq!(registered(), vec!["json".to_string(), "memory".to_string()]);
    assert_eq!(
        create(Some(&opts(&[
            ("PROVIDER", "memory"),
            ("DATASOURCE_URL", "memory://"),
        ])))
        .unwrap()
        .provider(),
        "memory"
    );
}

#[test]
fn create_does_not_take_installed_providers() {
    let _plugins = Plugins::enter();
    defaults(opts(&[("INSTALLED_PROVIDERS", "json,memory")])).unwrap();
    assert_eq!(
        create(Some(&opts(&[
            ("PROVIDER", "json"),
            ("DATASOURCE_URL", "json://"),
        ])))
        .unwrap()
        .provider(),
        "json"
    );
    assert_eq!(registered(), vec!["json".to_string(), "memory".to_string()]);
}

#[test]
fn create_reads_connection_from_environment() {
    let _plugins = Plugins::enter();
    std::env::set_var("INSTALLED_PROVIDERS", "json,memory");
    std::env::set_var("DATASOURCE_URL", "memory://");
    std::env::set_var("PROVIDER", "memory");
    assert_eq!(create(None).unwrap().provider(), "memory");
    assert_eq!(default().unwrap().provider(), "memory");
    assert_eq!(registered(), vec!["json".to_string(), "memory".to_string()]);
}

#[test]
fn default_overrides_env() {
    let _plugins = Plugins::enter();
    std::env::set_var("INSTALLED_PROVIDERS", "json,memory");
    std::env::set_var("PROVIDER", "json");
    std::env::set_var("DATASOURCE_URL", "json://");
    assert_eq!(default().unwrap().provider(), "json");
}

#[test]
fn missing_package_or_unknown_name() {
    let _plugins = Plugins::enter();
    assert!(matches!(
        defaults(opts(&[("INSTALLED_PROVIDERS", "sqlite")])),
        Err(DatasourceError::ProviderNotInstalled(_))
    ));
    reset();
    assert!(matches!(
        defaults(opts(&[("INSTALLED_PROVIDERS", "nope")])),
        Err(DatasourceError::ProviderNotInstalled(_))
    ));
}

#[test]
fn url_without_provider_throws() {
    let _plugins = Plugins::enter();
    assert_eq!(
        defaults(opts(&[("DATASOURCE_URL", "not-a-url")]))
            .unwrap_err()
            .to_string(),
        "DATASOURCE_URL has no provider"
    );
}

#[tokio::test]
async fn loaded_memory_opens() {
    let _plugins = Plugins::enter();
    std::env::set_var("INSTALLED_PROVIDERS", "memory");
    std::env::set_var("PROVIDER", "memory");
    std::env::set_var("DATASOURCE_URL", "memory://");
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    assert!(!connection.is_open());
    connection.open().await.unwrap();
    assert!(connection.is_open());
}
