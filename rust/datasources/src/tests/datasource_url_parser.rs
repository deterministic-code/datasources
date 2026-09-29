use crate::*;

#[test]
fn postgres_url() {
    let url = DatasourceUrlParser::parse(
        "postgres://ds:ds@127.0.0.1:5432/datasources?schema=public&sslmode=require",
    )
    .unwrap();
    assert_eq!(url.provider, "postgres");
    assert_eq!(url.username.as_deref(), Some("ds"));
    assert_eq!(url.password.as_deref(), Some("ds"));
    assert_eq!(url.host.as_deref(), Some("127.0.0.1"));
    assert_eq!(url.port, Some(5432));
    assert_eq!(url.path, "/datasources");
    assert_eq!(url.datasource(), Some("datasources"));
    assert_eq!(url.schema.as_deref(), Some("public"));
    assert_eq!(url.options.get("sslmode").map(String::as_str), Some("require"));
}

#[test]
fn postgres_user_without_password() {
    let url = DatasourceUrlParser::parse("postgres://ds@127.0.0.1/datasources").unwrap();
    assert_eq!(url.username.as_deref(), Some("ds"));
    assert!(url.password.is_none());
    assert!(url.port.is_none());
    assert!(url.schema.is_none());
    assert!(url.options.is_empty());
}

#[test]
fn schema_only() {
    let url = DatasourceUrlParser::parse("postgres://127.0.0.1/datasources?schema=public").unwrap();
    assert_eq!(url.schema.as_deref(), Some("public"));
    assert!(url.options.is_empty());
}

#[test]
fn options_only() {
    let url = DatasourceUrlParser::parse("postgres://127.0.0.1/datasources?sslmode=require").unwrap();
    assert!(url.schema.is_none());
    assert_eq!(url.options.get("sslmode").map(String::as_str), Some("require"));
}

#[test]
fn repeated_option_keeps_the_last_value() {
    let url = DatasourceUrlParser::parse(
        "postgres://127.0.0.1/datasources?sslmode=require&sslmode=disable",
    )
    .unwrap();
    assert_eq!(url.options.get("sslmode").map(String::as_str), Some("disable"));
}

#[test]
fn flag_option_is_empty() {
    let url = DatasourceUrlParser::parse("postgres://127.0.0.1/datasources?readonly").unwrap();
    assert_eq!(url.options.get("readonly").map(String::as_str), Some(""));
}

#[test]
fn mysql_url() {
    let url = DatasourceUrlParser::parse("mysql://ds:secret@127.0.0.1:3306/app").unwrap();
    assert_eq!(url.provider, "mysql");
    assert_eq!(url.username.as_deref(), Some("ds"));
    assert_eq!(url.password.as_deref(), Some("secret"));
    assert_eq!(url.host.as_deref(), Some("127.0.0.1"));
    assert_eq!(url.port, Some(3306));
    assert_eq!(url.datasource(), Some("app"));
}

#[test]
fn mysql_host_and_datasource() {
    let url = DatasourceUrlParser::parse("mysql://db/app").unwrap();
    assert_eq!(url.host.as_deref(), Some("db"));
    assert_eq!(url.datasource(), Some("app"));
    assert!(url.username.is_none());
    assert!(url.port.is_none());
}

#[test]
fn sqlite_relative_file() {
    let url = DatasourceUrlParser::parse("sqlite:///app.db").unwrap();
    assert_eq!(url.provider, "sqlite");
    assert_eq!(url.path, "/app.db");
    assert_eq!(url.datasource(), Some("app.db"));
    assert!(url.host.is_none());
}

#[test]
fn sqlite_absolute_file() {
    let url = DatasourceUrlParser::parse("sqlite:////tmp/app.db").unwrap();
    assert_eq!(url.path, "//tmp/app.db");
    assert_eq!(url.datasource(), Some("/tmp/app.db"));
}

#[test]
fn sqlite_memory() {
    let url = DatasourceUrlParser::parse("sqlite:///:memory:").unwrap();
    assert_eq!(url.provider, "sqlite");
    assert_eq!(url.path, "/:memory:");
    assert_eq!(url.datasource(), Some(":memory:"));
}

#[test]
fn sqlite_memory_without_path() {
    let url = DatasourceUrlParser::parse("sqlite://:memory:").unwrap();
    assert_eq!(url.host.as_deref(), Some(":memory:"));
    assert!(url.datasource().is_none());
}

