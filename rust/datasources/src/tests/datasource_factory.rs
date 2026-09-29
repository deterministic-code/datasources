use crate::{create, default, register, reset, Datasource, DatasourceError, MockDriver};
use std::collections::HashMap;

fn clear_env() {
    reset();
    for key in [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "PROVIDER",
    ] {
        std::env::remove_var(key);
    }
}

#[test]
fn default_from_environment() {
    let _guard = super::env_lock();
    clear_env();
    register("sqlite", MockDriver::arc());
    std::env::set_var("PROVIDER", "sqlite");
    let ds = default().expect("default");
    assert_eq!(Datasource::provider(&*ds), "sqlite");
}

#[test]
fn missing_provider_throws() {
    let _guard = super::env_lock();
    reset();
    assert!(matches!(
        create(Some(&HashMap::from([("PROVIDER".into(), "missing".into())]))),
        Err(DatasourceError::ProviderNotInstalled(name)) if name == "missing"
    ));
}
