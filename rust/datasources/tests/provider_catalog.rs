use datasources::{
    create, default, defaults, register, registered, reset, DatasourceError, ProviderCatalog,
};
use datasources_memory::MemoryDriver;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

static ENV: Mutex<()> = Mutex::new(());

fn clear_env() {
    for key in ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"] {
        std::env::remove_var(key);
    }
}

struct Isolated {
    _lock: std::sync::MutexGuard<'static, ()>,
}

impl Isolated {
    fn enter() -> Self {
        let lock = ENV.lock().expect("env");
        reset();
        clear_env();
        Self { _lock: lock }
    }
}

impl Drop for Isolated {
    fn drop(&mut self) {
        clear_env();
        reset();
    }
}

fn opts(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
        .collect()
}

fn bind_memory() {
    register("memory", Arc::new(MemoryDriver));
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
    let _isolated = Isolated::enter();
    bind_memory();
    defaults(opts(&[
        ("INSTALLED_PROVIDERS", "memory"),
        ("DATASOURCE_URL", "memory://"),
    ]))
    .unwrap();
    assert_eq!(registered(), vec!["memory".to_string()]);
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
    let _isolated = Isolated::enter();
    bind_memory();
    defaults(opts(&[("INSTALLED_PROVIDERS", "memory")])).unwrap();
    assert_eq!(
        create(Some(&opts(&[
            ("PROVIDER", "memory"),
            ("DATASOURCE_URL", "memory://"),
        ])))
        .unwrap()
        .provider(),
        "memory"
    );
    assert_eq!(registered(), vec!["memory".to_string()]);
}

#[test]
fn create_reads_connection_from_environment() {
    let _isolated = Isolated::enter();
    bind_memory();
    std::env::set_var("INSTALLED_PROVIDERS", "memory");
    std::env::set_var("DATASOURCE_URL", "memory://");
    std::env::set_var("PROVIDER", "memory");
    assert_eq!(create(None).unwrap().provider(), "memory");
    assert_eq!(default().unwrap().provider(), "memory");
    assert_eq!(registered(), vec!["memory".to_string()]);
}

#[test]
fn default_overrides_env() {
    let _isolated = Isolated::enter();
    bind_memory();
    std::env::set_var("INSTALLED_PROVIDERS", "memory");
    std::env::set_var("PROVIDER", "memory");
    std::env::set_var("DATASOURCE_URL", "memory://");
    assert_eq!(default().unwrap().provider(), "memory");
}

#[test]
fn missing_package_or_unknown_name() {
    let _isolated = Isolated::enter();
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
    let _isolated = Isolated::enter();
    assert_eq!(
        defaults(opts(&[("DATASOURCE_URL", "not-a-url")]))
            .unwrap_err()
            .to_string(),
        "DATASOURCE_URL has no provider"
    );
}

#[tokio::test]
async fn loaded_memory_opens() {
    let _isolated = Isolated::enter();
    bind_memory();
    std::env::set_var("INSTALLED_PROVIDERS", "memory");
    std::env::set_var("PROVIDER", "memory");
    std::env::set_var("DATASOURCE_URL", "memory://");
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    assert!(!connection.is_open());
    connection.open().await.unwrap();
    assert!(connection.is_open());
}
