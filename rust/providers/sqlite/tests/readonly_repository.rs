use chrono::{TimeZone, Utc};
use datasources::{
    create_readonly_repository_mapping_options, default, eq, gt, BuiltDatasource, Casing, ChangeCasingMapper,
    Connection, ConvertSchema, Datasource, DictionaryMapper, Mapper, Native, PluralizerMapper,
    Result, Row, TransformMapper, TypeConverter, Value, Where,
};
use std::sync::Arc;
use datasources_sql_specs::apply_schema;
use std::collections::HashMap;
use uuid::Uuid;

const ADA: &str = "11111111-1111-1111-1111-111111111111";
const GRACE: &str = "22222222-2222-2222-2222-222222222222";
const LIN: &str = "33333333-3333-3333-3333-333333333333";

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

fn snake() -> ChangeCasingMapper {
    ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)
}

fn snake_then_plural() -> TransformMapper<String, String> {
    let snake = Arc::new(snake());
    let forward = snake.clone();
    let reverse = snake;
    TransformMapper::new(
        move |name: String| PluralizerMapper::new().map(Mapper::map(forward.as_ref(), name)),
        move |name: String| {
            Mapper::reverse_map(reverse.as_ref(), PluralizerMapper::new().reverse_map(name))
        },
    )
}

fn datasource() -> Arc<BuiltDatasource> {
    Arc::new(BuiltDatasource::new(
        "sqlite",
        Arc::new(crate::SqliteDriver),
        Some(HashMap::from([(
            "DATASOURCE_URL".into(),
            "sqlite:///:memory:".into(),
        )])),
    ))
}

#[tokio::test]
async fn reads_users_by_id_and_by_username_and_enabled() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = open(ds.as_ref()).await;
    apply_schema(connection.as_ref(), "minimal-users").await.unwrap();
    execute_sql(
        &connection,
        "INSERT INTO users (id, username, is_enabled) VALUES (1, 'ada', 1)",
    )
    .await;
    execute_sql(
        &connection,
        "INSERT INTO users (id, username, is_enabled) VALUES (2, 'grace', 0)",
    )
    .await;
    execute_sql(
        &connection,
        "INSERT INTO users (id, username, is_enabled) VALUES (3, 'bea', 1)",
    )
    .await;
    let mut options = create_readonly_repository_mapping_options::<User>();
    options.entity_mapper = Some(std::sync::Arc::new(snake_then_plural()));
    options.property_mappings = Some(std::sync::Arc::new(snake()));
    let repo = ds.create_readonly_repository::<User>(Some(connection), Some(options));
    let by_id = repo
        .read_single(Some(&Where::binary("Id", eq, 1_i64)), None, None)
        .await
        .unwrap()
        .unwrap();
    let row = repo
        .read(
            Some(&Where::binary("Username", eq, "grace").and("IsEnabled", eq, 0_i64)),
            None, None
        )
        .await
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    assert_eq!(by_id.get("Username"), Some(&Value::from("ada")));
    assert_eq!(by_id.get("IsEnabled"), Some(&Value::Bool(true)));
    assert_eq!(row.get("Id"), Some(&Value::Int(2)));
    assert_eq!(row.get("Username"), Some(&Value::from("grace")));
    assert_eq!(row.get("IsEnabled"), Some(&Value::Bool(false)));
}

struct UserConverter;

impl TypeConverter<Row, Row> for UserConverter {
    fn convert(&self, from: &Row) -> Result<Row> {
        let mut row = from.clone();
        let enabled = match row.get("IsEnabled") {
            Some(Value::Bool(flag)) => Value::Bool(*flag),
            Some(value) => value.clone(),
            None => Value::Null,
        };
        row.insert("IsEnabled".into(), enabled);
        Ok(row)
    }

    fn reverse_convert(&self, to: &Row) -> Result<Row> {
        let mut row = to.clone();
        let enabled = match row.get("IsEnabled") {
            Some(Value::Bool(flag)) => Value::Int(if *flag { 1 } else { 0 }),
            Some(value) => value.clone(),
            None => Value::Null,
        };
        row.insert("IsEnabled".into(), enabled);
        Ok(row)
    }
}

