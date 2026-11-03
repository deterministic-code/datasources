use crate::program;
use datasources::{Column, CreateTableStatement, DefaultStatementTextRenderer, Driver};

fn driver(provider: &str) -> Box<dyn Driver> {
    match provider {
        "sqlite" => Box::new(datasources_sqlite::SqliteDriver),
        "mysql" => Box::new(datasources_mysql::MysqlDriver),
        "postgres" => Box::new(datasources_postgres::PostgresDriver),
        name => panic!("unknown provider {name}"),
    }
}

fn user_table(provider: &str) -> String {
    let columns = [
        ("id", "int", true),
        ("uuid", "uuid", false),
        ("username", "string", false),
        ("is_enabled", "bool", false),
        ("balance", "decimal", false),
        ("location", "float", false),
        ("created", "datetime", false),
        ("version", "binary", false),
    ];
    let types = driver(provider);
    let mapped: Vec<Column> = columns
        .iter()
        .map(|(name, canonical, id)| {
            let affinity = types.column_type(canonical).unwrap();
            Column::new(
                (*name).to_string(),
                if *id {
                    format!("{affinity} PRIMARY KEY")
                } else {
                    affinity
                },
            )
        })
        .collect();
    types
        .statement_renderer()
        .expect("renderer")
        .render(&CreateTableStatement::new("user", &mapped, &[]))
}

#[test]
fn parses_commands() {
    let s = |value: &str| value.to_string();
    assert_eq!(
        program::parse(&[s("add-user"), s("a"), s("b")])
            .unwrap()
            .values,
        ["a", "b"]
    );
    assert_eq!(
        program::parse(&[s("add-role"), s("admin")]).unwrap().verb,
        "add-role"
    );
    assert_eq!(
        program::parse(&[s("add-group"), s("staff")]).unwrap().verb,
        "add-group"
    );
    assert_eq!(
        program::parse(&[s("add-user-role"), s("a"), s("admin")])
            .unwrap()
            .verb,
        "add-user-role"
    );
    assert_eq!(
        program::parse(&[s("add-user-group"), s("a"), s("staff")])
            .unwrap()
            .verb,
        "add-user-group"
    );
    assert_eq!(
        program::parse(&[s("enable-user"), s("a")]).unwrap().verb,
        "enable-user"
    );
    assert_eq!(
        program::parse(&[s("disable-user"), s("a")]).unwrap().verb,
        "disable-user"
    );
    assert_eq!(
        program::parse(&[s("list-users-roles-groups")])
            .unwrap()
            .verb,
        "list-users-roles-groups"
    );
    for verb in [
        "add-user",
        "add-role",
        "add-group",
        "add-user-role",
        "add-user-group",
        "enable-user",
        "disable-user",
        "nope",
    ] {
        assert!(program::parse(&[s(verb)]).is_err());
    }
    assert!(program::parse(&[]).is_err());
    assert!(program::parse(&[s("add-user"), s("a"), s("b"), s("extra")]).is_err());
    assert!(program::parse(&[s("list-users-roles-groups"), s("extra")]).is_err());
}

