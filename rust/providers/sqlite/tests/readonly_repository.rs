use chrono::{TimeZone, Utc};
use datasources::{
    create_repository_options, eq, gt, BuiltDatasource, Connection, ConvertSchema, Datasource,
    DictionaryMapper, PluralizerMapper, Value, WhereClause,
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
        .read_single(Some(&WhereClause::binary("id", eq, 1_i64)))
        .await
        .unwrap()
        .unwrap();
    let disabled = repo
        .read(Some(&WhereClause::binary("username", eq, "grace").and(
            "is_enabled",
            eq,
            0_i64,
        )))
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
        .read_single(Some(&WhereClause::binary(
            "Key",
            eq,
            Uuid::parse_str(ADA).unwrap(),
        )))
        .await
        .unwrap()
        .unwrap();
    let by_login = repo
        .read(Some(
            &WhereClause::binary("Login", eq, "ada").and("Active", eq, true),
        ))
        .await
        .unwrap();
    let later = repo
        .read(Some(
            &WhereClause::binary("Active", eq, true).and("Opened", gt, opened),
        ))
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
        DictionaryMapper, EnrichedReadonlyRepository, IReadonlyRepository, Value, WhereClause,
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
        let users = ds.create_readonly_repository::<UserAccount>(
            Some(connection.clone()),
            Some(users_options.read),
        );
        let types =
            ds.create_readonly_repository::<UserType>(Some(connection), Some(type_options.read));
        let mut enriched = EnrichedReadonlyRepository::new(users);
        enriched.add_enriched(
            "UserType",
            "UserTypeId",
            types,
            "Id",
            Some("Name".into()),
            false,
            (),
        );
        let account = enriched
            .read_single(Some(&WhereClause::binary(
                "Key",
                eq,
                Uuid::parse_str(ADA).unwrap(),
            )))
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
        create_repository_options, encode_rows, eq, Casing, ChangeCasingMapper, ConvertSchema,
        Datasource, DictionaryMapper, EnrichedReadonlyRepository, FlatToNestedMapper,
        FlatToNestedReadonlyEnrichmentStrategy, HashsetOptimizedReadonlyEnrichmentStrategy, Join,
        MappedValue, Query, QueryStatement, TransformMapper, TypeMap, TypeMapField, TypeMapGroup,
        Value, WhereClause,
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

    struct JoinRow;

    impl ConvertSchema for JoinRow {
        fn schema() -> Vec<(&'static str, &'static str)> {
            Vec::new()
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
        let users = ds.create_readonly_repository::<UsersView>(
            Some(connection.clone()),
            Some(users_options.read),
        );
        let types =
            ds.create_readonly_repository::<UserType>(Some(connection.clone()), Some(type_options.read));
        let roles =
            ds.create_readonly_repository::<UserRole>(Some(connection), Some(role_options.read));
        let mut users_repo = EnrichedReadonlyRepository::with_strategy(
            users,
            Arc::new(HashsetOptimizedReadonlyEnrichmentStrategy),
        );
        users_repo.add_enriched(
            "UserType",
            "UserType",
            types,
            "Id",
            Some("Name".into()),
            false,
            (),
        );
        users_repo.add_enriched("Roles", "Key", roles, "UserId", None, true, ());
        let users_view = users_repo
            .read_single(Some(&WhereClause::binary("Key", eq, ADA)))
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
        let source = ds.create_readonly_repository::<JoinRow>(Some(connection.clone()), None);
        let command: Arc<dyn Query> = Arc::new(QueryStatement::new(
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
            None,
            &[
                Join::left("user_type", "user_type.id", "user.user_type_id"),
                Join::left("user_roles", "user_roles.user_id", "user.id"),
                Join::left("roles", "roles.id", "user_roles.role_id"),
            ],
        ));
        let users_repo = EnrichedReadonlyRepository::with_strategy(
            ds.create_readonly_repository::<UsersView>(Some(connection.clone()), None),
            Arc::new(FlatToNestedReadonlyEnrichmentStrategy::new(
                source,
                FlatToNestedMapper::new(TypeMap::new(
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
                )),
                |row| {
                    let mut view = datasources::Row::new();
                    for key in ["Key", "Login", "Active", "Opened", "UserType"] {
                        if let Some(MappedValue::Cell(value)) = row.get(key) {
                            view.insert(key.into(), value.clone());
                        }
                    }
                    let roles = match row.get("Roles") {
                        Some(MappedValue::Many(items)) => items
                            .iter()
                            .map(|item| {
                                let mut role = datasources::Row::new();
                                for key in ["UserId", "Name"] {
                                    if let Some(MappedValue::Cell(value)) = item.get(key) {
                                        role.insert(key.into(), value.clone());
                                    }
                                }
                                role
                            })
                            .collect(),
                        _ => Vec::new(),
                    };
                    view.insert("Roles".into(), encode_rows(&roles));
                    view
                },
                Some(command),
            )),
        );
        let items = users_repo.read(None).await.unwrap();
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