#[tokio::test]
async fn maps_an_object_explicitly() {
    let built = datasource();
    let ds: &dyn Datasource = &*built;
    let connection = open(ds).await;
    apply_schema(connection.as_ref(), "minimal-users").await.unwrap();
    execute_sql(
        &connection,
        "
        INSERT INTO users (id, username, is_enabled) VALUES
            (1, 'ada', 1),
            (2, 'grace', 0);
        ",
    )
    .await;
    let mut options = create_readonly_repository_mapping_options::<User>();
    options.entity_mapper = Some(std::sync::Arc::new(snake_then_plural()));
    options.property_mappings = Some(std::sync::Arc::new(snake()));
    options.object_converter = Some(Box::new(UserConverter));
    let users_repo = ds.create_readonly_repository::<User>(Some(connection), Some(options));
    let user = users_repo
        .read_single(Some(&Where::binary("Id", eq, 1_i64)), None, None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.get("Id"), Some(&Value::Int(1)));
    assert_eq!(user.get("Username"), Some(&Value::from("ada")));
    assert_eq!(user.get("IsEnabled"), Some(&Value::Bool(true)));
}

#[tokio::test]
async fn reads_user_by_guid_username_and_created() {
    let built = datasource();
    let ds: &dyn Datasource = &*built;
    let connection = open(ds).await;
    apply_schema(connection.as_ref(), "user-account").await.unwrap();
    execute_sql(
        &connection,
        &format!(
            "
        INSERT INTO user (id, username, is_enabled, created) VALUES
            ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
            ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
            ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z');
        "
        ),
    )
    .await;
    let mut options = create_readonly_repository_mapping_options::<UserAccount>();
    options.entity_mapper = Some(std::sync::Arc::new(DictionaryMapper::new([(
        "UserAccount".to_string(),
        "user".to_string(),
    )])));
    options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
        ("Key".to_string(), "id".to_string()),
        ("Login".to_string(), "username".to_string()),
        ("Active".to_string(), "is_enabled".to_string()),
        ("Opened".to_string(), "created".to_string()),
    ])));
    let repo = ds.create_readonly_repository::<UserAccount>(Some(connection), Some(options));
    let opened = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let by_id = repo
        .read_single(
            Some(&Where::binary("Key", eq, Uuid::parse_str(ADA).unwrap())),
            None, None
        )
        .await
        .unwrap()
        .unwrap();
    let by_login = repo
        .read(
            Some(&Where::binary("Login", eq, "ada").and("Active", eq, true)),
            None, None
        )
        .await
        .unwrap();
    let later = repo
        .read(
            Some(&Where::binary("Active", eq, true).and("Opened", gt, opened)),
            None, None
        )
        .await
        .unwrap();
    assert_eq!(by_id.get("Login"), Some(&Value::from("ada")));
    assert_eq!(by_id.get("Key"), Some(&Value::from(ADA)));
    assert_eq!(by_id.get("Active"), Some(&Value::Bool(true)));
    assert_eq!(
        by_id.get("Opened"),
        Some(&Value::from("2020-01-01T00:00:00.000Z"))
    );
    assert_eq!(by_login.len(), 1);
    assert_eq!(by_login[0].get("Login"), Some(&Value::from("ada")));
    assert_eq!(later.len(), 1);
    assert_eq!(later[0].get("Login"), Some(&Value::from("grace")));
}

async fn open(ds: &dyn Datasource) -> Arc<dyn Connection> {
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    connection
}

async fn execute_sql(connection: &Arc<dyn Connection>, sql: &str) {
    let command = connection.create_command();
    for statement in sql
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        command
            .execute_native(&Native::new(statement), None)
            .await
            .unwrap();
    }
}

