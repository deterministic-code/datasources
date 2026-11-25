use crate::identifier;
use datasources::{
    CreateIndexStatement, CreateTriggerStatement, ForeignKey, PrimaryKey, StatementNaming,
};
use datasources_code_generators::generate_sql;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, Once};

fn register_sql() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        datasources_code_generators::register_dialect(
            "mysql",
            Arc::new(crate::MysqlDriver),
        )
        .unwrap();
    });
}

#[test]
fn infers_a_reference_column_type_from_the_parent_id() {
    register_sql();
    let sql = generate_sql(
        &HashMap::from([(
            "types.yaml".into(),
            r#"
types:
  - gender:
      tags: [datasource_type]
      inherits: set
      fields:
        - name:
            type: string
            size: 128
  - contact:
      tags: [datasource_type]
      inherits: set
      fields:
        - gender_type_id:
            references: gender.id
"#
            .into(),
        )]),
        &HashMap::new(),
        &["mysql".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("0001_initial_up.sql"))
    .unwrap()
    .content;
    let contacts = Regex::new(r#"CREATE TABLE contacts \(([\s\S]*?)\);"#)
        .unwrap()
        .captures(&sql)
        .unwrap()[1]
        .to_string();
    assert!(Regex::new(r#"gender_type_id INT"#)
        .unwrap()
        .is_match(&contacts));
    assert!(!Regex::new(r#"`gender_type_id` TEXT"#)
        .unwrap()
        .is_match(&contacts));
}

#[test]
fn separates_table_and_trigger_with_datasource_batch_separator() {
    register_sql();
    let files = HashMap::from([(
        "types.yaml".into(),
        r#"
types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
            default_value: UtcNow
        - updated:
            type: datetime
            default_value: UtcNow
"#
        .into(),
    )]);
    let sql = generate_sql(&files, &HashMap::new(), &["mysql".into()])
        .unwrap()
        .into_iter()
        .find(|e| e.filename.ends_with("0001_initial_up.sql"))
        .unwrap()
        .content;
    assert!(Regex::new(r";\nGO\nCREATE TRIGGER trg_genders_updated_at")
        .unwrap()
        .is_match(&sql));
    let custom = generate_sql(
        &files,
        &HashMap::from([("datasource.batch_separator".into(), "BATCH".into())]),
        &["mysql".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("0001_initial_up.sql"))
    .unwrap()
    .content;
    assert!(Regex::new(r";\nBATCH\nCREATE TRIGGER trg_genders_updated_at")
        .unwrap()
        .is_match(&custom));
    assert!(!Regex::new(r"\nGO\n").unwrap().is_match(&custom));
}

#[test]
fn separates_seed_inserts_with_datasource_batch_separator() {
    register_sql();
    let sql = generate_sql(
        &HashMap::from([
            (
                "types.yaml".into(),
                r#"
types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
"#
                .into(),
            ),
            (
                "datasource_seeds.yaml".into(),
                r#"
version: 1.0.0
seeds:
  - gender:
      - id1:
          name: Male
      - id2:
          name: Female
"#
                .into(),
            ),
        ]),
        &HashMap::new(),
        &["mysql".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("0001_initial_up.sql"))
    .unwrap()
    .content;
    assert!(Regex::new(r"INSERT INTO genders[\s\S]*;\nGO\nINSERT INTO genders")
        .unwrap()
        .is_match(&sql));
}

#[test]
fn shortens_index_foreign_key_and_trigger_names_over_64_characters_in_migrations() {
    register_sql();
    let entity = "game_journey_setting_category_preference_option_value";
    let naming = StatementNaming::default();
    let index = CreateIndexStatement::name_of(entity, &naming, &["category_type_id", "index"]);
    let foreign_key = ForeignKey::name_of(entity, &naming, &["category_type_id", "foreign_key"]);
    let trigger = CreateTriggerStatement::name_of(entity, &naming);
    let primary_key = PrimaryKey::name_of(entity, &naming, &["primary_key"]);
    for name in [&index, &foreign_key, &trigger, &primary_key] {
        assert!(name.len() > 64, "{name}");
    }
    let files = HashMap::from([
        (
            "types.yaml".into(),
            format!(
                r#"
types:
  - category_type:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - {entity}:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
        - updated:
            type: datetime
        - category_type_id:
            type: integer
            references: category_type.id
        - external_reference_code:
            type: string
"#
            ),
        ),
        (
            "datasource.yaml".into(),
            format!(
                r#"
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - {entity}:
      indexes:
        - category_type_id:
            fields:
              - category_type_id
"#
            ),
        ),
    ]);
    let entries = generate_sql(&files, &HashMap::new(), &["mysql".into()]).unwrap();
    let up = entries
        .iter()
        .find(|e| e.filename.ends_with("mysql/migrations/0001_initial_up.sql"))
        .unwrap()
        .content
        .clone();
    let down = entries
        .iter()
        .find(|e| e.filename.ends_with("mysql/migrations/0001_initial_down.sql"))
        .unwrap()
        .content
        .clone();
    let index_name = identifier::fit(&index);
    let foreign_name = identifier::fit(&foreign_key);
    let trigger_name = identifier::fit(&trigger);
    let primary_name = identifier::fit(&primary_key);
    assert!(index_name.len() <= 64);
    assert_ne!(index_name, index);
    let ident = |name: &str| format!(r"(?:`{name}`|{name})");
    assert!(Regex::new(&format!(r"CREATE INDEX {} ON", ident(&index_name)))
        .unwrap()
        .is_match(&up));
    assert!(Regex::new(&format!(r"CONSTRAINT {} FOREIGN KEY", ident(&foreign_name)))
        .unwrap()
        .is_match(&up));
    assert!(Regex::new(&format!(r"CONSTRAINT {} PRIMARY KEY", ident(&primary_name)))
        .unwrap()
        .is_match(&up));
    assert!(Regex::new(&format!(r"CREATE TRIGGER {}", ident(&trigger_name)))
        .unwrap()
        .is_match(&up));
    for name in [&index, &foreign_key, &trigger, &primary_key] {
        assert!(!up.contains(name), "{name}");
        assert!(!down.contains(name), "{name}");
    }
    assert!(up.contains(&index_name));
    assert!(up.contains(&foreign_name));
    assert!(up.contains(&trigger_name));
    assert!(up.contains(&primary_name));
}
