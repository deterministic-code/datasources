use datasources::{CreateRoutineStatement, RoutineKind, RoutineParameter};

pub struct PostgresCreateRoutineStatementRenderer;

impl PostgresCreateRoutineStatementRenderer {
    pub fn render(&self, statement: &CreateRoutineStatement) -> String {
        let aliased = statement
            .columns()
            .iter()
            .map(|c| format!("t.{c}"))
            .collect::<Vec<_>>()
            .join(", ");
        match statement.kind() {
            RoutineKind::Create => format!(
                "CREATE OR REPLACE FUNCTION {}(\n{}\n) RETURNS {} LANGUAGE plpgsql AS $$\nDECLARE\n  new_id {};\nBEGIN\n  INSERT INTO {} ({})\n  VALUES ({})\n  RETURNING {} INTO new_id;\n  RETURN new_id;\nEND;\n$$",
                statement.name(),
                params(statement.parameters()),
                statement.key_type(),
                statement.key_type(),
                statement.table(),
                statement.parameters().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
                statement.parameters().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
                statement.key(),
            ),
            RoutineKind::FindOne => format!(
                "CREATE OR REPLACE FUNCTION {}({} {}) RETURNS SETOF {}\nLANGUAGE sql AS $$ SELECT {aliased} FROM {} t WHERE t.{} = {}.{}; $$",
                statement.name(),
                statement.key(),
                statement.key_type(),
                statement.table(),
                statement.table(),
                statement.key(),
                statement.name(),
                statement.key(),
            ),
            RoutineKind::FindAll => format!(
                "CREATE OR REPLACE FUNCTION {}() RETURNS SETOF {}\nLANGUAGE sql AS $$ SELECT {aliased} FROM {} t ORDER BY t.{}; $$",
                statement.name(),
                statement.table(),
                statement.table(),
                statement.key(),
            ),
            RoutineKind::FindBy => {
                let by = statement.by_field().unwrap_or(statement.key());
                let ty = statement.parameters().first().map(|p| p.r#type.as_str()).unwrap_or(statement.key_type());
                format!(
                    "CREATE OR REPLACE FUNCTION {}({by} {ty}) RETURNS SETOF {}\nLANGUAGE sql AS $$ SELECT {aliased} FROM {} t WHERE t.{by} = {}.{by}; $$",
                    statement.name(),
                    statement.table(),
                    statement.table(),
                    statement.name(),
                )
            }
            RoutineKind::Update => update(statement),
            RoutineKind::Delete => format!(
                "CREATE OR REPLACE FUNCTION {}({} {}) RETURNS INT LANGUAGE plpgsql AS $$\nDECLARE\n  affected INT;\nBEGIN\n  DELETE FROM {} t WHERE t.{} = {}.{};\n  GET DIAGNOSTICS affected = ROW_COUNT;\n  RETURN affected;\nEND;\n$$",
                statement.name(),
                statement.key(),
                statement.key_type(),
                statement.table(),
                statement.key(),
                statement.name(),
                statement.key(),
            ),
        }
    }
}

fn params(list: &[RoutineParameter]) -> String {
    list.iter()
        .map(|p| format!("  {} {}", p.name, p.r#type))
        .collect::<Vec<_>>()
        .join(",\n")
}

fn update(statement: &CreateRoutineStatement) -> String {
    let mut sets = statement
        .writable()
        .iter()
        .map(|c| format!("    {c} = {}.{c}", statement.name()))
        .collect::<Vec<_>>();
    if let Some(audit) = statement.audit_updated() {
        if let Some(param) = statement.parameters().iter().find(|p| p.name.contains("new_updated")) {
            sets.push(format!("    {audit} = {}.{}", statement.name(), param.name));
        }
    }
    let by = statement.by_field().unwrap_or(statement.key());
    format!(
        "CREATE OR REPLACE FUNCTION {}(\n{}\n) RETURNS INT LANGUAGE plpgsql AS $$\nDECLARE\n  affected INT;\nBEGIN\n  UPDATE {} t\n  SET\n{}\n  WHERE t.{by} = {}.{by};\n  GET DIAGNOSTICS affected = ROW_COUNT;\n  RETURN affected;\nEND;\n$$",
        statement.name(),
        params(statement.parameters()),
        statement.table(),
        sets.join(",\n"),
        statement.name(),
    )
}
