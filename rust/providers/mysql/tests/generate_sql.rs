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
    let contacts = Regex::new(r#"CREATE TABLE `contacts` \(([\s\S]*?)\);"#)
        .unwrap()
        .captures(&sql)
        .unwrap()[1]
        .to_string();
    assert!(Regex::new(r#"`gender_type_id` INT"#)
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
    assert!(Regex::new(r";\nGO\nCREATE TRIGGER `trg_genders_updated_at`")
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
    assert!(Regex::new(r";\nBATCH\nCREATE TRIGGER `trg_genders_updated_at`")
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
    assert!(Regex::new(r"INSERT INTO `genders`[\s\S]*;\nGO\nINSERT INTO `genders`")
        .unwrap()
        .is_match(&sql));
}
