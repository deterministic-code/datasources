use datasources::{CreateRoutineStatement, RoutineKind, RoutineParameter};

pub struct SqlServerCreateRoutineStatementRenderer;

impl SqlServerCreateRoutineStatementRenderer {
    pub fn render(&self, statement: &CreateRoutineStatement) -> String {
        let cols = statement.columns().join(", ");
        match statement.kind() {
            RoutineKind::Create => format!(
                "CREATE OR ALTER PROCEDURE {}\n{}\nAS\nBEGIN\n  INSERT INTO {} ({})\n  VALUES ({});\n  SELECT CAST(SCOPE_IDENTITY() AS BIGINT) AS id;\nEND;\nGO",
                statement.name(),
                params(statement.parameters()),
                statement.table(),
                statement.parameters().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
                statement.parameters().iter().map(|p| format!("@{}", p.name)).collect::<Vec<_>>().join(", "),
            ),
            RoutineKind::FindOne => format!(
                "CREATE OR ALTER PROCEDURE {} @{} {}\nAS\nBEGIN\n  SELECT {cols} FROM {} WHERE {} = @{};\nEND;\nGO",
                statement.name(),
                statement.key(),
                statement.key_type(),
                statement.table(),
                statement.key(),
                statement.key(),
            ),
            RoutineKind::FindAll => format!(
                "CREATE OR ALTER PROCEDURE {}\nAS\nBEGIN\n  SELECT {cols} FROM {} ORDER BY {};\nEND;\nGO",
                statement.name(),
                statement.table(),
                statement.key(),
            ),
            RoutineKind::FindBy => {
                let by = statement.by_field().unwrap_or(statement.key());
                let ty = statement.parameters().first().map(|p| p.r#type.as_str()).unwrap_or(statement.key_type());
                format!(
                    "CREATE OR ALTER PROCEDURE {} @{by} {ty}\nAS\nBEGIN\n  SELECT {cols} FROM {} WHERE {by} = @{by};\nEND;\nGO",
                    statement.name(),
                    statement.table(),
                )
            }
            RoutineKind::Update => update(statement),
            RoutineKind::Delete => format!(
                "CREATE OR ALTER PROCEDURE {} @{} {}\nAS\nBEGIN\n  DELETE FROM {} WHERE {} = @{};\n  SELECT @@ROWCOUNT AS affected;\nEND;\nGO",
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
        .map(|p| format!("  @{} {}", p.name, p.r#type))
        .collect::<Vec<_>>()
        .join(",\n")
}

fn update(statement: &CreateRoutineStatement) -> String {
    let mut sets = statement
        .writable()
        .iter()
        .map(|c| format!("    {c} = @{c}"))
        .collect::<Vec<_>>();
    if let Some(audit) = statement.audit_updated() {
        if let Some(param) = statement.parameters().iter().find(|p| p.name.contains("new_updated")) {
            sets.push(format!("    {audit} = @{}", param.name));
        }
    }
    let by = statement.by_field().unwrap_or(statement.key());
    format!(
        "CREATE OR ALTER PROCEDURE {}\n{}\nAS\nBEGIN\n  UPDATE {}\n  SET\n{}\n  WHERE {by} = @{by};\n  SELECT @@ROWCOUNT AS affected;\nEND;\nGO",
        statement.name(),
        params(statement.parameters()),
        statement.table(),
        sets.join(",\n"),
    )
}