#[test]
fn json_file() {
    let url = DatasourceUrlParser::parse("json://contacts.json").unwrap();
    assert_eq!(url.provider, "json");
    assert_eq!(url.datasource(), Some("contacts.json"));
    assert!(url.host.is_none());
}

#[test]
fn percent_encoded_password() {
    let url = DatasourceUrlParser::parse("postgres://ds:p%3Ax@127.0.0.1/datasources").unwrap();
    assert_eq!(url.password.as_deref(), Some("p:x"));
}

#[test]
fn password_contains_colon() {
    let url = DatasourceUrlParser::parse("postgres://ds:a:b@127.0.0.1/datasources").unwrap();
    assert_eq!(url.username.as_deref(), Some("ds"));
    assert_eq!(url.password.as_deref(), Some("a:b"));
}

#[test]
fn edges() {
    let upper = DatasourceUrlParser::parse("Postgres://127.0.0.1/db").unwrap();
    assert_eq!(upper.provider, "postgres");
    let opaque = DatasourceUrlParser::parse("a.b+c1:file").unwrap();
    assert_eq!(opaque.provider, "a.b+c1");
    assert_eq!(opaque.datasource(), Some("file"));
    let slash = DatasourceUrlParser::parse("postgres://127.0.0.1/").unwrap();
    assert_eq!(slash.path, "/");
    assert!(slash.datasource().is_none());
    let ipv6 = DatasourceUrlParser::parse("postgres://[::1]:5432/db").unwrap();
    assert_eq!(ipv6.host.as_deref(), Some("::1"));
    assert_eq!(ipv6.port, Some(5432));
    let host_only = DatasourceUrlParser::parse("postgres://[::1]/db").unwrap();
    assert_eq!(host_only.host.as_deref(), Some("::1"));
    assert!(host_only.port.is_none());
    let no_host = DatasourceUrlParser::parse("postgres://ds@/db").unwrap();
    assert!(no_host.host.is_none());
    let port_only = DatasourceUrlParser::parse("postgres://:5432/db").unwrap();
    assert!(port_only.host.is_none());
    assert_eq!(port_only.port, Some(5432));
    let password_only = DatasourceUrlParser::parse("postgres://:secret@h/db").unwrap();
    assert!(password_only.username.is_none());
    assert_eq!(password_only.password.as_deref(), Some("secret"));
    let fragment = DatasourceUrlParser::parse("postgres://h/db#frag").unwrap();
    assert_eq!(fragment.datasource(), Some("db"));
    let accented = DatasourceUrlParser::parse("postgres://dé@h/db").unwrap();
    assert_eq!(accented.username.as_deref(), Some("dé"));
    let encoded = DatasourceUrlParser::parse("postgres://d%C3%A9@h/db").unwrap();
    assert_eq!(encoded.username.as_deref(), Some("dé"));
    assert_eq!(DatasourceUrlParser::parse("   "), None);
    assert_eq!(DatasourceUrlParser::parse("9abc:file"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://ds:a%zz@h/db"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://[/db"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://[::1]x/db"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://[::1]:no/db"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://[%zz]/db"), None);
    let empty_user = DatasourceUrlParser::parse("postgres://@h/db").unwrap();
    assert!(empty_user.username.is_none());
    assert_eq!(empty_user.host.as_deref(), Some("h"));
    assert_eq!(DatasourceUrlParser::parse("postgres://ds:a%@h/db"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://%zz@h/db"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://%zz:5432/db"), None);
    assert_eq!(DatasourceUrlParser::parse("json://%zz"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://%zz/db"), None);
    assert_eq!(DatasourceUrlParser::parse("postgres://h/db?sslmode=%zz"), None);
    let query = DatasourceUrlParser::parse("postgres://h/db?&sslmode=on").unwrap();
    assert_eq!(query.options.get("sslmode").map(String::as_str), Some("on"));
}

#[test]
fn not_a_url() {
    let blank = DatasourceUrlParser::parse("");
    let text = DatasourceUrlParser::parse("not-a-url");
    let connection_string = DatasourceUrlParser::parse("Host=db;Port=1");
    assert_eq!(blank, None);
    assert_eq!(text, None);
    assert_eq!(connection_string, None);
}
