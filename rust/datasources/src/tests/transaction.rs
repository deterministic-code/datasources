use crate::{default, register, reset, IsolationLevel, MockDriver};

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
async fn start_from_default() {
    clear_env();
    let ds = default().expect("default");
    let connection = ds.create_connection().await.expect("connection");
    connection.open().await.expect("open");
    let txn = connection.start_transaction(None).await.expect("txn");
    assert_eq!(txn.isolation(), IsolationLevel::Serializable);
}
