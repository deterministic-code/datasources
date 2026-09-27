use chrono::{TimeZone, Utc};
use datasources::{
    create_repository_options, eq, gt, BuiltDatasource, Connection, ConvertSchema, Datasource,
    DictionaryMapper, PluralizerMapper, Value, Where,
};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

const ADA: &str = "11111111-1111-1111-1111-111111111111";
const GRACE: &str = "22222222-2222-2222-2222-222222222222";
const LIN: &str = "33333333-3333-3333-3333-333333333333";

struct User;

impl ConvertSchema for User {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "INTEGER"),
            ("username", "TEXT"),
            ("is_enabled", "INTEGER"),
        ]
    }
}

struct UserAccount;

impl ConvertSchema for UserAccount {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Key", "TEXT"),
            ("Login", "TEXT"),
            ("Active", "INTEGER"),
            ("Opened", "TEXT"),
        ]
    }
}

fn datasource() -> Arc<BuiltDatasource> {
    Arc::new(BuiltDatasource::new(
        "sqlite",
        Arc::new(crate::SqliteDriver),
        Some(HashMap::from([("filename".into(), ":memory:".into())])),
    ))
}

#[tokio::test]
async fn reads_users_by_id_and_by_username_and_enabled() {
    let built = datasource();
    let ds: &dyn Datasource = &*built;
    let connection = open(
        ds,
        "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            is_enabled INTEGER NOT NULL
        );
        INSERT INTO users (id, username, is_enabled) VALUES
            (1, 'ada', 1),
            (2, 'grace', 0);
        ",
    )
    .await;
    let mut options = create_repository_options::<User>();
    options.entity_mapper = Box::new(PluralizerMapper::new());
    let repo = ds.create_readonly_repository::<User>(Some(connection), Some(options.read));
    let by_id = repo
        .read_single(Some(&Where::binary("id", eq, 1_i64)), None)
        .await
        .unwrap()
        .unwrap();
    let disabled = repo
        .read(
            Some(&Where::binary("username", eq, "grace").and(
                "is_enabled",
                eq,
                0_i64,
            )),
            None,
        )
        .await
        .unwrap();
    assert_eq!(by_id.get("username"), Some(&Value::from("ada")));
    assert_eq!(by_id.get("is_enabled"), Some(&Value::Int(1)));
    assert_eq!(disabled.len(), 1);
    assert_eq!(disabled[0].get("id"), Some(&Value::Int(2)));
    assert_eq!(disabled[0].get("is_enabled"), Some(&Value::Int(0)));
}

#[tokio::test]
async fn reads_user_by_guid_username_and_created() {
    let built = datasource();
    let ds: &dyn Datasource = &*built;
    let connection = open(
        ds,
        &format!(
            "
        CREATE TABLE user (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            is_enabled INTEGER NOT NULL,
            created TEXT NOT NULL
        );
        INSERT INTO user (id, username, is_enabled, created) VALUES
            ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
            ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
            ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z');
        "
        ),
    )
    .await;
    let mut options = create_repository_options::<UserAccount>();
    options.entity = "user".into();
    options.column_mappings = Box::new(DictionaryMapper::new([
        ("Key".to_string(), "id".to_string()),
        ("Login".to_string(), "username".to_string()),
        ("Active".to_string(), "is_enabled".to_string()),
        ("Opened".to_string(), "created".to_string()),
    ]));
    let repo = ds.create_readonly_repository::<UserAccount>(Some(connection), Some(options.read));
    let opened = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let by_id = repo
        .read_single(
            Some(&Where::binary(
                "Key",
                eq,
                Uuid::parse_str(ADA).unwrap(),
            )),
            None,
        )
        .await
        .unwrap()
        .unwrap();
    let by_login = repo
        .read(
            Some(&Where::binary("Login", eq, "ada").and("Active", eq, true)),
            None,
        )
        .await
        .unwrap();
    let later = repo
        .read(
            Some(&Where::binary("Active", eq, true).and("Opened", gt, opened)),
            None,
        )
        .await
        .unwrap();
    assert_eq!(by_id.get("Login"), Some(&Value::from("ada")));
    assert_eq!(by_id.get("Key"), Some(&Value::from(ADA)));
    assert_eq!(by_id.get("Active"), Some(&Value::Int(1)));
    assert_eq!(
        by_id.get("Opened"),
        Some(&Value::from("2020-01-01T00:00:00.000Z"))
    );
    assert_eq!(by_login.len(), 1);
    assert_eq!(by_login[0].get("Login"), Some(&Value::from("ada")));
    assert_eq!(later.len(), 1);
    assert_eq!(later[0].get("Login"), Some(&Value::from("grace")));
}

