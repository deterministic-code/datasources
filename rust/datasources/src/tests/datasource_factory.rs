use crate::{create, default, register, reset, Datasource, DatasourceError, MockDriver};
use std::collections::HashMap;

fn clear_env() {
    reset();
    for key in [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "DATABASE_SCHEMA",
        "DEFAULT_PROVIDER",
        "DEFAULT_DATASOURCE_URL",
        "PROVIDER",
    ] {
        std::env::remove_var(key);
    }
}

#[test]
fn default_from_environment() {
    clear_env();
    register("sqlite", MockDriver::arc());
    std::env::set_var("PROVIDER", "sqlite");
    std::env::set_var("DATASOURCE_URL", "sqlite:///:memory:");
    let ds = default().expect("default");
    assert_eq!(Datasource::provider(&*ds), "sqlite");
}

#[test]
fn missing_provider_throws() {
    reset();
    assert!(matches!(
        create(Some(&HashMap::from([("provider".into(), "missing".into())]))),
        Err(DatasourceError::ProviderNotInstalled(name)) if name == "missing"
    ));
}
