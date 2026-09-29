use crate::{default, register, reset, Datasource, MockDriver};
use std::sync::Arc;

fn arrange() {
    reset();
    for key in ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"] {
        std::env::remove_var(key);
    }
    register("sqlite", MockDriver::arc());
    std::env::set_var("PROVIDER", "sqlite");
}

#[test]
fn create_default_from_env_settings() {
    let _guard = super::env_lock();
    arrange();
    let ds = default().expect("default");
    let command = ds.create_command(None);
    assert_eq!(Datasource::provider(&*ds), "sqlite");
    assert_eq!(Arc::strong_count(&command) > 0, true);
}
