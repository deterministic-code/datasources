use datasources_code_generators::generate_sql;
use std::sync::{Arc, Once};

fn register_sql() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        datasources_code_generators::register_dialect(
            "postgres",
            Arc::new(crate::PostgresDriver),
            crate::PostgresDriver::apply_keywords,
        )
        .unwrap();
    });
}
use regex::Regex;
use std::collections::HashMap;

const TYPES: &str = r#"
types:
  - users_base:
      tags: [datasource_type]
      inherits: set
      fields:
        - email:
            type: string
        - created:
            type: datetime
        - updated:
            type: datetime
  - child:
      tags: [datasource_type]
      inherits: set
      fields:
        - owner_id:
            type: integer
            references: users_base.id
        - email:
            type: string
        - created:
            type: datetime
        - updated:
            type: datetime
  - person:
      tags: [datasource_type]
      inherits: set
      fields:
        - name:
            type: string
  - old_reviews:
      tags: [datasource_type]
      inherits: set
      fields:
        - title:
            type: string
  - notification:
      tags: [datasource_type]
      inherits: set
      fields:
        - title:
            type: string
"#;

const DATASOURCE: &str = r#"
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - users_base:
      mapping: user
  - child:
      fields:
        - email:
            mapping: email_address
  - old_reviews:
      mapping: OldRvwsTbl
"#;

const SEEDS: &str = r#"
seeds:
  - child:
      - id1:
          email: mapped@example.com
          owner_id: 1
"#;

fn up_sql(settings: &HashMap<String, String>) -> String {
    register_sql();
    generate_sql(
        &HashMap::from([
            ("types.yaml".into(), TYPES.into()),
            ("datasource.yaml".into(), DATASOURCE.into()),
            ("datasource_seeds.yaml".into(), SEEDS.into()),
        ]),
        settings,
        &["postgres".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("0001_initial_up.sql"))
    .unwrap()
    .content
}

#[test]
fn pluralizes_a_snake_table_mapping() {
    let sql = up_sql(&HashMap::new());
    assert!(sql.contains("CREATE TABLE \"users\""));
    assert!(sql.contains("CONSTRAINT \"users_primary_key\""));
    assert!(sql.contains("trg_users_updated_at"));
    assert!(!sql.contains("users_base"));
    assert!(!sql.contains("users_bases"));
}

#[test]
fn keeps_a_snake_table_mapping_singular_when_pluralize_is_off() {
    let sql = up_sql(&HashMap::from([(
        "datasource.pluralize_datatable_names".into(),
        "false".into(),
    )]));
    assert!(sql.contains("CREATE TABLE \"user\""));
    assert!(sql.contains("CONSTRAINT \"user_primary_key\""));
    assert!(Regex::new(r#"REFERENCES "user"\s*\("id"\)"#).unwrap().is_match(&sql));
}

#[test]
fn pluralizes_an_unmapped_type_name() {
    let sql = up_sql(&HashMap::new());
    assert!(sql.contains("CREATE TABLE \"people\""));
    assert!(sql.contains("CREATE TABLE \"notifications\""));
}

#[test]
fn emits_mapped_columns_and_seed_insert_lists() {
    let sql = up_sql(&HashMap::new());
    assert!(Regex::new(r#"CREATE TABLE "children"[\s\S]*"email_address""#)
        .unwrap()
        .is_match(&sql));
    assert!(Regex::new(r#"INSERT INTO "children" \("id", "email_address", "owner_id"\)"#)
        .unwrap()
        .is_match(&sql));
    assert!(!Regex::new(r#"CREATE TABLE "children"[\s\S]*"email"[^_]"#)
        .unwrap()
        .is_match(&sql));
}

#[test]
fn resolves_logical_fk_references_to_the_target_physical_name() {
    assert!(Regex::new(r#"REFERENCES "users"\s*\("id"\)"#)
        .unwrap()
        .is_match(&up_sql(&HashMap::new())));
}

#[test]
fn keeps_a_non_snake_mapping_verbatim_when_pluralize_is_on() {
    let sql = up_sql(&HashMap::new());
    assert!(sql.contains("CREATE TABLE \"OldRvwsTbl\""));
    assert!(!sql.contains("OldRvwsTbls"));
}
