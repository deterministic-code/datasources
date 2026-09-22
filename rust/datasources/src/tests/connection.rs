use crate::{default, register, reset, MockDriver};

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
    register("sqlite", MockDriver::arc());
    std::env::set_var("PROVIDER", "sqlite");
    std::env::set_var("DATASOURCE_URL", "sqlite:///:memory:");
}

#[tokio::test]
async fn create_from_default() {
    clear_env();
    let ds = default().expect("default");
    let connection = ds.create_connection().await.expect("connection");
    assert_eq!(connection.is_open(), false);
}
