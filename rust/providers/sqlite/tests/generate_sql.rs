use datasources_code_generators::{generate_sql, SqlGenerateDriver};
use std::sync::Once;

fn register_sql() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        datasources_code_generators::register_dialect(
            "sqlite",
            SqlGenerateDriver {
                quote_left: crate::SqliteDriver::QUOTE_LEFT,
                quote_right: crate::SqliteDriver::QUOTE_RIGHT,
                type_converter: crate::SqliteDriver::types(),
                default_value_converter: crate::SqliteDriver::defaults(),
                statement_renderer: crate::SqliteDriver::statement_text_renderer(),
            },
        )
        .unwrap();
    });
}
use regex::Regex;
use std::collections::HashMap;

const TYPES_YAML: &str = r#"
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
"#;

fn files(types: &str, datasource: Option<&str>, seeds: Option<&str>) -> HashMap<String, String> {
    register_sql();
    let mut files = HashMap::from([("types.yaml".into(), types.to_string())]);
    if let Some(ds) = datasource {
        files.insert("datasource.yaml".into(), ds.into());
    }
    if let Some(seeds) = seeds {
        files.insert("datasource_seeds.yaml".into(), seeds.into());
    }
    files
}

fn names(entries: &[datasources_code_generators::GenerateEntry]) -> Vec<String> {
    let mut names: Vec<_> = entries.iter().map(|e| e.filename.clone()).collect();
    names.sort();
    names
}

#[test]
fn omits_stored_procedure_migrations_on_sqlite_even_when_the_flag_is_true() {
    let files = names(
        &generate_sql(
            &files(TYPES_YAML, None, None),
            &HashMap::from([("datasource.use_stored_procedures".into(), "true".into())]),
            &["sqlite".into()],
        )
        .unwrap(),
    );
    assert!(files.iter().any(|f| f == "sqlite/migrations/0001_initial_up.sql"));
    assert!(!files.iter().any(|f| f.contains("stored_procedures")));
}

fn up_sql(settings: HashMap<String, String>, types: &str) -> String {
    let entries = generate_sql(&files(types, None, None), &settings, &["sqlite".into()]).unwrap();
    entries
        .into_iter()
        .find(|entry| entry.filename.ends_with("sqlite/migrations/0001_initial_up.sql"))
        .expect("expected sqlite/migrations/0001_initial_up.sql")
        .content
}