mod user_view {
    use super::{apply_schema, datasource, execute_sql, open, ADA, GRACE, LIN};
    use datasources::{
        create_readonly_repository_mapping_options, eq, Casing, ChangeCasingMapper, ConvertSchema, Datasource,
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
        let connection = open(ds).await;
        apply_schema(connection.as_ref(), "user-type-lookup").await.unwrap();
        execute_sql(
            &connection,
            &format!(
                "
            INSERT INTO user_type (id, name) VALUES
                (1, 'admin'),
                (2, 'user'),
                (3, 'service');
            INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
                ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
                ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
                ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
            "
            ),
        )
        .await;
        let mut users_options = create_readonly_repository_mapping_options::<UserAccount>();
        users_options.entity = "user".into();
        users_options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
            ("Active".to_string(), "is_enabled".to_string()),
            ("Opened".to_string(), "created".to_string()),
            ("UserTypeId".to_string(), "user_type_id".to_string()),
        ])));
        let mut type_options = create_readonly_repository_mapping_options::<UserType>();
        type_options.entity = "UserType".into();
        type_options.entity_mapper =
            Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        type_options.property_mappings =
            Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        let mut users_repo =
            ReadonlyRepository::new(connection.clone(), users_options).unwrap();
        let types =
            ds.create_readonly_repository::<UserType>(Some(connection), Some(type_options));
        users_repo.add_enriched(
            datasources::Enrichment::new(
                "UserType",
                "UserTypeId",
                "Id",
                Some("Name".into()),
                datasources::EnrichmentPropertyType::Object,
            ),
            types,
            None,
        );
        let account = users_repo
            .read_single(
                Some(&Where::binary("Key", eq, Uuid::parse_str(ADA).unwrap())),
                None, None
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
        assert_eq!(users_view.get("Active"), Some(&Value::Bool(true)));
        assert_eq!(
            users_view.get("Opened"),
            Some(&Value::from("2020-01-01T00:00:00.000Z"))
        );
        assert!(users_view.get("UserTypeId").is_none());
    }
}

