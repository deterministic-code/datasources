use futures_util::TryStreamExt;
use datasources::{
    create_crud_repository_options, create_readonly_repository_mapping_options, default, eq,
    Casing, ChangeCasingMapper, ConvertSchema, Datasource, ICRUDRepository, Response, Row, Value,
    Where,
};
use datasources_tests::apply_schema;
use std::sync::Arc;

struct User;

impl ConvertSchema for User {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Id", "INTEGER"),
            ("Username", "TEXT"),
            ("IsEnabled", "INTEGER"),
        ]
    }
}

trait Repositories {
    fn users(&self) -> Arc<dyn ICRUDRepository>;
}

impl Repositories for dyn Datasource {
    fn users(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("users", None).expect("users")
    }
}

async fn open(ds: &dyn Datasource) -> Arc<dyn datasources::Connection> {
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    connection
}

fn register(ds: &Arc<dyn Datasource>) {
    let crud = Arc::clone(ds);
    let read = Arc::clone(ds);
    ds.repositories().register_named(
        "User",
        "users",
        move |connection| {
            let mut options = create_crud_repository_options::<User>();
            options.property_mappings =
                Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
            options.ids = vec!["Id".into()];
            crud.create_repository::<User>(Some(connection), Some(options))
        },
        move |connection| {
            let mut options = create_readonly_repository_mapping_options::<User>();
            options.property_mappings =
                Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
            read.create_readonly_repository::<User>(Some(connection), Some(options))
        },
    );
}

#[tokio::test]
async fn creates_using_dynamic_after_register() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = open(ds.as_ref()).await;
    apply_schema(connection.as_ref(), "user-pascal")
        .await
        .unwrap();
    register(&ds);
    let users = ds.as_ref().users();
    let mut created = Row::new();
    created.insert("Username".into(), Value::from("ada"));
    created.insert("IsEnabled".into(), Value::Bool(true));
    let created = users.create(created).await.unwrap();
    let loaded: Option<Row> = users.read_single(None, None, None).await.unwrap();
    let updated: Response = users.update(created.clone(), None).await.unwrap();
    let deleted: Response = users.delete(created.clone(), None).await.unwrap();
    assert_eq!(
        matches!(created.get("Id"), Some(Value::Int(id)) if *id > 0),
        true
    );
    assert_eq!(
        loaded.as_ref().and_then(|row| row.get("Username")),
        Some(&Value::from("ada"))
    );
    assert_eq!(updated.rows_affected, 1);
    assert_eq!(deleted.rows_affected, 1);
}

#[tokio::test]
async fn writes_user_through_plural_name() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = open(ds.as_ref()).await;
    apply_schema(connection.as_ref(), "user-pascal")
        .await
        .unwrap();
    register(&ds);
    let users_repo = ds.as_ref().users();
    let mut created = datasources::Row::new();
    created.insert("Username".into(), Value::from("ada"));
    created.insert("IsEnabled".into(), Value::Bool(true));
    let mut created = users_repo.create(created).await.unwrap();
    let by_id = users_repo
        .read_single_by_id(created.get("Id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let matches = users_repo
        .read(
            Some(&Where::binary("Username", eq, "ada").and("IsEnabled", eq, Value::Bool(true))),
            None,
            None,
        )
        .try_collect::<Vec<_>>().await
        .unwrap();
    created.insert("Username".into(), Value::from("ada-v2"));
    let updated = users_repo.update(created.clone(), None).await.unwrap();
    let renamed = users_repo
        .read_single_by_id(created.get("Id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let removed = users_repo.delete(created.clone(), None).await.unwrap();
    let gone = users_repo
        .read_single_by_id(created.get("Id").unwrap())
        .await
        .unwrap();
    assert_eq!(
        matches!(created.get("Id"), Some(Value::Int(id)) if *id > 0),
        true
    );
    assert_eq!(updated.rows_affected, 1);
    assert_eq!(removed.rows_affected, 1);
    assert_eq!(by_id.get("Id"), created.get("Id"));
    assert_eq!(by_id.get("Username"), Some(&Value::from("ada")));
    assert_eq!(by_id.get("IsEnabled"), Some(&Value::Bool(true)));
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].get("Username"), Some(&Value::from("ada")));
    assert_eq!(renamed.get("Username"), Some(&Value::from("ada-v2")));
    assert!(gone.is_none());
}

#[tokio::test]
async fn reads_through_access_create_readonly() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = open(ds.as_ref()).await;
    apply_schema(connection.as_ref(), "user-pascal")
        .await
        .unwrap();
    register(&ds);
    let mut created = datasources::Row::new();
    created.insert("Username".into(), Value::from("ada"));
    created.insert("IsEnabled".into(), Value::Bool(true));
    let created = ds.as_ref().users().create(created).await.unwrap();
    let users_repo = ds
        .as_ref()
        .access("users", None)
        .create_readonly()
        .unwrap();
    let loaded = users_repo
        .read_single(
            Some(&Where::binary("Id", eq, created.get("Id").unwrap().clone())),
            None,
            None,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(loaded.get("Username"), Some(&Value::from("ada")));
    assert_eq!(loaded.get("IsEnabled"), Some(&Value::Bool(true)));
    assert_eq!(loaded.get("Id"), created.get("Id"));
}
