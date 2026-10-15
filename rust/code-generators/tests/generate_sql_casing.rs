mod helpers;

use datasources_code_generators::generate_sql;
use regex::Regex;
use std::collections::HashMap;

const FIXTURE: &str = r#"
types:
  - user:
      tags: [datasource_type]
      inherits: set
      fields:
        - email:
            type: string
        - created:
            type: datetime
        - updated:
            type: datetime
  - notification_type:
      tags: [datasource_type]
      inherits: set
      fields:
        - channel_name:
            type: string
        - user_id:
            type: integer
            references: user.id
        - created:
            type: datetime
            default_value: UtcNow
        - updated:
            type: datetime
            default_value: UtcNow
"#;

const SEEDS: &str = r#"
seeds:
  - user:
      - id1:
          email: CREATE
"#;

fn files(with_seeds: bool) -> HashMap<String, String> {
    helpers::register_sql();
    let mut files = HashMap::from([("types.yaml".into(), FIXTURE.to_string())]);
    if with_seeds {
        files.insert("datasource_seeds.yaml".into(), SEEDS.into());
    }
    files
}

fn file_sql(
    filename: &str,
    settings: &HashMap<String, String>,
    with_seeds: bool,
    dialects: &[&str],
) -> String {
    generate_sql(
        &files(with_seeds),
        settings,
        &dialects.iter().map(|d| (*d).to_string()).collect::<Vec<_>>(),
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename == filename)
    .unwrap()
    .content
}

fn up_sql(settings: &HashMap<String, String>) -> String {
    file_sql("postgres/migrations/0001_initial_up.sql", settings, false, &["postgres"])
}

fn proc_sql(settings: &HashMap<String, String>) -> String {
    let mut merged = settings.clone();
    merged.insert("datasource.use_stored_procedures".into(), "true".into());
    file_sql(
        "postgres/migrations/0002_stored_procedures_up.sql",
        &merged,
        false,
        &["postgres"],
    )
}