mod users_view_roles {
    use super::{apply_schema, datasource, execute_sql, open, ADA, GRACE, LIN};
    use datasources::{
        create_readonly_repository_mapping_options, eq, Casing, ChangeCasingMapper, ConvertSchema, Datasource,
        DictionaryMapper, HashsetOptimizedReadonlyEnrichmentStrategy, IReadonlyRepository, Join,
        NestedToObjectEnrichmentStrategy, ReadonlyRepository, ReadonlyRepositoryMappingOptions,
        SelectStatement, SelectStatementOptions, TypeMap,
        TypeMapField, TypeMapGroup, Value, Where,
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
            INSERT INTO user_type (id, name) VALUES
                (1, 'admin'),
                (2, 'user'),
                (3, 'service');
            INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
                ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
                ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
                ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
            INSERT INTO roles (id, name) VALUES
                (1, 'admin'),
                (2, 'editor'),
                (3, 'viewer');
            INSERT INTO user_roles (user_id, role_id) VALUES
                ('{ADA}', 1),
                ('{ADA}', 2),
                ('{GRACE}', 3);
            "
        )
    }

    fn joined_users() -> SelectStatement {
        let joins = [
            Join::left("UserType", "UserType.Id", "User.UserTypeId"),
            Join::left("UserRole", "UserRole.UserId", "User.Key"),
            Join::left("Role", "Role.Id", "UserRole.RoleId"),
        ];
        SelectStatement::new(SelectStatementOptions {
            joins: &joins,
            ..SelectStatementOptions::new(
                "User",
                &[
                    "User.Key",
                    "User.Login",
                    "User.Active",
                    "User.Opened",
                    "User.UserType",
                    "UserRole.UserId",
                    "UserRole.RoleId",
                    "Role.Name",
                ],
            )
        })
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
        let connection = open(ds).await;
        apply_schema(connection.as_ref(), "user-roles-read")
            .await
            .unwrap();
        execute_sql(&connection, &seed()).await;
        let mut users_options = create_readonly_repository_mapping_options::<UsersView>();
        users_options.entity_mapper = Some(std::sync::Arc::new(DictionaryMapper::new([(
            "UsersView".to_string(),
            "user".to_string(),
        )])));
        users_options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
            ("Active".to_string(), "is_enabled".to_string()),
            ("Opened".to_string(), "created".to_string()),
            ("UserType".to_string(), "user_type_id".to_string()),
        ])));
        let mut type_options = create_readonly_repository_mapping_options::<UserType>();
        type_options.entity_mapper =
            Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        type_options.property_mappings =
            Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        let mut role_options = create_readonly_repository_mapping_options::<UserRole>();
        role_options.entity_mapper = Some(std::sync::Arc::new(DictionaryMapper::new([(
            "UserRole".to_string(),
            "user_roles".to_string(),
        )])));
        role_options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
            ("UserId".to_string(), "user_id".to_string()),
            ("Name".to_string(), "name".to_string()),
        ])));
        let mut users_repo =
            ReadonlyRepository::new(connection.clone(), users_options).unwrap();
        let types = ds.create_readonly_repository::<UserType>(
            Some(connection.clone()),
            Some(type_options),
        );
        let roles =
            ds.create_readonly_repository::<UserRole>(Some(connection), Some(role_options));
        let strategy = Arc::new(HashsetOptimizedReadonlyEnrichmentStrategy);
        users_repo.add_enriched(
            datasources::Enrichment::new("UserType", "UserType", "Id", Some("Name".into()), datasources::EnrichmentPropertyType::Object),
            types,
            Some(strategy.clone()),
        );
        users_repo.add_enriched(
            datasources::Enrichment::new("Roles", "Key", "UserId", None, datasources::EnrichmentPropertyType::List),
            roles,
            Some(strategy),
        );
        let users_view = users_repo
            .read_single(Some(&Where::binary("Key", eq, ADA)), None, None)
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
        let connection = open(ds).await;
        apply_schema(connection.as_ref(), "user-roles-read")
            .await
            .unwrap();
        execute_sql(&connection, &seed()).await;
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "User".into();
        options.entity_mapper = Some(std::sync::Arc::new(DictionaryMapper::new([
            ("User".into(), "user".into()),
            ("UserType".into(), "user_type".into()),
            ("UserRole".into(), "user_roles".into()),
            ("Role".into(), "roles".into()),
        ])));
        options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
            ("Key".into(), "id".into()),
            ("Login".into(), "username".into()),
            ("Active".into(), "is_enabled".into()),
            ("Opened".into(), "created".into()),
            ("UserType".into(), "user_type.name".into()),
            ("UserTypeId".into(), "user_type_id".into()),
            ("UserId".into(), "user_id".into()),
            ("RoleId".into(), "role_id".into()),
            ("Name".into(), "roles.name".into()),
            ("Id".into(), "id".into()),
        ])));
        let mut users_repo = ReadonlyRepository::new(connection.clone(), options).unwrap();
        users_repo.add_enriched(
            datasources::Enrichment::new("Roles", "Key", "UserId", None, datasources::EnrichmentPropertyType::List),
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
                                    ("Name".into(), TypeMapField::column("Name")),
                                ],
                            )),
                        ),
                    ],
                ),
            ))),
        );
        let joined = joined_users();
        let items = users_repo.read(Some(&joined), None, None).await.unwrap();
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
        assert_eq!(ada.get("Key"), Some(&Value::from(ADA)));
        assert_eq!(ada.get("Login"), Some(&Value::from("ada")));
        assert_eq!(ada.get("Active"), Some(&Value::Bool(true)));
        assert_eq!(
            ada.get("Opened"),
            Some(&Value::from("2020-01-01T00:00:00.000Z"))
        );
        assert_eq!(ada.get("UserType"), Some(&Value::from("admin")));
        let mut names = role_names(ada.get("Roles").unwrap());
        names.sort();
        assert_eq!(names, ["admin", "editor"]);
        assert!(ada
            .get("Roles")
            .unwrap()
            .wire_text()
            .contains(&format!("UserId={ADA}")));
        assert_eq!(grace.get("Key"), Some(&Value::from(GRACE)));
        assert_eq!(grace.get("Login"), Some(&Value::from("grace")));
        assert_eq!(grace.get("Active"), Some(&Value::Bool(true)));
        assert_eq!(
            grace.get("Opened"),
            Some(&Value::from("2024-06-01T00:00:00.000Z"))
        );
        assert_eq!(grace.get("UserType"), Some(&Value::from("user")));
        assert_eq!(role_names(grace.get("Roles").unwrap()), ["viewer"]);
        assert!(grace
            .get("Roles")
            .unwrap()
            .wire_text()
            .contains(&format!("UserId={GRACE}")));
        assert_eq!(lin.get("Key"), Some(&Value::from(LIN)));
        assert_eq!(lin.get("Login"), Some(&Value::from("lin")));
        assert_eq!(lin.get("Active"), Some(&Value::Bool(false)));
        assert_eq!(
            lin.get("Opened"),
            Some(&Value::from("2024-06-01T00:00:00.000Z"))
        );
        assert_eq!(lin.get("UserType"), Some(&Value::from("service")));
        assert_eq!(lin.get("Roles"), Some(&Value::from("")));
    }
}

