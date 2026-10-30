use datasources::{
    CreateBulkCreateStoredProcedureStatement, CreateBulkDeleteStoredProcedureStatement,
    CreateBulkUpdateStoredProcedureStatement, CreateCreateStoredProcedureStatement,
    CreateDeleteStoredProcedureStatement, CreateNestedReadStoredProcedureStatement,
    CreateReadStoredProcedureStatement, CreateUpdateStoredProcedureStatement,
    DefaultStatementTextRenderer, RoutineParameter,
};

pub struct PostgresStoredProcedureStatementRenderer;

fn params(list: &[RoutineParameter]) -> String {
    list.iter()
        .map(|p| format!("  {} {}", p.name, crate::PostgresDriver::native_param(p)))
        .collect::<Vec<_>>()
        .join(",\n")
}

fn type_of(parameters: &[RoutineParameter], name: &str, fallback: &str) -> String {
    parameters
        .iter()
        .find(|p| p.name == name)
        .map(|p| p.r#type.clone())
        .unwrap_or_else(|| fallback.to_string())
}

fn recordset(columns: &[RoutineParameter]) -> String {
    columns
        .iter()
        .map(|c| format!("{} {}", c.name, c.r#type))
        .collect::<Vec<_>>()
        .join(", ")
}

impl PostgresStoredProcedureStatementRenderer {
    pub fn render_create(&self, statement: &CreateCreateStoredProcedureStatement) -> String {
        format!(
            "CREATE OR REPLACE FUNCTION {}(\n{}\n) RETURNS {} LANGUAGE plpgsql AS $$\nDECLARE\n  new_id {};\nBEGIN\n  INSERT INTO {} ({})\n  VALUES ({})\n  RETURNING {} INTO new_id;\n  RETURN new_id;\nEND;\n$$",
            statement.name(),
            params(statement.parameters()),
            statement.key_type(),
            statement.key_type(),
            statement.table(),
            statement.parameters().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
            statement.parameters().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
            statement.key(),
        )
    }

    pub fn render_nested_read(&self, statement: &CreateNestedReadStoredProcedureStatement) -> String {
        let select = DefaultStatementTextRenderer::instance().render(&statement.select());
        let signature = statement
            .arguments()
            .iter()
            .map(|argument| format!("{} {}", argument.parameter.name, argument.parameter.r#type))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "CREATE OR REPLACE FUNCTION {}({}) RETURNS SETOF RECORD\nLANGUAGE sql AS $$ {}; $$",
            statement.name(),
            signature,
            select
        )
    }

    pub fn render_read(&self, statement: &CreateReadStoredProcedureStatement) -> String {
        let aliased = statement.columns().iter().map(|c| format!("t.{c}")).collect::<Vec<_>>().join(", ");
        if statement.by_properties().is_empty() {
            return format!(
                "CREATE OR REPLACE FUNCTION {}() RETURNS SETOF {}\nLANGUAGE sql AS $$ SELECT {aliased} FROM {} t ORDER BY t.{}; $$",
                statement.name(),
                statement.table(),
                statement.table(),
                statement.key(),
            );
        }
        let signature = statement
            .by_properties()
            .iter()
            .map(|c| format!("{c} {}", type_of(statement.parameters(), c, statement.key_type())))
            .collect::<Vec<_>>()
            .join(", ");
        let pred = statement
            .by_properties()
            .iter()
            .map(|c| format!("t.{c} = {}.{c}", statement.name()))
            .collect::<Vec<_>>()
            .join(" AND ");
        format!(
            "CREATE OR REPLACE FUNCTION {}({signature}) RETURNS SETOF {}\nLANGUAGE sql AS $$ SELECT {aliased} FROM {} t WHERE {pred}; $$",
            statement.name(),
            statement.table(),
            statement.table(),
        )
    }

    pub fn render_update(&self, statement: &CreateUpdateStoredProcedureStatement) -> String {
        let mut sets: Vec<_> = statement
            .writable()
            .iter()
            .map(|c| format!("    {c} = {}.{c}", statement.name()))
            .collect();
        if let Some(audit) = statement.audit_updated() {
            if let Some(param) = statement.parameters().iter().find(|p| p.name.contains("new_updated")) {
                sets.push(format!("    {audit} = {}.{}", statement.name(), param.name));
            }
        }
        let by = if statement.by_properties().is_empty() {
            vec![statement.key().to_string()]
        } else {
            statement.by_properties().to_vec()
        };
        let pred = by
            .iter()
            .map(|c| format!("t.{c} = {}.{c}", statement.name()))
            .collect::<Vec<_>>()
            .join(" AND ");
        format!(
            "CREATE OR REPLACE FUNCTION {}(\n{}\n) RETURNS INT LANGUAGE plpgsql AS $$\nDECLARE\n  affected INT;\nBEGIN\n  UPDATE {} t\n  SET\n{}\n  WHERE {pred};\n  GET DIAGNOSTICS affected = ROW_COUNT;\n  RETURN affected;\nEND;\n$$",
            statement.name(),
            params(statement.parameters()),
            statement.table(),
            sets.join(",\n"),
        )
    }

    pub fn render_delete(&self, statement: &CreateDeleteStoredProcedureStatement) -> String {
        let by = if statement.by_properties().is_empty() {
            vec![statement.key().to_string()]
        } else {
            statement.by_properties().to_vec()
        };
        let signature = by
            .iter()
            .map(|c| format!("{c} {}", type_of(statement.parameters(), c, statement.key_type())))
            .collect::<Vec<_>>()
            .join(", ");
        let pred = by
            .iter()
            .map(|c| format!("t.{c} = {}.{c}", statement.name()))
            .collect::<Vec<_>>()
            .join(" AND ");
        format!(
            "CREATE OR REPLACE FUNCTION {}({signature}) RETURNS INT LANGUAGE plpgsql AS $$\nDECLARE\n  affected INT;\nBEGIN\n  DELETE FROM {} t WHERE {pred};\n  GET DIAGNOSTICS affected = ROW_COUNT;\n  RETURN affected;\nEND;\n$$",
            statement.name(),
            statement.table(),
        )
    }

    pub fn render_bulk_create(&self, statement: &CreateBulkCreateStoredProcedureStatement) -> String {
        let cols = statement.columns().iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ");
        format!(
            "CREATE OR REPLACE PROCEDURE {}({} jsonb, INOUT ids jsonb DEFAULT NULL)\nLANGUAGE plpgsql AS $$\nBEGIN\n  WITH inserted AS (\n    INSERT INTO {} ({cols})\n    SELECT {cols}\n    FROM jsonb_to_recordset({}) AS x({})\n    RETURNING {}\n  )\n  SELECT COALESCE(jsonb_agg({}), '[]'::jsonb) INTO ids FROM inserted;\nEND;\n$$",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.payload().name,
            recordset(statement.columns()),
            statement.key(),
            statement.key(),
        )
    }

    pub fn render_bulk_update(&self, statement: &CreateBulkUpdateStoredProcedureStatement) -> String {
        let sets = statement
            .columns()
            .iter()
            .filter(|c| c.name != statement.key())
            .map(|c| format!("{} = x.{}", c.name, c.name))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "CREATE OR REPLACE PROCEDURE {}({} jsonb)\nLANGUAGE plpgsql AS $$\nBEGIN\n  UPDATE {} t SET\n    {sets}\n  FROM jsonb_to_recordset({}) AS x({})\n  WHERE t.{} = x.{};\nEND;\n$$",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.payload().name,
            recordset(statement.columns()),
            statement.key(),
            statement.key(),
        )
    }

    pub fn render_bulk_delete(&self, statement: &CreateBulkDeleteStoredProcedureStatement) -> String {
        format!(
            "CREATE OR REPLACE PROCEDURE {}({} jsonb)\nLANGUAGE plpgsql AS $$\nBEGIN\n  DELETE FROM {} WHERE {} IN (SELECT {} FROM jsonb_to_recordset({}) AS x({} {}));\nEND;\n$$",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.key(),
            statement.key(),
            statement.payload().name,
            statement.key(),
            statement.key_type(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{
        NestedSelectBuilder, NestedSelectBuilderOptions, NestedSelectType, NestedWhereArgument,
    };

    #[test]
    fn renders_nested_read() {
        let nested = NestedSelectBuilder::build(
            &NestedSelectType::new(
                "Contact",
                "Id",
                vec!["Id".into(), "Email".into()],
                vec![],
            ),
            NestedSelectBuilderOptions::default(),
        );
        let sql = PostgresStoredProcedureStatementRenderer.render_nested_read(
            &CreateNestedReadStoredProcedureStatement::new(
                "ReadContactsByEmail",
                nested,
                vec![NestedWhereArgument::new(
                    "Contact.Email",
                    RoutineParameter {
                        name: "email".into(),
                        r#type: "varchar".into(),
                        field: None,
                    },
                )],
            ),
        );
        assert!(sql.contains("RETURNS SETOF RECORD"));
    }
}
