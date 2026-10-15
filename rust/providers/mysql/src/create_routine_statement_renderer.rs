use datasources::{CreateRoutineStatement, RoutineKind, RoutineParameter};

pub struct MysqlCreateRoutineStatementRenderer;

impl MysqlCreateRoutineStatementRenderer {
    pub fn render(&self, statement: &CreateRoutineStatement) -> String {
        let aliased = statement
            .columns()
            .iter()
            .map(|c| format!("t.{c}"))
            .collect::<Vec<_>>()
            .join(", ");
        let cols = statement
            .parameters()
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        match statement.kind() {
            RoutineKind::Create => create(statement, &cols),
            RoutineKind::FindOne => format!(
                "DROP PROCEDURE IF EXISTS {};\nGO\nCREATE PROCEDURE {}(IN {} {})\nBEGIN\n  SELECT {aliased} FROM {} t WHERE t.{} = {};\nEND;\nGO",
                statement.name(),
                statement.name(),
                statement.key(),
                statement.key_type(),
                statement.table(),
                statement.key(),
                statement.key(),
            ),
            RoutineKind::FindAll => format!(
                "DROP PROCEDURE IF EXISTS {};\nGO\nCREATE PROCEDURE {}()\nBEGIN\n  SELECT {aliased} FROM {} t ORDER BY t.{};\nEND;\nGO",
                statement.name(),
                statement.name(),
                statement.table(),
                statement.key(),
            ),
            RoutineKind::FindBy => find_by(statement, &aliased),
            RoutineKind::Update => update(statement),
            RoutineKind::Delete => format!(
                "DROP PROCEDURE IF EXISTS {};\nGO\nCREATE PROCEDURE {}(IN {} {})\nBEGIN\n  DELETE t FROM {} t WHERE t.{} = {};\n  SELECT ROW_COUNT() AS affected;\nEND;\nGO",
                statement.name(),
                statement.name(),
                statement.key(),
                statement.key_type(),
                statement.table(),
                statement.key(),
                statement.key(),
            ),
        }
    }
}

fn params(list: &[RoutineParameter]) -> String {
    list.iter()
        .map(|p| format!("  IN {} {}", p.name, p.r#type))
        .collect::<Vec<_>>()
        .join(",\n")
}

fn create(statement: &CreateRoutineStatement, cols: &str) -> String {
    let generated = statement.generated_key();
    let insert_cols = if generated {
        statement.columns().join(", ")
    } else {
        cols.to_string()
    };
    let vals = if generated {
        statement
            .columns()
            .iter()
            .map(|c| if c == statement.key() { "new_id".into() } else { c.clone() })
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        statement
            .parameters()
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    };
    if generated {
        format!(
            "DROP PROCEDURE IF EXISTS {};\nGO\nCREATE PROCEDURE {}(\n{}\n)\nBEGIN\n  DECLARE new_id VARCHAR(36);\n  SET new_id = UUID();\n  INSERT INTO {} ({insert_cols})\n  VALUES ({vals});\n  SELECT new_id AS {};\nEND;\nGO",
            statement.name(),
            statement.name(),
            params(statement.parameters()),
            statement.table(),
            statement.key(),
        )
    } else {
        format!(
            "DROP PROCEDURE IF EXISTS {};\nGO\nCREATE PROCEDURE {}(\n{}\n)\nBEGIN\n  INSERT INTO {} ({cols})\n  VALUES ({vals});\n  SELECT LAST_INSERT_ID() AS {};\nEND;\nGO",
            statement.name(),
            statement.name(),
            params(statement.parameters()),
            statement.table(),
            statement.key(),
        )
    }
}

fn find_by(statement: &CreateRoutineStatement, aliased: &str) -> String {
    let by = statement.by_field().unwrap_or(statement.key());
    let ty = statement
        .parameters()
        .first()
        .map(|p| p.r#type.as_str())
        .unwrap_or(statement.key_type());
    format!(
        "DROP PROCEDURE IF EXISTS {};\nGO\nCREATE PROCEDURE {}(IN {by} {ty})\nBEGIN\n  SELECT {aliased} FROM {} t WHERE t.{by} = {by};\nEND;\nGO",
        statement.name(),
        statement.name(),
        statement.table(),
    )
}

fn update(statement: &CreateRoutineStatement) -> String {
    let mut sets = statement
        .writable()
        .iter()
        .map(|c| format!("    t.{c}     = {c}"))
        .collect::<Vec<_>>();
    if let Some(audit) = statement.audit_updated() {
        if let Some(param) = statement.parameters().iter().find(|p| p.name.contains("new_updated")) {
            sets.push(format!("    t.{audit} = {}", param.name));
        }
    }
    let by = statement.by_field().unwrap_or(statement.key());
    format!(
        "DROP PROCEDURE IF EXISTS {};\nGO\nCREATE PROCEDURE {}(\n{}\n)\nBEGIN\n  UPDATE {} t\n  SET\n{}\n  WHERE t.{by} = {by};\n  SELECT ROW_COUNT() AS affected;\nEND;\nGO",
        statement.name(),
        statement.name(),
        params(statement.parameters()),
        statement.table(),
        sets.join(",\n"),
    )
}
