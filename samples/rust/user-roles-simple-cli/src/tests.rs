use crate::app::App;
use crate::as_user_view;
use crate::columns;
use crate::commands::{
    AddGroup, AddRole, AddUser, AddUserGroup, AddUserRole, CliCommand, DisableUser, EnableUser,
    ListUsersRolesGroups,
};
use crate::parse;
use crate::password_hash;
use crate::schema;
use crate::users_roles_groups_query;
use crate::users_view;
use datasources::{
    Column, CreateTableStatement, DefaultStatementTextRenderer, Driver, FlatToNestedMapper,
    MappedRow, MappedValue, Mapper, MockCommand, Row, Value,
};

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
                schema::ident(provider, name),
                if *id {
                    format!("{affinity} PRIMARY KEY")
                } else {
                    affinity
                },
            )
        })
        .collect();
    DefaultStatementTextRenderer::instance().render(&CreateTableStatement::new(
        schema::ident(provider, "user"),
        &mapped,
        &[],
    ))
}

fn row(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

fn mapped(pairs: &[(&str, MappedValue)]) -> MappedRow {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

#[test]
fn parses_commands() {
    let s = |value: &str| value.to_string();
    assert_eq!(
        parse::args(&[s("add-user"), s("a"), s("b")]).unwrap(),
        CliCommand::AddUser(AddUser {
            username: "a".into(),
            password: "b".into()
        })
    );
    assert_eq!(
        parse::args(&[s("add-role"), s("admin")]).unwrap(),
        CliCommand::AddRole(AddRole {
            name: "admin".into()
        })
    );
    assert_eq!(
        parse::args(&[s("add-group"), s("staff")]).unwrap(),
        CliCommand::AddGroup(AddGroup {
            name: "staff".into()
        })
    );
    assert_eq!(
        parse::args(&[s("add-user-role"), s("a"), s("admin")]).unwrap(),
        CliCommand::AddUserRole(AddUserRole {
            username: "a".into(),
            role: "admin".into()
        })
    );
    assert_eq!(
        parse::args(&[s("add-user-group"), s("a"), s("staff")]).unwrap(),
        CliCommand::AddUserGroup(AddUserGroup {
            username: "a".into(),
            group: "staff".into()
        })
    );
    assert_eq!(
        parse::args(&[s("enable-user"), s("a")]).unwrap(),
        CliCommand::EnableUser(EnableUser {
            username: "a".into()
        })
    );
    assert_eq!(
        parse::args(&[s("disable-user"), s("a")]).unwrap(),
        CliCommand::DisableUser(DisableUser {
            username: "a".into()
        })
    );
    assert_eq!(
        parse::args(&[s("list-users-roles-groups")]).unwrap(),
        CliCommand::ListUsersRolesGroups(ListUsersRolesGroups)
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
        assert!(parse::args(&[s(verb)]).is_err());
    }
    assert!(parse::args(&[]).is_err());
    assert!(parse::args(&[s("add-user"), s("a"), s("b"), s("extra")]).is_err());
    assert!(parse::args(&[s("list-users-roles-groups"), s("extra")]).is_err());
}

#[test]
fn builds_schema() {
    for provider in ["sqlite", "mysql", "postgres"] {
        let sql = schema::tables(provider)
            .unwrap()
            .iter()
            .map(|table| DefaultStatementTextRenderer::instance().render(table))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(sql.contains(&format!(
            "FOREIGN KEY ({}) REFERENCES",
            schema::ident(provider, "user_id")
        )));
        assert!(sql.contains(&format!(
            "FOREIGN KEY ({}) REFERENCES",
            schema::ident(provider, "bad_role_id")
        )));
        assert!(sql.contains("ugly_name"));
    }
    let sqlite = schema::tables("sqlite").unwrap();
    assert!(sqlite[0].columns[0].identity);
    assert!(DefaultStatementTextRenderer::instance()
        .render(&sqlite[0])
        .contains("CONSTRAINT pk_users PRIMARY KEY"));
    assert!(schema::tables("mysql").unwrap()[0].columns[0].identity);
    assert!(schema::tables("postgres").unwrap()[0].columns[0].identity);
    assert_eq!(schema::ident("sqlite", "user"), "\"user\"");
    assert_eq!(schema::ident("postgres", "a\"b"), "\"a\"\"b\"");
    assert_eq!(schema::ident("mysql", "a`b"), "`a``b`");
    assert_eq!(schema::qualified("sqlite", "users.id"), "\"users\".\"id\"");
    assert_eq!(schema::qualified("mysql", "users.id"), "`users`.`id`");
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
    assert_eq!(columns::map("name"), "ugly_name");
    assert_eq!(columns::map("role_id"), "bad_role_id");
    assert_eq!(columns::map("username"), "username");
    assert_eq!(columns::reverse_map("ugly_name"), "name");
    assert_eq!(columns::reverse_map("bad_role_id"), "role_id");
    assert_eq!(columns::reverse_map("username"), "username");
    assert!(schema::mysql_procedures()
        .join("\n")
        .contains("INSERT INTO `groups` (ugly_name"));
    assert_eq!(
        users_roles_groups_query::text("sqlite"),
        "SELECT \"users\".id AS id, \"users\".username AS username, \"users\".is_enabled AS is_enabled, \"users\".created AS created, \"roles\".id AS role_id, \"roles\".name AS role_name, \"roles\".is_enabled AS role_is_enabled, \"groups\".id AS group_id, \"groups\".ugly_name AS group_name, \"groups\".is_enabled AS group_is_enabled FROM \"users\" LEFT JOIN \"user_roles\" ON \"users\".id = \"user_roles\".user_id LEFT JOIN \"roles\" ON \"user_roles\".bad_role_id = \"roles\".id LEFT JOIN \"user_groups\" ON \"users\".id = \"user_groups\".user_id LEFT JOIN \"groups\" ON \"user_groups\".group_id = \"groups\".id ORDER BY \"users\".id ASC, \"roles\".id ASC, \"groups\".id ASC"
    );
}

#[test]
fn hashes_password() {
    let hash = password_hash::hash("secret").unwrap();
    assert_ne!(hash, "secret");
    assert!(bcrypt::verify("secret", &hash).unwrap());
}

#[test]
fn maps_nested_views() {
    let mapped_rows = Mapper::map(
        &FlatToNestedMapper::new(users_view::type_map()),
        vec![
            row(&[
                ("id", Value::Int(1)),
                ("username", Value::Text("alice".into())),
                ("is_enabled", Value::Bool(true)),
                ("created", Value::Text("2026-01-01T00:00:00.000Z".into())),
                ("role_id", Value::Int(2)),
                ("role_name", Value::Text("admin".into())),
                ("role_is_enabled", Value::Int(1)),
                ("group_id", Value::Int(3)),
                ("group_name", Value::Text("staff".into())),
                ("group_is_enabled", Value::Bool(false)),
            ]),
            row(&[
                ("id", Value::Int(1)),
                ("username", Value::Text("alice".into())),
                ("is_enabled", Value::Bool(true)),
                ("created", Value::Text("2026-01-01T00:00:00.000Z".into())),
                ("role_id", Value::Int(4)),
                ("role_name", Value::Text("ops".into())),
                ("role_is_enabled", Value::Int(0)),
                ("group_id", Value::Null),
                ("group_name", Value::Null),
                ("group_is_enabled", Value::Null),
            ]),
        ],
    );
    let users = as_user_view::from_mapped(&mapped_rows);
    assert_eq!(users.len(), 1);
    assert_eq!(
        users[0]
            .roles
            .iter()
            .map(|role| role.name.as_str())
            .collect::<Vec<_>>(),
        ["admin", "ops"]
    );
    assert_eq!(
        as_user_view::from_mapped(&[mapped(&[
            ("id", MappedValue::Cell(Value::Text("9".into()))),
            ("username", MappedValue::Cell(Value::Int(1))),
            ("is_enabled", MappedValue::Cell(Value::Null)),
            ("created", MappedValue::Cell(Value::Float(1.0))),
            ("roles", MappedValue::Cell(Value::Null)),
        ])])[0]
            .id,
        9
    );
    assert_eq!(
        as_user_view::from_mapped(&[mapped(&[
            ("id", MappedValue::Cell(Value::Float(1.2))),
            ("username", MappedValue::Cell(Value::Null)),
            ("is_enabled", MappedValue::Cell(Value::Text("x".into()))),
            ("created", MappedValue::Cell(Value::Null)),
        ])])[0]
            .id,
        1
    );
}

#[tokio::test]
async fn writes_sqlite_and_lists() {
    let command = MockCommand::new();
    *command.reader_rows.lock().expect("mock") = vec![row(&[("id", Value::Int(7))])];
    let app = App::new(&command, "sqlite").with_clock(
        || "2026-09-22T00:00:00.000Z".into(),
        |password| Ok(format!("#{password}")),
        || vec![1, 2, 3],
    );
    app.run(CliCommand::AddUser(AddUser {
        username: "alice".into(),
        password: "secret".into(),
    }))
    .await
    .unwrap();
    app.run(CliCommand::AddRole(AddRole {
        name: "admin".into(),
    }))
    .await
    .unwrap();
    app.run(CliCommand::AddGroup(AddGroup {
        name: "staff".into(),
    }))
    .await
    .unwrap();
    *command.reader_rows.lock().expect("mock") =
        vec![row(&[("id", Value::Int(1))]), row(&[("id", Value::Int(2))])];
    app.run(CliCommand::AddUserRole(AddUserRole {
        username: "alice".into(),
        role: "admin".into(),
    }))
    .await
    .unwrap();
    *command.reader_rows.lock().expect("mock") =
        vec![row(&[("id", Value::Int(1))]), row(&[("id", Value::Int(3))])];
    app.run(CliCommand::AddUserGroup(AddUserGroup {
        username: "alice".into(),
        group: "staff".into(),
    }))
    .await
    .unwrap();
    app.run(CliCommand::DisableUser(DisableUser {
        username: "alice".into(),
    }))
    .await
    .unwrap();
    let logs = command.execute_logs.lock().expect("mock").clone();
    assert!(logs
        .iter()
        .any(|(text, _)| text.contains("INSERT INTO") && text.contains("users")));
    assert!(logs.iter().any(|(text, _)| text.contains("ugly_name")));
    assert!(logs.iter().any(|(text, _)| text.contains("bad_role_id")));
    *command.reader_rows.lock().expect("mock") = vec![row(&[
        ("id", Value::Int(1)),
        ("username", Value::Text("alice".into())),
        ("is_enabled", Value::Bool(false)),
        ("created", Value::Text("2026-09-22T00:00:00.000Z".into())),
        ("role_id", Value::Int(2)),
        ("role_name", Value::Text("admin".into())),
        ("role_is_enabled", Value::Bool(true)),
        ("group_id", Value::Int(3)),
        ("group_name", Value::Text("staff".into())),
        ("group_is_enabled", Value::Bool(true)),
    ])];
    let json = app
        .run(CliCommand::ListUsersRolesGroups(ListUsersRolesGroups))
        .await
        .unwrap();
    assert!(json.contains("\"username\":\"alice\""));
    assert!(!json.contains("password"));
    assert!(!json.contains("ugly_name"));
}

#[tokio::test]
async fn writes_mysql_procedures() {
    let command = MockCommand::new();
    *command.reader_rows.lock().expect("mock") = vec![row(&[("id", Value::Int(1))])];
    let app = App::new(&command, "mysql").with_clock(
        || "2026-09-22T00:00:00.000Z".into(),
        |password| Ok(format!("#{password}")),
        || vec![1, 2, 3],
    );
    app.run(CliCommand::AddUser(AddUser {
        username: "alice".into(),
        password: "secret".into(),
    }))
    .await
    .unwrap();
    app.run(CliCommand::AddRole(AddRole {
        name: "admin".into(),
    }))
    .await
    .unwrap();
    app.run(CliCommand::AddGroup(AddGroup {
        name: "staff".into(),
    }))
    .await
    .unwrap();
    *command.reader_rows.lock().expect("mock") =
        vec![row(&[("id", Value::Int(1))]), row(&[("id", Value::Int(2))])];
    app.run(CliCommand::AddUserRole(AddUserRole {
        username: "alice".into(),
        role: "admin".into(),
    }))
    .await
    .unwrap();
    *command.reader_rows.lock().expect("mock") =
        vec![row(&[("id", Value::Int(1))]), row(&[("id", Value::Int(3))])];
    app.run(CliCommand::AddUserGroup(AddUserGroup {
        username: "alice".into(),
        group: "staff".into(),
    }))
    .await
    .unwrap();
    app.run(CliCommand::EnableUser(EnableUser {
        username: "alice".into(),
    }))
    .await
    .unwrap();
    let logs = command.execute_logs.lock().expect("mock").clone();
    assert!(logs.iter().all(|(text, _)| text.starts_with("CALL ")));
}

#[tokio::test]
async fn mysql_sql_path_inserts_and_updates() {
    let command = MockCommand::new();
    *command.reader_rows.lock().expect("mock") = vec![row(&[("id", Value::Int(1))])];
    let app = App::new(&command, "mysql").procedures(false).with_clock(
        || "2026-09-22T00:00:00.000Z".into(),
        |password| Ok(format!("#{password}")),
        || vec![1, 2, 3],
    );
    app.run(CliCommand::AddUser(AddUser {
        username: "alice".into(),
        password: "secret".into(),
    }))
    .await
    .unwrap();
    app.run(CliCommand::EnableUser(EnableUser {
        username: "alice".into(),
    }))
    .await
    .unwrap();
    let logs = command.execute_logs.lock().expect("mock").clone();
    assert!(logs.iter().any(|(text, _)| text.contains("INSERT INTO")));
    assert!(logs.iter().any(|(text, _)| text.contains("UPDATE")));
    assert!(logs.iter().all(|(text, _)| !text.starts_with("CALL ")));
}

#[tokio::test]
async fn rejects_missing() {
    let command = MockCommand::new();
    *command.reader_rows.lock().expect("mock") = vec![];
    let app = App::new(&command, "sqlite");
    assert!(app
        .run(CliCommand::EnableUser(EnableUser {
            username: "missing".into()
        }))
        .await
        .is_err());
    *command.reader_rows.lock().expect("mock") = vec![row(&[("id", Value::Text("x".into()))])];
    assert!(app
        .run(CliCommand::EnableUser(EnableUser {
            username: "bad".into()
        }))
        .await
        .is_err());
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
    repo.create(&mut omitted).await.unwrap();
    let stored = repo
        .read_single_by_id(omitted.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    omitted.insert("username".into(), Value::from("ada-v2"));
    repo.update(&mut omitted, None).await.unwrap();
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
    repo.create(&mut explicit).await.unwrap();
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
