use datasources::{
    CreateBulkCreateStoredProcedureStatement, CreateBulkDeleteStoredProcedureStatement,
    CreateBulkUpdateStoredProcedureStatement, CreateCreateStoredProcedureStatement,
    CreateDeleteStoredProcedureStatement, CreateNestedReadStoredProcedureStatement,
    CreateReadStoredProcedureStatement, CreateUpdateStoredProcedureStatement,
    DefaultStatementTextRenderer, RoutineParameter,
};

pub struct OracleStoredProcedureStatementRenderer;

fn params(list: &[RoutineParameter]) -> String {
    list.iter().map(|p| format!("  {} IN {}", p.name, p.r#type)).collect::<Vec<_>>().join(",\n")
}

fn type_of(parameters: &[RoutineParameter], name: &str, fallback: &str) -> String {
    parameters.iter().find(|p| p.name == name).map(|p| p.r#type.clone()).unwrap_or_else(|| fallback.to_string())
}

fn json_columns(columns: &[RoutineParameter]) -> String {
    columns.iter().map(|c| format!("{} {} PATH '$.{}'", c.name, c.r#type, c.name)).collect::<Vec<_>>().join(", ")
}

impl OracleStoredProcedureStatementRenderer {
    pub fn render_create(&self, statement: &CreateCreateStoredProcedureStatement) -> String {
        format!(
            "CREATE OR REPLACE PROCEDURE {}(\n{}\n) AS\nBEGIN\n  INSERT INTO {} ({})\n  VALUES ({});\nEND;",
            statement.name(),
            params(statement.parameters()),
            statement.table(),
            statement.parameters().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
            statement.parameters().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
        )
    }

    pub fn render_nested_read(&self, statement: &CreateNestedReadStoredProcedureStatement) -> String {
        let select = DefaultStatementTextRenderer::instance().render(&statement.select());
        let signature = statement
            .arguments()
            .iter()
            .map(|argument| format!("{} IN {}", argument.parameter.name, argument.parameter.r#type))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "CREATE OR REPLACE PROCEDURE {}({}) AS\nBEGIN\n  {};\nEND;",
            statement.name(),
            signature,
            select
        )
    }

    pub fn render_read(&self, statement: &CreateReadStoredProcedureStatement) -> String {
        let aliased = statement.columns().iter().map(|c| format!("t.{c}")).collect::<Vec<_>>().join(", ");
        if statement.by_properties().is_empty() {
            return format!(
                "CREATE OR REPLACE PROCEDURE {} AS\nBEGIN\n  SELECT {aliased} FROM {} t ORDER BY t.{};\nEND;",
                statement.name(),
                statement.table(),
                statement.key(),
            );
        }
        let signature = statement.by_properties().iter().map(|c| format!("{c} IN {}", type_of(statement.parameters(), c, statement.key_type()))).collect::<Vec<_>>().join(", ");
        let pred = statement.by_properties().iter().map(|c| format!("t.{c} = {c}")).collect::<Vec<_>>().join(" AND ");
        format!(
            "CREATE OR REPLACE PROCEDURE {}({signature}) AS\nBEGIN\n  SELECT {aliased} FROM {} t WHERE {pred};\nEND;",
            statement.name(),
            statement.table(),
        )
    }

    pub fn render_update(&self, statement: &CreateUpdateStoredProcedureStatement) -> String {
        let mut sets: Vec<_> = statement.writable().iter().map(|c| format!("    t.{c} = {c}")).collect();
        if let Some(audit) = statement.audit_updated() {
            if let Some(param) = statement.parameters().iter().find(|p| p.name.contains("new_updated")) {
                sets.push(format!("    t.{audit} = {}", param.name));
            }
        }
        let by = if statement.by_properties().is_empty() { vec![statement.key().to_string()] } else { statement.by_properties().to_vec() };
        let pred = by.iter().map(|c| format!("t.{c} = {c}")).collect::<Vec<_>>().join(" AND ");
        format!(
            "CREATE OR REPLACE PROCEDURE {}(\n{}\n) AS\nBEGIN\n  UPDATE {} t\n  SET\n{}\n  WHERE {pred};\nEND;",
            statement.name(),
            params(statement.parameters()),
            statement.table(),
            sets.join(",\n"),
        )
    }

    pub fn render_delete(&self, statement: &CreateDeleteStoredProcedureStatement) -> String {
        let by = if statement.by_properties().is_empty() { vec![statement.key().to_string()] } else { statement.by_properties().to_vec() };
        let signature = by.iter().map(|c| format!("{c} IN {}", type_of(statement.parameters(), c, statement.key_type()))).collect::<Vec<_>>().join(", ");
        let pred = by.iter().map(|c| format!("t.{c} = {c}")).collect::<Vec<_>>().join(" AND ");
        format!(
            "CREATE OR REPLACE PROCEDURE {}({signature}) AS\nBEGIN\n  DELETE FROM {} t WHERE {pred};\nEND;",
            statement.name(),
            statement.table(),
        )
    }

    pub fn render_bulk_create(&self, statement: &CreateBulkCreateStoredProcedureStatement) -> String {
        let cols = statement.columns().iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ");
        format!(
            "CREATE OR REPLACE PROCEDURE {}({} IN CLOB) AS\nBEGIN\n  INSERT INTO {} ({cols})\n  SELECT {cols}\n  FROM JSON_TABLE({}, '$[*]' COLUMNS ({}));\nEND;",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.payload().name,
            json_columns(statement.columns()),
        )
    }

    pub fn render_bulk_update(&self, statement: &CreateBulkUpdateStoredProcedureStatement) -> String {
        let sets = statement.columns().iter().filter(|c| c.name != statement.key()).map(|c| format!("t.{} = payload_rows.{}", c.name, c.name)).collect::<Vec<_>>().join(", ");
        format!(
            "CREATE OR REPLACE PROCEDURE {}({} IN CLOB) AS\nBEGIN\n  MERGE INTO {} t\n  USING (\n    SELECT * FROM JSON_TABLE({}, '$[*]' COLUMNS ({}))\n  ) payload_rows ON (t.{} = payload_rows.{})\n  WHEN MATCHED THEN UPDATE SET {sets};\nEND;",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.payload().name,
            json_columns(statement.columns()),
            statement.key(),
            statement.key(),
        )
    }

    pub fn render_bulk_delete(&self, statement: &CreateBulkDeleteStoredProcedureStatement) -> String {
        format!(
            "CREATE OR REPLACE PROCEDURE {}({} IN CLOB) AS\nBEGIN\n  DELETE FROM {} t\n  WHERE t.{} IN (\n    SELECT {} FROM JSON_TABLE({}, '$[*]' COLUMNS ({} {} PATH '$.{}'))\n  );\nEND;",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.key(),
            statement.key(),
            statement.payload().name,
            statement.key(),
            statement.key_type(),
            statement.key(),
        )
    }
}