async fn open(ds: &dyn Datasource, sql: &str) -> Arc<dyn Connection> {
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    let command = ds.create_command(Some(connection.clone()));
    for statement in sql
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        command.execute(statement, None).await.unwrap();
    }
    connection
}

mod user_view {
    use super::{datasource, open, ADA, GRACE, LIN};
    use datasources::{
        create_repository_options, eq, Casing, ChangeCasingMapper, ConvertSchema, Datasource,
        DictionaryMapper, IReadonlyRepository, ReadonlyRepository, Value, Where,
    };
    use uuid::Uuid;

    struct UserAccount;

    impl ConvertSchema for UserAccount {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![
                ("Key", "TEXT"),
                ("Login", "TEXT"),
                ("Active", "INTEGER"),
                ("Opened", "TEXT"),
                ("UserTypeId", "INTEGER"),
            ]
        }
    }

    struct UserType;

    impl ConvertSchema for UserType {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![("Id", "INTEGER"), ("Name", "TEXT")]
        }
    }

    #[tokio::test]
    async fn fills_user_type_on_user_view() {
        let built = datasource();
        let ds: &dyn Datasource = &*built;
        let connection = open(
            ds,
            &format!(
                "
            CREATE TABLE user_type (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            INSERT INTO user_type (id, name) VALUES
                (1, 'admin'),
                (2, 'user'),
                (3, 'service');
            CREATE TABLE user (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                created TEXT NOT NULL,
                user_type_id INTEGER NOT NULL
            );
            INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
                ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
                ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
                ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
            "
            ),
        )
        .await;
        let mut users_options = create_repository_options::<UserAccount>();
        users_options.entity = "user".into();
        users_options.column_mappings = Box::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
            ("Active".to_string(), "is_enabled".to_string()),
            ("Opened".to_string(), "created".to_string()),
            ("UserTypeId".to_string(), "user_type_id".to_string()),
        ]));
        let mut type_options = create_repository_options::<UserType>();
        type_options.entity = "UserType".into();
        type_options.entity_mapper =
            Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
        type_options.column_mappings =
            Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
        let mut users_repo = ReadonlyRepository::new(
            connection.clone(),
            users_options.read,
        )
        .unwrap();
        let types =
            ds.create_readonly_repository::<UserType>(Some(connection), Some(type_options.read));
        users_repo.add_enriched(datasources::Enrichment::new("UserType", "UserTypeId", "Id", Some("Name".into()), false), types, None);
        let account = users_repo
            .read_single(
                Some(&Where::binary(
                    "Key",
                    eq,
                    Uuid::parse_str(ADA).unwrap(),
                )),
                None,
            )
            .await
            .unwrap()
            .unwrap();
        let mut users_view = account.clone();
        users_view.remove("UserTypeId");
        assert_eq!(account.get("UserTypeId"), Some(&Value::Int(1)));
        assert_eq!(users_view.get("UserType"), Some(&Value::from("admin")));
        assert_eq!(users_view.get("Login"), Some(&Value::from("ada")));
        assert_eq!(users_view.get("Key"), Some(&Value::from(ADA)));
        assert_eq!(users_view.get("Active"), Some(&Value::Int(1)));
        assert_eq!(
            users_view.get("Opened"),
            Some(&Value::from("2020-01-01T00:00:00.000Z"))
        );
        assert!(users_view.get("UserTypeId").is_none());
    }
}

