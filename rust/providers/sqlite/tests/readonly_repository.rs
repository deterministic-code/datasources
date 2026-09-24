use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use chrono::{TimeZone, Utc};
use datasources::{
    create_repository_options, eq, gt, Command, Connection, ConvertSchema, DictionaryMapper,
    IReadonlyRepository, PluralizerMapper, ReadonlyRepository, RowTypeConverter, Value,
    WhereClause,
};
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

#[tokio::test]
async fn reads_users_by_id_and_by_username_and_enabled() {
    let connection = open(
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
    options.row_converter = Some(RowTypeConverter::of::<User>());
    let repo = ReadonlyRepository::new(connection, options.read).unwrap();
    let by_id = repo
        .read_single(Some(&WhereClause::binary("id", eq, 1_i64)))
        .await
        .unwrap()
        .unwrap();
    let disabled = repo
        .read(Some(&WhereClause::binary("username", eq, "grace").and("is_enabled", eq, 0_i64)))
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
    let connection = open(&format!(
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
    ))
    .await;
    let mut options = create_repository_options::<UserAccount>();
    options.entity = "user".into();
    options.row_converter = Some(RowTypeConverter::of::<UserAccount>());
    options.column_mappings = Box::new(DictionaryMapper::new([
        ("Key".to_string(), "id".to_string()),
        ("Login".to_string(), "username".to_string()),
        ("Active".to_string(), "is_enabled".to_string()),
        ("Opened".to_string(), "created".to_string()),
    ]));
    options.row_type_converters = crate::converters::row_type_converters();
    let repo = ReadonlyRepository::new(connection, options.read).unwrap();
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
        .read(Some(&WhereClause::binary("Login", eq, "ada").and("Active", eq, true)))
        .await
        .unwrap();
    let later = repo
        .read(Some(&WhereClause::binary("Active", eq, true).and("Opened", gt, opened)))
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

async fn open(sql: &str) -> Arc<SqliteConnection> {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(connection.clone());
    for statement in sql
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        command.execute(statement, None).await.unwrap();
    }
    connection
}