#[test]
fn defaults_to_a_whole_line_go_between_table_and_trigger() {
    let sql = up_sql(HashMap::new(), TYPES_YAML);
    assert!(Regex::new(r#";\nGO\nCREATE TRIGGER "trg_users_updated_at""#)
        .unwrap()
        .is_match(&sql));
}

#[test]
fn puts_go_between_tables() {
    let sql = up_sql(
        HashMap::new(),
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
        - updated:
            type: datetime
  - contact:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
        - updated:
            type: datetime
"#,
    );
    assert!(Regex::new(r#"trg_genders_updated_at[\s\S]*;\nGO\nCREATE TABLE "contacts""#)
        .unwrap()
        .is_match(&sql));
}

#[test]
fn uses_the_configured_separator() {
    let sql = up_sql(
        HashMap::from([("datasource.batch_separator".into(), "BATCH".into())]),
        TYPES_YAML,
    );
    assert!(Regex::new(r#";\nBATCH\nCREATE TRIGGER "trg_users_updated_at""#)
        .unwrap()
        .is_match(&sql));
    assert!(!Regex::new(r"\nGO\n").unwrap().is_match(&sql));
}

#[test]
fn separates_seed_inserts_with_datasource_batch_separator() {
    let sql = generate_sql(
        &files(
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
"#,
            None,
            Some(
                r#"
version: 1.0.0
seeds:
  - gender:
      - id1:
          name: Male
      - id2:
          name: Female
"#,
            ),
        ),
        &HashMap::new(),
        &["sqlite".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("sqlite/migrations/0001_initial_up.sql"))
    .unwrap()
    .content;
    assert!(Regex::new(r#"INSERT INTO "genders"[\s\S]*;\nGO\nINSERT INTO "genders""#)
        .unwrap()
        .is_match(&sql));
}

#[test]
fn throws_when_id_is_missing_is_id() {
    let error = generate_sql(
        &files(
            r#"
types:
  - card:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
        - label:
            type: string
"#,
            None,
            None,
        ),
        &HashMap::new(),
        &["sqlite".into()],
    )
    .unwrap_err();
    assert!(error.to_string().contains("card.id must have is_id: true"));
}

#[test]
fn defaults_uuid_created_updated_and_version_on_a_base_table() {
    let sql = generate_sql(
        &files(
            r#"
types:
  - base:
      fields:
        - id:
            type: integer
            is_id: true
        - uuid:
            type: uuid
            default_value: NewId
        - created:
            type: datetime
            default_value: UtcNow
        - updated:
            type: datetime
            default_value: UtcNow
        - version:
            type: binary
            default_value: Hex('00')
  - card:
      tags: [datasource_type]
      inherits: base
      fields:
        - label:
            type: string
"#,
            None,
            None,
        ),
        &HashMap::new(),
        &["sqlite".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("0001_initial_up.sql"))
    .unwrap()
    .content;
    let cards = Regex::new(r#"CREATE TABLE "cards" \(([\s\S]*?)\);"#)
        .unwrap()
        .captures(&sql)
        .unwrap()[1]
        .to_string();
    assert!(Regex::new(r#""uuid" TEXT .*DEFAULT \(lower\(hex\(randomblob"#)
        .unwrap()
        .is_match(&cards));
    assert!(Regex::new(r#""created" TEXT .*strftime\('%Y-%m-%dT%H:%M:%fZ'"#)
        .unwrap()
        .is_match(&cards));
    assert!(Regex::new(r#""updated" TEXT .*strftime\('%Y-%m-%dT%H:%M:%fZ'"#)
        .unwrap()
        .is_match(&cards));
    assert!(Regex::new(r#""version" BLOB .*DEFAULT X'00'"#)
        .unwrap()
        .is_match(&cards));
}

#[test]
fn infers_a_reference_column_type_from_the_parent_id() {
    let sql = generate_sql(
        &files(
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
"#,
            None,
            None,
        ),
        &HashMap::new(),
        &["sqlite".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("0001_initial_up.sql"))
    .unwrap()
    .content;
    let contacts = Regex::new(r#"CREATE TABLE "contacts" \(([\s\S]*?)\);"#)
        .unwrap()
        .captures(&sql)
        .unwrap()[1]
        .to_string();
    assert!(Regex::new(r#""gender_type_id" INTEGER"#)
        .unwrap()
        .is_match(&contacts));
    assert!(!Regex::new(r#""gender_type_id" TEXT"#)
        .unwrap()
        .is_match(&contacts));
}

#[test]
fn omits_view_collections_from_table_columns_and_keeps_the_child_fk() {
    let sql = generate_sql(
        &files(
            r#"
types:
  - contact:
      tags: [datasource_type, view_type]
      inherits: set
      fields:
        - email:
            type: string
        - addresses:
            type: address[]
            references: address.contact_id
  - address:
      tags: [datasource_type, view_type]
      inherits: set
      fields:
        - contact_id:
            type: number
            references: contact.id
        - city:
            type: string
"#,
            None,
            None,
        ),
        &HashMap::new(),
        &["sqlite".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("0001_initial_up.sql"))
    .unwrap()
    .content;
    assert!(Regex::new(r#"CREATE TABLE "contacts""#).unwrap().is_match(&sql));
    assert!(Regex::new(r#"CREATE TABLE "addresses""#).unwrap().is_match(&sql));
    let contacts = Regex::new(r#"CREATE TABLE "contacts" \(([\s\S]*?)\);"#)
        .unwrap()
        .captures(&sql)
        .unwrap()[1]
        .to_string();
    assert!(contacts.contains("\"email\""));
    assert!(!contacts.contains("\"addresses\""));
    assert!(sql.contains("\"contact_id\""));
    assert!(Regex::new(r#"FOREIGN KEY \("contact_id"\) REFERENCES "contacts""#)
        .unwrap()
        .is_match(&sql));
}

#[test]
fn emits_a_dictionary_table_with_unique_owner_and_key_flattening_object_values() {
    let sql = generate_sql(
        &files(
            r#"
types:
  - contact:
      tags: [datasource_type]
      inherits: set
      fields:
        - email:
            type: string
  - locale_preferences:
      tags: [view_type]
      fields:
        - locale:
            type: string
        - timezone:
            type: string
  - contact_settings:
      tags: [datasource_type]
      inherits: set
      fields:
        - contact_id:
            type: integer
            references: contact.id
        - key:
            type: string
        - value:
            type: string
  - contact_preferences:
      tags: [datasource_type]
      inherits: set
      fields:
        - contact_id:
            type: integer
            references: contact.id
        - key:
            type: string
        - locale:
            type: string
        - timezone:
            type: string
  - contact_settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
        - value:
            type: string
  - contact_preferences_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
        - value:
            type: locale_preferences
  - contact_view:
      tags: [view_type]
      inherits: contact
      fields:
        - settings:
            type: contact_settings_map{}
            references: contact_settings.contact_id
        - prefs:
            type: contact_preferences_map{}
            references: contact_preferences.contact_id
  - phone:
      tags: [datasource_type]
      inherits: set
  - phone_attribute:
      tags: [datasource_type]
      inherits: set
      fields:
        - phone_id:
            type: integer
            references: phone.id
        - attribute_name:
            type: string
        - attribute_value:
            type: string
  - phone_attribute_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: phone_attribute.attribute_name
        - value:
            type: string
            references: phone_attribute.attribute_value
  - phone_view:
      tags: [view_type]
      inherits: phone
      fields:
        - attributes:
            type: phone_attribute_map{}
            references: phone_attribute.phone_id
  - card_labels:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
        - value:
            type: string
"#,
            None,
            None,
        ),
        &HashMap::new(),
        &["sqlite".into()],
    )
    .unwrap()
    .into_iter()
    .find(|e| e.filename.ends_with("0001_initial_up.sql"))
    .unwrap()
    .content;
    assert!(sql.contains("CREATE TABLE \"contact_settings\""));
    assert!(sql.contains("CREATE TABLE \"contact_preferences\""));
    assert!(!sql.contains("CREATE TABLE \"card_labels\""));
    let settings = Regex::new(r#"CREATE TABLE "contact_settings" \(([\s\S]*?)\);"#)
        .unwrap()
        .captures(&sql)
        .unwrap()[1]
        .to_string();
    assert!(settings.contains("\"contact_id\""));
    assert!(settings.contains("\"key\""));
    assert!(settings.contains("\"value\""));
    assert!(Regex::new(r#"(?m)^\s*"id""#).unwrap().is_match(&settings));
    let prefs = Regex::new(r#"CREATE TABLE "contact_preferences" \(([\s\S]*?)\);"#)
        .unwrap()
        .captures(&sql)
        .unwrap()[1]
        .to_string();
    assert!(prefs.contains("\"locale\""));
    assert!(prefs.contains("\"timezone\""));
    assert!(!prefs.contains("\"value\""));
    assert!(Regex::new(r#"CREATE UNIQUE INDEX .* ON "contact_settings""#)
        .unwrap()
        .is_match(&sql));
    assert!(Regex::new(r#"CREATE UNIQUE INDEX .* ON "contact_preferences""#)
        .unwrap()
        .is_match(&sql));
    assert!(
        Regex::new(r#"CREATE UNIQUE INDEX .* ON "phone_attributes".*"phone_id".*"attribute_name""#)
            .unwrap()
            .is_match(&sql)
    );
}

fn frozen_diff(types_yaml: &str) -> String {
    let types: serde_yaml::Value = serde_yaml::from_str(types_yaml).unwrap();
    let mut envelope = serde_yaml::Mapping::new();
    envelope.insert(serde_yaml::Value::String("added".into()), serde_yaml::Value::Bool(true));
    envelope.insert(serde_yaml::Value::String("content".into()), types);
    let mut doc = serde_yaml::Mapping::new();
    doc.insert(
        serde_yaml::Value::String("types.yaml".into()),
        serde_yaml::Value::Mapping(envelope),
    );
    serde_yaml::to_string(&serde_yaml::Value::Mapping(doc)).unwrap()
}

#[test]
fn emits_numbered_up_down_files_from_frozen_diffs() {
    let path = "version-20261016T000000Z-001/diff.yaml";
    let mut files = files(TYPES_YAML, None, None);
    files.insert(path.into(), frozen_diff(TYPES_YAML));
    let names = names(&generate_sql(&files, &HashMap::new(), &["sqlite".into()]).unwrap());
    assert_eq!(
        names,
        vec![
            "sqlite/migrations/0001_down.sql".to_string(),
            "sqlite/migrations/0001_up.sql".to_string()
        ]
    );
}

#[test]
fn throws_when_live_yaml_drifted_from_frozen_diffs() {
    let path = "version-20261016T000000Z-001/diff.yaml";
    let mut files = files(TYPES_YAML, None, None);
    files.insert(path.into(), frozen_diff(TYPES_YAML));
    files.insert(
        "types.yaml".into(),
        r#"
types:
  - extra:
      tags: [datasource_type]
      inherits: set
      fields:
        - name:
            type: string
"#
        .into(),
    );
    let error = generate_sql(&files, &HashMap::new(), &["sqlite".into()]).unwrap_err();
    assert!(error.to_string().contains("run datasources version first"));
}