mod users_view_roles {
    use super::{datasource, open, ADA, GRACE, LIN};
    use datasources::{
        create_repository_options, eq, Casing, ChangeCasingMapper, ConvertSchema, Datasource,
        DictionaryMapper, HashsetOptimizedReadonlyEnrichmentStrategy, IReadonlyRepository, Join,
        NestedToObjectEnrichmentStrategy, ReadonlyRepository, ReadonlyRepositoryOptions,
        SelectStatement, SelectStatementOptions, TransformMapper, TypeMap, TypeMapField,
        TypeMapGroup, Value, Where,
    };
    use std::sync::Arc;

    struct UserType;

    impl ConvertSchema for UserType {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![("Id", "INTEGER"), ("Name", "TEXT")]
        }
    }

    struct UserRole;

    impl ConvertSchema for UserRole {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![("UserId", "TEXT"), ("Name", "TEXT")]
        }
    }

    struct UsersView;

    impl ConvertSchema for UsersView {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![
                ("Key", "TEXT"),
                ("Login", "TEXT"),
                ("Active", "INTEGER"),
                ("Opened", "TEXT"),
                ("UserType", "TEXT"),
            ]
        }
    }

    fn seed() -> String {
        format!(
            "
            CREATE TABLE user_type (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            INSERT INTO user_type (id, name) VALUES
                (1, 'admin'),
                (2, 'user'),
                (3, 'service');
            CREATE TABLE user (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                created TEXT NOT NULL,
                user_type_id INTEGER NOT NULL
            );
            INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
                ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
                ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
                ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
            CREATE TABLE roles (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            INSERT INTO roles (id, name) VALUES
                (1, 'admin'),
                (2, 'editor'),
                (3, 'viewer');
            CREATE TABLE user_roles (
                user_id TEXT NOT NULL,
                role_id INTEGER NOT NULL
            );
            INSERT INTO user_roles (user_id, role_id) VALUES
                ('{ADA}', 1),
                ('{ADA}', 2),
                ('{GRACE}', 3);
            "
        )
    }

    fn roles_sql(sql: String) -> String {
        let select = "SELECT user_roles.user_id AS user_id, roles.name AS name FROM user_roles INNER JOIN roles ON roles.id = user_roles.role_id";
        match sql.split_once(" WHERE ") {
            Some((_, rest)) => format!("{select} WHERE {rest}"),
            None => select.to_string(),
        }
    }

    fn role_names(value: &Value) -> Vec<String> {
        value
            .wire_text()
            .split('|')
            .filter(|part| !part.is_empty())
            .filter_map(|part| {
                part.split(',')
                    .find_map(|field| field.strip_prefix("Name="))
                    .map(str::to_string)
            })
            .collect()
    }

    #[tokio::test]
    async fn fills_roles_with_a_hashset() {
        let built = datasource();
        let ds: &dyn Datasource = &*built;
        let connection = open(ds, &seed()).await;
        let mut users_options = create_repository_options::<UsersView>();
        users_options.entity_mapper = Box::new(DictionaryMapper::new([(
            "UsersView".to_string(),
            "user".to_string(),
        )]));
        users_options.column_mappings = Box::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
            ("Active".to_string(), "is_enabled".to_string()),
            ("Opened".to_string(), "created".to_string()),
            ("UserType".to_string(), "user_type_id".to_string()),
        ]));
        let mut type_options = create_repository_options::<UserType>();
        type_options.entity_mapper =
            Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
        type_options.column_mappings =
            Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
        let mut role_options = create_repository_options::<UserRole>();
        role_options.entity = "user_roles".into();
        role_options.column_mappings = Box::new(DictionaryMapper::new([
            ("UserId".to_string(), "user_id".to_string()),
            ("Name".to_string(), "name".to_string()),
        ]));
        role_options.query = Some(Arc::new(TransformMapper::new(roles_sql, |sql| sql)));
        let mut users_repo = ReadonlyRepository::new(
            connection.clone(),
            users_options.read,
        )
        .unwrap();
        let types = ds.create_readonly_repository::<UserType>(
            Some(connection.clone()),
            Some(type_options.read),
        );
        let roles =
            ds.create_readonly_repository::<UserRole>(Some(connection), Some(role_options.read));
        let strategy = Arc::new(HashsetOptimizedReadonlyEnrichmentStrategy);
        users_repo.add_enriched(datasources::Enrichment::new("UserType", "UserType", "Id", Some("Name".into()), false), types, Some(strategy.clone()));
        users_repo.add_enriched(datasources::Enrichment::new("Roles", "Key", "UserId", None, true), roles, Some(strategy));
        let users_view = users_repo
            .read_single(Some(&Where::binary("Key", eq, ADA)), None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(users_view.get("UserType"), Some(&Value::from("admin")));
        let mut names = role_names(users_view.get("Roles").unwrap());
        names.sort();
        assert_eq!(names, ["admin", "editor"]);
        assert!(users_view
            .get("Roles")
            .unwrap()
            .wire_text()
            .contains(&format!("UserId={ADA}")));
    }

    #[tokio::test]
    async fn fills_roles_from_a_join() {
        let built = datasource();
        let ds: &dyn Datasource = &*built;
        let connection = open(ds, &seed()).await;
        let command = Arc::new(SelectStatement::new(SelectStatementOptions {
            joins: &[
                Join::left("user_type", "user_type.id", "user.user_type_id"),
                Join::left("user_roles", "user_roles.user_id", "user.id"),
                Join::left("roles", "roles.id", "user_roles.role_id"),
            ],
            ..SelectStatementOptions::new(
                "user",
                &[
                    "user.id AS Key",
                    "user.username AS Login",
                    "user.is_enabled AS Active",
                    "user.created AS Opened",
                    "user_type.name AS UserType",
                    "user.id AS UserId",
                    "user_roles.role_id AS RoleId",
                    "roles.name AS RoleName",
                ],
            )
        }));
        let options = ReadonlyRepositoryOptions::default();
        let mut users_repo = ReadonlyRepository::new(connection.clone(), options).unwrap();
        users_repo.add_enriched(
            datasources::Enrichment::new("Roles", "Key", "UserId", None, true),
            Arc::new(ReadonlyRepository::memory(vec![])),
            Some(Arc::new(NestedToObjectEnrichmentStrategy::new(
                TypeMap::new(
                    "Key",
                    vec![
                        ("Key".into(), TypeMapField::column("Key")),
                        ("Login".into(), TypeMapField::column("Login")),
                        ("Active".into(), TypeMapField::column("Active")),
                        ("Opened".into(), TypeMapField::column("Opened")),
                        ("UserType".into(), TypeMapField::column("UserType")),
                        (
                            "Roles".into(),
                            TypeMapField::Group(TypeMapGroup::new(
                                "RoleId",
                                vec![
                                    ("RoleId".into(), TypeMapField::column("RoleId")),
                                    ("UserId".into(), TypeMapField::column("UserId")),
                                    ("Name".into(), TypeMapField::column("RoleName")),
                                ],
                            )),
                        ),
                    ],
                ),
            ))),
        );
        let items = users_repo.read(Some(command.as_ref()), None).await.unwrap();
        let view = |login: &str| {
            items
                .iter()
                .find(|item| item.get("Login") == Some(&Value::from(login)))
                .unwrap()
                .clone()
        };
        let ada = view("ada");
        let grace = view("grace");
        let lin = view("lin");
        assert_eq!(items.len(), 3);
        assert_eq!(ada.get("UserType"), Some(&Value::from("admin")));
        let mut names = role_names(ada.get("Roles").unwrap());
        names.sort();
        assert_eq!(names, ["admin", "editor"]);
        assert!(ada
            .get("Roles")
            .unwrap()
            .wire_text()
            .contains(&format!("UserId={ADA}")));
        assert_eq!(role_names(grace.get("Roles").unwrap()), ["viewer"]);
        assert_eq!(lin.get("Roles"), Some(&Value::from("")));
    }
}