#[test]
fn builds_schema() {
    let tables = datasources_tests::schema_tables("user-roles-cli", |id| id.to_string(), None);
    let sql = tables
        .iter()
        .map(|table| DefaultStatementTextRenderer::instance().render(table))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(sql.contains("FOREIGN KEY (user_id) REFERENCES"));
    assert!(sql.contains("FOREIGN KEY (bad_role_id) REFERENCES"));
    assert!(sql.contains("ugly_name"));
    assert!(tables[0].columns[0].identity);
    assert!(DefaultStatementTextRenderer::instance()
        .render(&tables[0])
        .contains("CONSTRAINT pk_users PRIMARY KEY"));
    assert_eq!(
        user_table("sqlite"),
        "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" TEXT, \"username\" TEXT, \"is_enabled\" BOOLEAN, \"balance\" NUMERIC, \"location\" REAL, \"created\" TEXT, \"version\" BLOB)"
    );
    assert_eq!(
        user_table("mysql"),
        "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)"
    );
    assert_eq!(
        user_table("postgres"),
        "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" UUID, \"username\" TEXT, \"is_enabled\" BOOLEAN, \"balance\" NUMERIC, \"location\" DOUBLE PRECISION, \"created\" TIMESTAMPTZ, \"version\" BYTEA)"
    );
    assert_eq!(program::map_name("name"), "ugly_name");
    assert_eq!(program::map_name("role_id"), "bad_role_id");
    assert_eq!(program::map_name("username"), "username");
    assert_eq!(program::reverse_map_name("ugly_name"), "name");
    assert_eq!(program::reverse_map_name("bad_role_id"), "role_id");
    assert_eq!(program::reverse_map_name("username"), "username");
    assert!(include_str!("../../../user-roles-simple-cli-shared/mysql-procedures.sql")
        .contains("INSERT INTO `groups` (ugly_name"));
    assert_eq!(
        program::list_query_text(),
        "SELECT users.id AS id, users.username AS username, users.is_enabled AS is_enabled, users.created AS created, roles.id AS role_id, roles.name AS role_name, roles.is_enabled AS role_is_enabled, groups.id AS group_id, groups.ugly_name AS group_name, groups.is_enabled AS group_is_enabled FROM users LEFT JOIN user_roles ON users.id = user_roles.user_id LEFT JOIN roles ON user_roles.bad_role_id = roles.id LEFT JOIN user_groups ON users.id = user_groups.user_id LEFT JOIN groups ON user_groups.group_id = groups.id ORDER BY users.id ASC, roles.id ASC, groups.id ASC"
    );
}

#[test]
fn hashes_password() {
    let hash = program::hash("secret").unwrap();
    assert_ne!(hash, "secret");
    assert!(bcrypt::verify("secret", &hash).unwrap());
}

#[test]
fn normalizes_dialect_noise_and_matches_sql_text() {
    assert_eq!(
        crate::console_log::normalize_console_line("[2026-09-22T00:00:00.000Z]-[datasource][Request]-[{password: hash, updated: AQID, blob: ab12, id: 11111111-1111-1111-1111-111111111111}] 2ms"),
        "[<date>]-[datasource][Request]-[{password: <token>, updated: <token>, blob: <token>, id: <uuid>}]"
    );
    assert_eq!(
        crate::console_log::normalize_console_line("[2026-09-22T00:00:00.000Z]-[datasource][Start]-[INSERT INTO `users` (\"roles\") VALUES ($name)] 3μs"),
        "[<date>]-[datasource][Start]-[INSERT INTO users (roles) VALUES ($name)]"
    );
    assert_eq!(
        crate::console_log::normalize_console_line("count: 1234"),
        "count: 1234"
    );
    assert_eq!(crate::console_log::normalize_console_line("plain"), "plain");
    assert_eq!(
        crate::console_log::normalize_console_line("[x] kept"),
        "[x] kept"
    );
    assert_eq!(
        crate::console_log::dialect_transcript(&[
            "[2026-09-22T00:00:00.000Z]-[datasource][Request]-[{is_enabled: True}]".into()
        ]),
        vec!["[<date>]-[datasource][Request]-[{is_enabled: 1}]".to_string()]
    );
    assert_eq!(
        crate::console_log::dialect_transcript(&[
            "[2026-09-22T00:00:00.000Z]-[datasource][Start]-[UPDATE users SET is_enabled = $is_enabled]".into(),
            "[2026-09-22T00:00:00.000Z]-[datasource][AfterCommand]-[{result: 5, rowsAffected: 1}]".into(),
        ]),
        vec![
            "[<date>]-[datasource][Start]-[UPDATE users SET is_enabled = $is_enabled]".to_string(),
            "[<date>]-[datasource][AfterCommand]-[{result: <id>, rowsAffected: 1}]".to_string(),
        ]
    );
    assert_eq!(
        crate::console_log::dialect_transcript(&[
            "[2026-09-22T00:00:00.000Z]-[datasource][Start]-[INSERT INTO users (name) VALUES ($name)]".into(),
            "[2026-09-22T00:00:00.000Z]-[datasource][AfterCommand]-[{result: 1, rowsAffected: 1}]".into(),
        ])[1],
        "[<date>]-[datasource][AfterCommand]-[{result: 1, rowsAffected: 1}]"
    );
    let shared =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../user-roles-simple-cli-shared");
    let lines = |name: &str| {
        std::fs::read_to_string(shared.join(name))
            .unwrap()
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>()
    };
    let sqlite = crate::console_log::sql_text(&lines("sqlite.log"));
    assert_eq!(crate::console_log::sql_text(&lines("mysql.log")), sqlite);
    assert_eq!(crate::console_log::sql_text(&lines("postgres.log")), sqlite);
    let transcript = crate::console_log::dialect_transcript(&lines("sqlite.log"));
    assert_eq!(
        crate::console_log::dialect_transcript(&lines("mysql.log")),
        transcript
    );
    assert_eq!(
        crate::console_log::dialect_transcript(&lines("postgres.log")),
        transcript
    );
}