mod user_crud {
    use super::{apply_schema, datasource, execute_sql, open, User};
    use datasources::{
        create_crud_repository_options, default, eq, CRUDRepository, Casing,
        ChangeCasingMapper, ConvertSchema, Datasource, ICRUDRepository,
        RowTypeConverter, Value, Where,
    };

    #[tokio::test]
    async fn writes_user_from_the_type() {
        let _guard = super::super::env_lock();
        let ds = default().unwrap();
        let connection = open(ds.as_ref()).await;
        apply_schema(connection.as_ref(), "user-pascal").await.unwrap();
        let mut options = create_crud_repository_options::<User>();
        options.property_mappings = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        options.ids = vec!["Id".into()];
        let users_repo = ds.create_repository::<User>(Some(connection), Some(options));
        let mut created = datasources::Row::new();
        created.insert("Username".into(), Value::from("ada"));
        created.insert("IsEnabled".into(), Value::Bool(true));
        let mut created = users_repo.create(&mut created).await.unwrap();
        let by_id = users_repo
            .read_single_by_id(created.get("Id").unwrap())
            .await
            .unwrap()
            .unwrap();
        let matches = users_repo
            .read(
                Some(&Where::binary("Username", eq, "ada").and(
                    "IsEnabled",
                    eq,
                    Value::Bool(true),
                )),
                None, None
        )
            .await
            .unwrap();
        created.insert("Username".into(), Value::from("ada-v2"));
        let updated = users_repo.update(&created, None).await.unwrap();
        let renamed = users_repo
            .read_single_by_id(created.get("Id").unwrap())
            .await
            .unwrap()
            .unwrap();
        let removed = users_repo.delete(&created, None).await.unwrap();
        let gone = users_repo
            .read_single_by_id(created.get("Id").unwrap())
            .await
            .unwrap();
        assert_eq!(matches!(created.get("Id"), Some(Value::Int(id)) if *id > 0), true);
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

    mod enriched {
        use super::super::{apply_schema, open};
        use datasources::{
            create_crud_repository_options, default, encode_rows, eq, CRUDRepository, Casing,
            ChangeCasingMapper, ConvertSchema, DictionaryMapper, ICRUDRepository,
            IReadonlyRepository, MetaDataRowTypeConverter, RowTypeConverter, Value, Where,
        };
        use std::sync::Arc;

        struct User;

        impl ConvertSchema for User {
            fn schema() -> Vec<(&'static str, &'static str)> {
                vec![
                    ("Key", "INTEGER"),
                    ("Login", "TEXT"),
                    ("Active", "INTEGER"),
                    ("Opened", "TEXT"),
                    ("UserType", "INTEGER"),
                    ("Version", "INTEGER"),
                ]
            }
        }

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
    async fn updates_users_view_when_the_version_matches() {
        let _guard = super::super::super::env_lock();
        let ds = default().unwrap();
        let connection = open(ds.as_ref()).await;
        apply_schema(connection.as_ref(), "user-view-version").await.unwrap();
        let mut users_options = create_crud_repository_options::<User>();
        users_options.ids = vec!["Key".into()];
        users_options.entity_mapper =
            Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        users_options.property_mappings = Some(std::sync::Arc::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
            ("Active".to_string(), "is_enabled".to_string()),
            ("Opened".to_string(), "created".to_string()),
            ("UserType".to_string(), "user_type_id".to_string()),
            ("Version".to_string(), "version".to_string()),
        ])));
        users_options.row_converter = Some(RowTypeConverter::of::<User>());
        users_options.object_converter = Some(Box::new(
            MetaDataRowTypeConverter::of::<User>().with_references(
                vec!["UserType".into()],
                Box::new(DictionaryMapper::new([
                    ("Key".to_string(), "id".to_string()),
                    ("Login".to_string(), "username".to_string()),
                    ("Active".to_string(), "is_enabled".to_string()),
                    ("Opened".to_string(), "created".to_string()),
                    ("UserType".to_string(), "user_type_id".to_string()),
                    ("Version".to_string(), "version".to_string()),
                ])),
            ),
        ));
        let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
        let mut type_options = create_crud_repository_options::<UserType>();
        type_options.entity_mapper =
            Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        type_options.property_mappings = Some(std::sync::Arc::new(casing));
        type_options.row_converter = Some(RowTypeConverter::of::<UserType>());
        let mut role_options = create_crud_repository_options::<UserRole>();
        role_options.entity_mapper =
            Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        role_options.property_mappings =
            Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        role_options.row_converter = Some(RowTypeConverter::of::<UserRole>());
        let mut users_repo = CRUDRepository::new(connection.clone(), users_options).unwrap();
        let user_types_repo =
            Arc::new(CRUDRepository::new(connection.clone(), type_options).unwrap());
        let user_roles_repo = Arc::new(CRUDRepository::new(connection, role_options).unwrap());
        users_repo.add_enriched(
            datasources::Enrichment::new(
                "UserType",
                "UserType",
                "Id",
                None,
                datasources::EnrichmentPropertyType::Object,
            ),
            user_types_repo.clone(),
            None,
        );
        users_repo.add_enriched(
            datasources::Enrichment::new("Roles", "Key", "UserId", None, datasources::EnrichmentPropertyType::List),
            user_roles_repo.clone(),
            None,
        );
        let mut admin = datasources::Row::new();
        admin.insert("Name".into(), Value::from("admin"));
        let admin = user_types_repo.create(&mut admin).await.unwrap();
        let mut admin_role = datasources::Row::new();
        admin_role.insert("Name".into(), Value::from("admin"));
        let mut editor_role = datasources::Row::new();
        editor_role.insert("Name".into(), Value::from("editor"));
        let mut user = datasources::Row::new();
        user.insert("Login".into(), Value::from("ada"));
        user.insert("Active".into(), Value::Bool(true));
        user.insert("Opened".into(), Value::from("2020-01-01T00:00:00.000Z"));
        user.insert("UserType".into(), admin.get("Id").cloned().unwrap());
        user.insert("Version".into(), Value::Int(1));
        user.insert("Roles".into(), encode_rows(&[admin_role, editor_role]));
        let user = users_repo.create(&mut user).await.unwrap();
        let mut viewer_role = datasources::Row::new();
        viewer_role.insert("Name".into(), Value::from("viewer"));
        let mut grace = datasources::Row::new();
        grace.insert("Login".into(), Value::from("grace"));
        grace.insert("Active".into(), Value::Bool(true));
        grace.insert("Opened".into(), Value::from("2024-06-01T00:00:00.000Z"));
        grace.insert("UserType".into(), admin.get("Id").cloned().unwrap());
        grace.insert("Version".into(), Value::Int(1));
        grace.insert("Roles".into(), encode_rows(&[viewer_role]));
        let grace = users_repo.create(&mut grace).await.unwrap();
        let key = user.get("Key").cloned().unwrap();
        let mut loaded = users_repo
            .read_single(Some(&Where::binary("Key", eq, key.clone())), None, None)
            .await
            .unwrap()
            .unwrap();
        let version = loaded.get("Version").cloned().unwrap();
        let mut member = datasources::Row::new();
        member.insert("Name".into(), Value::from("member"));
        let member = user_types_repo.create(&mut member).await.unwrap();
        let mut owner_role = datasources::Row::new();
        owner_role.insert("Name".into(), Value::from("owner"));
        let mut kept_admin = datasources::Row::new();
        kept_admin.insert("Name".into(), Value::from("admin"));
        let mut kept_editor = datasources::Row::new();
        kept_editor.insert("Name".into(), Value::from("editor"));
        loaded.insert("Login".into(), Value::from("ada-v2"));
        loaded.insert("UserType".into(), member.get("Id").cloned().unwrap());
        loaded.insert(
            "Roles".into(),
            encode_rows(&[kept_admin, kept_editor, owner_role]),
        );
        loaded.insert("Version".into(), Value::Int(2));
        let updated = users_repo
            .update(
                &loaded,
                Some(&Where::binary("Key", eq, key.clone()).and("Version", eq, version.clone())),
            )
            .await
            .unwrap();
        let current = users_repo
            .read_single(Some(&Where::binary("Key", eq, key.clone())), None, None)
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
        let gone = users_repo.read_single_by_id(&key).await.unwrap();
        assert_eq!(current.get("Version"), Some(&Value::Int(2)));
        assert_eq!(current.get("Login"), Some(&Value::from("ada-v2")));
        assert_eq!(current.get("Active"), Some(&Value::Bool(true)));
        assert_eq!(current.get("UserType"), Some(&Value::from("Id=2,Name=member")));
        let mut current_names = role_names(current.get("Roles").unwrap());
        current_names.sort();
        assert_eq!(current_names, ["admin", "editor", "owner"]);
        let roles = user_roles_repo
            .read(Some(&Where::binary("UserId", eq, key.clone())), None, None)
            .await
            .unwrap();
        assert_eq!(admin.get("Id"), Some(&Value::Int(1)));
        assert_eq!(user.get("Key"), Some(&Value::Int(1)));
        assert_eq!(grace.get("Key"), Some(&Value::Int(2)));
        assert_eq!(member.get("Id"), Some(&Value::Int(2)));
        assert_eq!(updated.rows_affected, 1);
        assert_eq!(stale.rows_affected, 0);
        assert_eq!(removed.rows_affected, 1);
        assert_eq!(gone.is_none(), true);
        assert_eq!(roles.is_empty(), true);
    }
    }

    struct Account;

    impl ConvertSchema for Account {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![
                ("TenantId", "INTEGER"),
                ("Id", "INTEGER"),
                ("Name", "TEXT"),
            ]
        }
    }

    #[tokio::test]
    async fn reads_updates_and_deletes_by_tenant_and_id() {
        let built = datasource();
        let ds: &dyn Datasource = &*built;
        let connection = open(ds).await;
        apply_schema(connection.as_ref(), "multi-tenant").await.unwrap();
        execute_sql(
            &connection,
            "
            INSERT INTO account (tenant_id, id, name) VALUES
                (1, 1, 'ada'),
                (2, 1, 'grace');
            ",
        )
        .await;
        let mut options = create_crud_repository_options::<Account>();
        let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
        options.entity_mapper = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
        options.property_mappings = Some(std::sync::Arc::new(casing));
        options.ids = vec!["TenantId".into(), "Id".into()];
        options.row_converter = Some(RowTypeConverter::of::<Account>());
        let accounts_repo = CRUDRepository::new(connection, options).unwrap();
        let mut key = datasources::Row::new();
        key.insert("TenantId".into(), Value::Int(1));
        key.insert("Id".into(), Value::Int(1));
        let mut ada = accounts_repo.read_single_by_id(&key).await.unwrap().unwrap();
        ada.insert("Name".into(), Value::from("ada-v2"));
        let updated = accounts_repo.update(&ada, None).await.unwrap();
        let renamed = accounts_repo.read_single_by_id(&key).await.unwrap().unwrap();
        let removed = accounts_repo.delete(&ada, None).await.unwrap();
        let gone = accounts_repo.read_single_by_id(&key).await.unwrap();
        let mut other = datasources::Row::new();
        other.insert("TenantId".into(), Value::Int(2));
        other.insert("Id".into(), Value::Int(1));
        let grace = accounts_repo
            .read_single_by_id(&other)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.rows_affected, 1);
        assert_eq!(removed.rows_affected, 1);
        assert_eq!(renamed.get("Name"), Some(&Value::from("ada-v2")));
        assert_eq!(renamed.get("TenantId"), Some(&Value::Int(1)));
        assert!(gone.is_none());
        assert_eq!(grace.get("Name"), Some(&Value::from("grace")));
        assert_eq!(grace.get("TenantId"), Some(&Value::Int(2)));
    }
}