mod user_crud {
    use super::{datasource, open, User};
    use datasources::{
        create_repository_options, encode_rows, eq, CRUDRepository, Casing, ChangeCasingMapper,
        ConvertSchema, Datasource, DictionaryMapper, ICRUDRepository, IReadonlyRepository,
        RowTypeConverter, Value, Where,
    };
    use std::sync::Arc;

    struct UserType;

    impl ConvertSchema for UserType {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![("Id", "INTEGER"), ("Name", "TEXT")]
        }
    }

    struct UserRole;

    impl ConvertSchema for UserRole {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![("UserId", "INTEGER"), ("Name", "TEXT")]
        }
    }

    struct UsersView;

    impl ConvertSchema for UsersView {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![
                ("Key", "INTEGER"),
                ("Login", "TEXT"),
                ("Active", "INTEGER"),
                ("Opened", "TEXT"),
                ("UserTypeId", "INTEGER"),
                ("Version", "INTEGER"),
            ]
        }
    }

    fn role_names(value: &Value) -> Vec<String> {
        value
            .wire_text()
            .split('|')
            .filter(|part| !part.is_empty())
            .filter_map(|part| {
                part.split(',')
                    .find_map(|field| field.strip_prefix("Name="))
                    .map(str::to_string)
            })
            .collect()
    }

    #[tokio::test]
    async fn writes_user_from_the_type() {
        let built = datasource();
        let ds: &dyn Datasource = &*built;
        let connection = open(
            ds,
            "
            CREATE TABLE User (
                id INTEGER PRIMARY KEY,
                is_enabled INTEGER NOT NULL,
                username TEXT NOT NULL
            );
            ",
        )
        .await;
        let users_repo = CRUDRepository::from_type::<User>(connection).unwrap();
        let mut created = datasources::Row::new();
        created.insert("username".into(), Value::from("ada"));
        created.insert("is_enabled".into(), Value::Bool(true));
        let mut created = users_repo.create(&mut created).await.unwrap();
        assert_eq!(created.get("id"), Some(&Value::Int(1)));
        let by_id = users_repo
            .read_single_by_id(created.get("id").unwrap())
            .await
            .unwrap()
            .unwrap();
        let matches = users_repo
            .read(
                Some(&Where::binary("username", eq, "ada").and(
                    "is_enabled",
                    eq,
                    Value::Bool(true),
                )),
                None,
            )
            .await
            .unwrap();
        created.insert("username".into(), Value::from("ada-v2"));
        let updated = users_repo.update(&created, None).await.unwrap();
        assert_eq!(updated.rows_affected, 1);
        let renamed = users_repo
            .read_single_by_id(created.get("id").unwrap())
            .await
            .unwrap()
            .unwrap();
        let removed = users_repo.delete(&created, None).await.unwrap();
        assert_eq!(removed.rows_affected, 1);
        let gone = users_repo
            .read_single_by_id(created.get("id").unwrap())
            .await
            .unwrap();
        assert_eq!(created.get("id"), Some(&Value::Int(1)));
        assert_eq!(by_id.get("id"), Some(&Value::Int(1)));
        assert_eq!(by_id.get("username"), Some(&Value::from("ada")));
        assert_eq!(by_id.get("is_enabled"), Some(&Value::Int(1)));
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].get("username"), Some(&Value::from("ada")));
        assert_eq!(renamed.get("username"), Some(&Value::from("ada-v2")));
        assert!(gone.is_none());
    }

    #[tokio::test]
    async fn updates_users_view_when_the_version_matches() {
        let built = datasource();
        let ds: &dyn Datasource = &*built;
        let connection = open(
            ds,
            "
            CREATE TABLE user_type (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE user (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                created TEXT NOT NULL,
                user_type_id INTEGER NOT NULL,
                version INTEGER NOT NULL
            );
            CREATE TABLE user_role (
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL
            );
            ",
        )
        .await;
        let mut users_options = create_repository_options::<UsersView>();
        users_options.ids = vec!["Key".into()];
        users_options.entity_mapper = Box::new(DictionaryMapper::new([(
            "UsersView".to_string(),
            "user".to_string(),
        )]));
        users_options.column_mappings = Box::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
            ("Active".to_string(), "is_enabled".to_string()),
            ("Opened".to_string(), "created".to_string()),
            ("UserTypeId".to_string(), "user_type_id".to_string()),
            ("Version".to_string(), "version".to_string()),
        ]));
        users_options.row_converter = Some(RowTypeConverter::of::<UsersView>());
        let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
        let mut type_options = create_repository_options::<UserType>();
        type_options.entity_mapper =
            Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
        type_options.column_mappings = Box::new(casing);
        type_options.row_converter = Some(RowTypeConverter::of::<UserType>());
        let mut role_options = create_repository_options::<UserRole>();
        role_options.entity_mapper =
            Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
        role_options.column_mappings =
            Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
        role_options.row_converter = Some(RowTypeConverter::of::<UserRole>());
        let mut users_repo = CRUDRepository::new(connection.clone(), users_options).unwrap();
        let user_types_repo =
            Arc::new(CRUDRepository::new(connection.clone(), type_options).unwrap());
        let user_roles_repo = Arc::new(CRUDRepository::new(connection, role_options).unwrap());
        users_repo.add_enriched(datasources::Enrichment::new("UserType", "UserTypeId", "Id", Some("Name".into()), false), user_types_repo.clone(), None);
        users_repo.add_enriched(datasources::Enrichment::new("Roles", "Key", "UserId", None, true), user_roles_repo.clone(), None);
        let mut admin = datasources::Row::new();
        admin.insert("Name".into(), Value::from("admin"));
        let admin = user_types_repo.create(&mut admin).await.unwrap();
        assert_eq!(admin.get("Id"), Some(&Value::Int(1)));
        let mut admin_role = datasources::Row::new();
        admin_role.insert("Name".into(), Value::from("admin"));
        let mut editor_role = datasources::Row::new();
        editor_role.insert("Name".into(), Value::from("editor"));
        let mut user = datasources::Row::new();
        user.insert("Login".into(), Value::from("ada"));
        user.insert("Active".into(), Value::Bool(true));
        user.insert("Opened".into(), Value::from("2020-01-01T00:00:00.000Z"));
        user.insert("UserTypeId".into(), admin.get("Id").cloned().unwrap());
        user.insert("Version".into(), Value::Int(1));
        user.insert("Roles".into(), encode_rows(&[admin_role, editor_role]));
        let user = users_repo.create(&mut user).await.unwrap();
        assert_eq!(user.get("Key"), Some(&Value::Int(1)));
        let key = user.get("Key").cloned().unwrap();
        let mut loaded = users_repo
            .read_single(Some(&Where::binary("Key", eq, key.clone())), None)
            .await
            .unwrap()
            .unwrap();
        let version = loaded.get("Version").cloned().unwrap();
        loaded.insert("Login".into(), Value::from("ada-v2"));
        loaded.insert("Version".into(), Value::Int(2));
        let updated = users_repo
            .update(
                &loaded,
                Some(&Where::binary("Key", eq, key.clone()).and("Version", eq, version.clone())),
            )
            .await
            .unwrap();
        let current = users_repo
            .read_single(Some(&Where::binary("Key", eq, key.clone())), None)
            .await
            .unwrap()
            .unwrap();
        let stale = users_repo
            .update(
                &loaded,
                Some(&Where::binary("Key", eq, key.clone()).and("Version", eq, version)),
            )
            .await
            .unwrap();
        let removed = users_repo.delete(&loaded, None).await.unwrap();
        assert_eq!(removed.rows_affected, 1);
        let gone = users_repo.read_single_by_id(&key).await.unwrap();
        let roles = user_roles_repo
            .read(Some(&Where::binary("UserId", eq, key.clone())), None)
            .await
            .unwrap();
        assert_eq!(admin.get("Id"), Some(&Value::Int(1)));
        assert_eq!(user.get("Key"), Some(&Value::Int(1)));
        assert_eq!(loaded.get("UserType"), Some(&Value::from("admin")));
        let mut names = role_names(loaded.get("Roles").unwrap());
        names.sort();
        assert_eq!(names, ["admin", "editor"]);
        assert_eq!(updated.rows_affected, 1);
        assert_eq!(current.get("Version"), Some(&Value::Int(2)));
        assert_eq!(current.get("Login"), Some(&Value::from("ada-v2")));
        assert_eq!(stale.rows_affected, 0);
        assert!(gone.is_none());
        assert!(roles.is_empty());
    }
}
