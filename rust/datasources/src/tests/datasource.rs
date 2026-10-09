use crate::{
    as_readonly, default, register, reset, BuiltDatasource, Datasource, ICRUDRepository, MockDriver,
    MockRepository, Response, Row, StubRepository, Value,
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

#[tokio::test]
async fn creates_from_plural_name() {
    let built = BuiltDatasource::new("mock", MockDriver::arc(), None);
    let registered = Arc::new(MockRepository::new());
    built.repositories().register_named(
        "Contact",
        "contacts",
        {
            let registered = Arc::clone(&registered);
            move |_| Arc::clone(&registered) as Arc<dyn ICRUDRepository>
        },
        |_| as_readonly(Arc::new(StubRepository::typed::<Item>("read"))),
    );
    let ds: &dyn Datasource = &built;
    let contacts = ds.create_repository_by_name("contacts", None).unwrap();
    let mut row = Row::new();
    row.insert("name".into(), Value::from("Ada"));
    let created: Row = contacts.create(&mut row).await.unwrap();
    let loaded: Option<Row> = contacts.read_single(None, None, None).await.unwrap();
    let updated: Response = contacts.update(&created, None).await.unwrap();
    let deleted: Response = contacts.delete(&created, None).await.unwrap();
    assert_eq!(created.get("name").map(Value::wire_text).as_deref(), Some("Ada"));
    assert_eq!(
        loaded.as_ref().and_then(|row| row.get("name")).map(Value::wire_text).as_deref(),
        Some("Ada")
    );
    assert_eq!(updated.rows_affected, 1);
    assert_eq!(deleted.rows_affected, 1);
    let readonly = ds.create_readonly_repository_by_name("contacts", None).unwrap();
    assert_eq!(readonly.entity_type(), TypeId::of::<Item>());
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