#[tokio::test]
async fn omits_null_created_and_updated() {
    use chrono::{TimeZone, Utc};
    use datasources::{
        CRUDRepository, CRUDRepositoryOptions, Driver, ICRUDRepository, Row, RowTypeConverter,
        Value,
    };
    use std::collections::HashMap;

    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), "sqlite:///:memory:".into());
    let connection = datasources_sqlite::SqliteDriver.connection(Some(&options));
    connection.open().await.unwrap();
    connection
        .create_command()
        .execute_native(
            &datasources::Native::new("CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                created TEXT NOT NULL DEFAULT '2020-01-01T00:00:00.000Z',
                updated BLOB NOT NULL DEFAULT X'0102'
            )"),
            None,
        )
        .await
        .unwrap();
    let mut repo_options = CRUDRepositoryOptions::default();
    repo_options.entity = "users".into();
    repo_options.row_converter = Some(RowTypeConverter::new(vec![
        "id".into(),
        "username".into(),
        "created".into(),
        "updated".into(),
    ]));
    let repo = CRUDRepository::new(connection, repo_options).unwrap();
    let mut omitted = Row::new();
    omitted.insert("username".into(), Value::from("ada"));
    omitted.insert("created".into(), Value::Null);
    omitted.insert("updated".into(), Value::Null);
    repo.create(omitted).await.unwrap();
    let stored = repo
        .read_single_by_id(omitted.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let mut renamed = stored.clone();
    renamed.insert("username".into(), Value::from("ada-v2"));
    repo.update(renamed).await.unwrap();
    let kept = repo
        .read_single_by_id(omitted.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let created = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
    let mut explicit = Row::new();
    explicit.insert("username".into(), Value::from("grace"));
    explicit.insert("created".into(), Value::DateTime(created));
    explicit.insert("updated".into(), Value::Binary(vec![9, 9]));
    repo.create(explicit).await.unwrap();
    let written = repo
        .read_single_by_id(explicit.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        stored.get("created"),
        Some(&Value::from("2020-01-01T00:00:00.000Z"))
    );
    assert_eq!(stored.get("updated"), Some(&Value::Binary(vec![1, 2])));
    assert_eq!(kept.get("username"), Some(&Value::from("ada-v2")));
    assert_eq!(kept.get("created"), stored.get("created"));
    assert_eq!(kept.get("updated"), stored.get("updated"));
    assert_eq!(
        written.get("created"),
        Some(&Value::Text(Value::iso_datetime(created)))
    );
    assert_eq!(written.get("updated"), Some(&Value::Binary(vec![9, 9])));
}
