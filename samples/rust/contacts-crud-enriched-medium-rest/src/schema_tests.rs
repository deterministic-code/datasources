use crate::schema::{sql_path, statements, with_seed_version, GENERATED_SQL};

#[test]
fn loads_generated_sql() {
    let sql = statements(GENERATED_SQL);
    assert!(sql.iter().any(|item| item.starts_with("CREATE TABLE \"contacts\"")));
    assert!(sql.iter().any(|item| item.starts_with("CREATE TABLE \"contact_settings\"")));
    assert!(sql.iter().any(|item| item.starts_with("CREATE TABLE \"contact_preferences\"")));
    assert!(sql.iter().all(|item| !item.contains("CREATE TABLE \"bases\"")));
    assert!(sql.iter().any(|item| item.contains("INSERT INTO \"contact_sources\"")));
    assert!(sql.iter().all(|item| item.ends_with(';')));
}

#[test]
fn splits_comments_triggers_and_seeds() {
    assert_eq!(
        statements(
            r#"-- heading
CREATE TABLE "t" ("id" INTEGER);
CREATE TRIGGER "trg"
AFTER UPDATE ON "t"
BEGIN
  UPDATE "t" SET "id" = "id" WHERE "id" = OLD."id";
END;
INSERT INTO "t" ("id") VALUES (1);
"#
        ),
        vec![
            r#"CREATE TABLE "t" ("id" INTEGER);"#.to_string(),
            "CREATE TRIGGER \"trg\"\nAFTER UPDATE ON \"t\"\nBEGIN\n  UPDATE \"t\" SET \"id\" = \"id\" WHERE \"id\" = OLD.\"id\";\nEND;".to_string(),
            r#"INSERT INTO "t" ("id", "version") VALUES (1, X'00');"#.to_string(),
        ]
    );
}

#[test]
fn covers_seed_and_word_edges() {
    assert_eq!(with_seed_version("SELECT 1;"), "SELECT 1;");
    assert_eq!(
        with_seed_version(r#"INSERT INTO "t" ("id", "version") VALUES (1, X'00');"#),
        r#"INSERT INTO "t" ("id", "version") VALUES (1, X'00');"#
    );
    assert_eq!(
        with_seed_version(r#"INSERT INTO "t" id VALUES 1;"#),
        r#"INSERT INTO "t" id VALUES 1;"#
    );
    assert_eq!(
        with_seed_version("INSERT INTO `t` (`id`) VALUES (1);"),
        "INSERT INTO `t` (`id`, `version`) VALUES (1, X'00');"
    );
    assert_eq!(
        statements(
            r#"CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
  NEW."updated" = NOW() AT TIME ZONE 'UTC';
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TABLE "t" ("id" INTEGER);
"#
        ),
        vec![
            r#"CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
  NEW."updated" = NOW() AT TIME ZONE 'UTC';
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;"#
                .to_string(),
            r#"CREATE TABLE "t" ("id" INTEGER);"#.to_string(),
        ]
    );
    assert!(statements("BEGIN").is_empty());
    assert_eq!(statements(";"), vec![";"]);
    assert_eq!(statements("END;"), vec!["END;"]);
    assert_eq!(statements("XEND;"), vec!["XEND;"]);
    assert_eq!(statements("SELECT 1; \n;"), vec!["SELECT 1;", ";"]);
    assert_eq!(statements("END ;"), vec!["END ;"]);
    assert_eq!(
        statements(
            r#"BEGIN
  UPDATE "t" SET "id" = 1;
END;
CREATE TABLE "XBEGIN" ("id" INTEGER);
"#
        )
        .len(),
        2
    );
    assert!(sql_path(env!("CARGO_MANIFEST_DIR"))
        .unwrap()
        .to_string_lossy()
        .contains("0001_initial_up.sql"));
    let missing = std::env::temp_dir().join(format!("contacts-{}", std::process::id()));
    std::fs::create_dir_all(&missing).unwrap();
    assert!(sql_path(&missing).is_err());
    std::fs::remove_dir_all(&missing).ok();
}
