use crate::{
    as_readonly, default, register, reset, BuiltDatasource, Datasource, MockDriver, StubRepository,
};
use std::any::TypeId;
use std::sync::Arc;

fn arrange() {
    reset();
    for key in ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"] {
        std::env::remove_var(key);
    }
    register("sqlite", MockDriver::arc());
    std::env::set_var("PROVIDER", "sqlite");
}

struct Item;

#[test]
fn creates_from_plural_name() {
    let built = BuiltDatasource::new("mock", MockDriver::arc(), None);
    built.repositories().register_named(
        "Contact",
        "contacts",
        |_| Arc::new(StubRepository::typed::<Item>("crud")),
        |_| as_readonly(Arc::new(StubRepository::typed::<Item>("read"))),
    );
    let ds: &dyn Datasource = &built;
    let created = ds.create_repository_by_name("Contact", None).unwrap();
    assert_eq!(created.entity_type(), TypeId::of::<Item>());
    let readonly = ds
        .create_readonly_repository_by_name("contacts", None)
        .unwrap();
    assert_eq!(readonly.entity_type(), TypeId::of::<Item>());
    let ds_dynamic = ds.access("contacts", None).create().unwrap();
    assert_eq!(ds_dynamic.entity_type(), TypeId::of::<Item>());
    match ds.create_repository_by_name("Missing", None) {
        Ok(_) => panic!("expected missing name"),
        Err(err) => assert_eq!(err.to_string(), "Repository \"Missing\" is not registered"),
    }
}

#[tokio::test]
async fn create_default_from_env_settings() {
    let _guard = super::env_lock();
    arrange();
    let ds = default().expect("default");
    let command = ds.create_command(None).await.unwrap();
    assert_eq!(Datasource::provider(&*ds), "sqlite");
    assert_eq!(Arc::strong_count(&command) > 0, true);
}