#[test]
fn auto_plus_pluralize_uses_snake_plural_tables_and_snake_columns() {
    let sql = up_sql(&HashMap::new());
    assert!(sql.contains("CREATE TABLE \"notification_types\""));
    assert!(sql.contains("CREATE TABLE \"users\""));
    assert!(sql.contains("\"channel_name\""));
    assert!(sql.contains("\"notification_types_user_id_foreign_key\""));
    assert!(Regex::new(r#"REFERENCES "users"\s*\("id"\)"#).unwrap().is_match(&sql));
}

#[test]
fn types_casing_matrix() {
    let cases = [
        ("Camel", "on", "notificationTypes", "users", "notificationTypesUserIdForeignKey", "createNotificationType", "findNotificationTypes"),
        ("Pascal", "on", "NotificationTypes", "Users", "NotificationTypesUserIdForeignKey", "CreateNotificationType", "FindNotificationTypes"),
        ("Snake", "on", "notification_types", "users", "notification_types_user_id_foreign_key", "create_notification_type", "find_notification_types"),
        ("Kebab", "on", "notification-types", "users", "notification-types-user-id-foreign-key", "create-notification-type", "find-notification-types"),
        ("Camel", "off", "notificationType", "user", "notificationTypeUserIdForeignKey", "createNotificationType", "findNotificationTypes"),
        ("Pascal", "off", "NotificationType", "User", "NotificationTypeUserIdForeignKey", "CreateNotificationType", "FindNotificationTypes"),
        ("Snake", "off", "notification_type", "user", "notification_type_user_id_foreign_key", "create_notification_type", "find_notification_types"),
        ("Kebab", "off", "notification-type", "user", "notification-type-user-id-foreign-key", "create-notification-type", "find-notification-types"),
    ];
    for (format, pluralize, notification, user, constraint, create_fn, find_all) in cases {
        let mut settings = HashMap::from([("datasource.casing.types".into(), format.into())]);
        if pluralize == "off" {
            settings.insert("datasource.pluralize_datatable_names".into(), "false".into());
        }
        let sql = up_sql(&settings);
        assert!(sql.contains(&format!("CREATE TABLE \"{notification}\"")), "{format} {pluralize}");
        assert!(sql.contains(&format!("CREATE TABLE \"{user}\"")), "{format} {pluralize}");
        assert!(sql.contains("\"channel_name\""));
        assert!(sql.contains(&format!("\"{constraint}\"")));
        assert!(Regex::new(&format!(r#"REFERENCES "{user}"\s*\("id"\)"#)).unwrap().is_match(&sql));
        if pluralize == "off" {
            assert!(!sql.contains("CREATE TABLE \"notification_types\""));
        }
        let procs = proc_sql(&settings);
        assert!(Regex::new(&format!(r#"CREATE OR REPLACE FUNCTION {create_fn}\("#))
            .unwrap()
            .is_match(&procs));
        assert!(Regex::new(&format!(r#"CREATE OR REPLACE FUNCTION {find_all}\("#))
            .unwrap()
            .is_match(&procs));
        assert!(procs.contains(&format!("INSERT INTO \"{notification}\"")));
    }
}

#[test]
fn fields_casing_columns_only() {
    let cases = [
        ("Camel", "channelName", "userId", "id"),
        ("Pascal", "ChannelName", "UserId", "Id"),
        ("Snake", "channel_name", "user_id", "id"),
        ("Kebab", "channel-name", "user-id", "id"),
    ];
    for (format, channel, user_id, id) in cases {
        let sql = up_sql(&HashMap::from([("datasource.casing.fields".into(), format.into())]));
        assert!(sql.contains("CREATE TABLE \"notification_types\""));
        assert!(sql.contains(&format!("\"{channel}\"")));
        assert!(sql.contains(&format!("\"{user_id}\"")));
        assert!(Regex::new(&format!(r#"REFERENCES "users"\s*\("{id}"\)"#))
            .unwrap()
            .is_match(&sql));
    }
}

#[test]
fn pascal_types_and_camel_fields_apply_independently() {
    let settings = HashMap::from([
        ("datasource.casing.types".into(), "Pascal".into()),
        ("datasource.casing.fields".into(), "Camel".into()),
    ]);
    let sql = up_sql(&settings);
    assert!(sql.contains("CREATE TABLE \"NotificationTypes\""));
    assert!(sql.contains("\"channelName\""));
    assert!(sql.contains("\"NotificationTypesUserIdForeignKey\""));
    assert!(Regex::new(r#"REFERENCES "Users"\s*\("id"\)"#).unwrap().is_match(&sql));
    let procs = proc_sql(&settings);
    assert!(Regex::new(r#"CREATE OR REPLACE FUNCTION CreateNotificationType\("#)
        .unwrap()
        .is_match(&procs));
    assert!(procs.contains("channelName"));
}

#[test]
fn languages_sql_casing_does_not_change_emit() {
    let sql = up_sql(&HashMap::from([
        ("languages.sql.casing.types".into(), "Pascal".into()),
        ("languages.sql.casing.fields".into(), "Camel".into()),
        ("languages.sql.casing.keywords".into(), "lower".into()),
    ]));
    assert!(sql.contains("CREATE TABLE \"notification_types\""));
    assert!(sql.contains("\"channel_name\""));
    assert!(!sql.contains("CREATE TABLE \"NotificationTypes\""));
    assert!(!sql.contains("create table"));
}

#[test]
fn keywords_lower_lowercases_sql_keywords_and_types() {
    let settings = HashMap::from([("datasource.casing.keywords".into(), "lower".into())]);
    let sql = up_sql(&settings);
    assert!(sql.contains("create table \"notification_types\""));
    assert!(sql.contains("not null"));
    assert!(Regex::new(r#"references "users"\s*\("id"\)"#).unwrap().is_match(&sql));
    assert!(sql.contains("\"email\" text not null"));
    assert!(!sql.contains("CREATE TABLE"));
    let procs = proc_sql(&settings);
    assert!(Regex::new(r#"create or replace function create_notification_type\("#)
        .unwrap()
        .is_match(&procs));
    assert!(!procs.contains("CREATE OR REPLACE"));
}

#[test]
fn objects_upper_screams_table_constraint_and_routine_names() {
    let settings = HashMap::from([("datasource.casing.objects".into(), "upper".into())]);
    let sql = up_sql(&settings);
    assert!(sql.contains("CREATE TABLE \"NOTIFICATION_TYPES\""));
    assert!(sql.contains("CREATE TABLE \"USERS\""));
    assert!(sql.contains("\"channel_name\""));
    assert!(!sql.contains("CREATE TABLE \"notification_types\""));
    let procs = proc_sql(&settings);
    assert!(Regex::new(r#"CREATE OR REPLACE FUNCTION CREATE_NOTIFICATION_TYPE\("#)
        .unwrap()
        .is_match(&procs));
}

#[test]
fn throws_when_keywords_or_objects_is_unknown() {
    let keywords = generate_sql(
        &files(false),
        &HashMap::from([("datasource.casing.keywords".into(), "screaming".into())]),
        &["postgres".into()],
    )
    .unwrap_err()
    .to_string();
    assert!(Regex::new(r#"datasource\.casing\.keywords must be one of \[upper, lower\]"#)
        .unwrap()
        .is_match(&keywords));
    let objects = generate_sql(
        &files(false),
        &HashMap::from([("datasource.casing.objects".into(), "screaming".into())]),
        &["postgres".into()],
    )
    .unwrap_err()
    .to_string();
    assert!(Regex::new(r#"datasource\.casing\.objects must be one of \[upper, lower\]"#)
        .unwrap()
        .is_match(&objects));
}

#[test]
fn leaves_seed_string_literals_when_keywords_are_lower() {
    let sql = file_sql(
        "postgres/migrations/0001_initial_up.sql",
        &HashMap::from([("datasource.casing.keywords".into(), "lower".into())]),
        true,
        &["postgres"],
    );
    assert!(sql.contains("insert into \"users\""));
    assert!(sql.contains("'CREATE'"));
    assert!(!sql.contains("'create'"));
}

#[test]
fn keywords_lower_keeps_quoted_object_names() {
    for (dialect, left, right) in [
        ("postgres", "\"", "\""),
        ("sqlite", "\"", "\""),
        ("mysql", "`", "`"),
        ("sqlserver", "[", "]"),
        ("oracle", "\"", "\""),
    ] {
        let body = file_sql(
            &format!("{dialect}/migrations/0001_initial_up.sql"),
            &HashMap::from([
                ("datasource.casing.keywords".into(), "lower".into()),
                ("datasource.casing.objects".into(), "upper".into()),
            ]),
            false,
            &[dialect],
        );
        assert!(
            body.contains(&format!("create table {left}NOTIFICATION_TYPES{right}")),
            "{dialect}"
        );
        assert!(body.contains("not null"), "{dialect}");
    }
}
