use datasources::{
    CreateBulkCreateStoredProcedureStatement, CreateBulkDeleteStoredProcedureStatement,
    CreateBulkUpdateStoredProcedureStatement, CreateCreateStoredProcedureStatement,
    CreateDeleteStoredProcedureStatement, CreateNestedReadStoredProcedureStatement,
    CreateReadStoredProcedureStatement, CreateUpdateStoredProcedureStatement,
    DefaultStatementTextRenderer, RoutineParameter,
};

pub struct SqlServerStoredProcedureStatementRenderer;

fn params(list: &[RoutineParameter]) -> String {
    list.iter().map(|p| format!("  @{} {}", p.name, crate::SqlServerDriver::native_param(p))).collect::<Vec<_>>().join(",\n")
}

fn type_of(parameters: &[RoutineParameter], name: &str, fallback: &str) -> String {
    parameters.iter().find(|p| p.name == name).map(|p| p.r#type.clone()).unwrap_or_else(|| fallback.to_string())
}

fn openjson(columns: &[RoutineParameter]) -> String {
    columns.iter().map(|c| format!("{} {} '$.{}'", c.name, c.r#type, c.name)).collect::<Vec<_>>().join(", ")
}

impl SqlServerStoredProcedureStatementRenderer {
    pub fn render_create(&self, statement: &CreateCreateStoredProcedureStatement) -> String {
        format!(
            "CREATE OR ALTER PROCEDURE {}\n{}\nAS\nBEGIN\n  INSERT INTO {} ({})\n  VALUES ({});\n  SELECT CAST(SCOPE_IDENTITY() AS BIGINT) AS {};\nEND;\nGO",
            statement.name(),
            params(statement.parameters()),
            statement.table(),
            statement.parameters().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
            statement.parameters().iter().map(|p| format!("@{}", p.name)).collect::<Vec<_>>().join(", "),
            statement.key(),
        )
    }

    pub fn render_nested_read(&self, statement: &CreateNestedReadStoredProcedureStatement) -> String {
        let select = DefaultStatementTextRenderer::instance().render(&statement.select());
        let signature = statement
            .arguments()
            .iter()
            .map(|argument| format!("  @{} {}", argument.parameter.name, argument.parameter.r#type))
            .collect::<Vec<_>>()
            .join(",\n");
        format!(
            "CREATE OR ALTER PROCEDURE {}\n{}\nAS\nBEGIN\n  {};\nEND;\nGO",
            statement.name(),
            signature,
            select
        )
    }

    pub fn render_read(&self, statement: &CreateReadStoredProcedureStatement) -> String {
        let cols = statement.columns().join(", ");
        if statement.by_properties().is_empty() {
            return format!(
                "CREATE OR ALTER PROCEDURE {}\nAS\nBEGIN\n  SELECT {cols} FROM {} ORDER BY {};\nEND;\nGO",
                statement.name(),
                statement.table(),
                statement.key(),
            );
        }
        let signature = statement.by_properties().iter().map(|c| format!("@{c} {}", type_of(statement.parameters(), c, statement.key_type()))).collect::<Vec<_>>().join(", ");
        let pred = statement.by_properties().iter().map(|c| format!("{c} = @{c}")).collect::<Vec<_>>().join(" AND ");
        format!(
            "CREATE OR ALTER PROCEDURE {} {signature}\nAS\nBEGIN\n  SELECT {cols} FROM {} WHERE {pred};\nEND;\nGO",
            statement.name(),
            statement.table(),
        )
    }

    pub fn render_update(&self, statement: &CreateUpdateStoredProcedureStatement) -> String {
        let mut sets: Vec<_> = statement.writable().iter().map(|c| format!("    {c} = @{c}")).collect();
        if let Some(audit) = statement.audit_updated() {
            if let Some(param) = statement.parameters().iter().find(|p| p.name.contains("new_updated")) {
                sets.push(format!("    {audit} = @{}", param.name));
            }
        }
        let by = if statement.by_properties().is_empty() { vec![statement.key().to_string()] } else { statement.by_properties().to_vec() };
        let pred = by.iter().map(|c| format!("{c} = @{c}")).collect::<Vec<_>>().join(" AND ");
        format!(
            "CREATE OR ALTER PROCEDURE {}\n{}\nAS\nBEGIN\n  UPDATE {}\n  SET\n{}\n  WHERE {pred};\n  SELECT @@ROWCOUNT AS affected;\nEND;\nGO",
            statement.name(),
            params(statement.parameters()),
            statement.table(),
            sets.join(",\n"),
        )
    }

    pub fn render_delete(&self, statement: &CreateDeleteStoredProcedureStatement) -> String {
        let by = if statement.by_properties().is_empty() { vec![statement.key().to_string()] } else { statement.by_properties().to_vec() };
        let signature = by.iter().map(|c| format!("@{c} {}", type_of(statement.parameters(), c, statement.key_type()))).collect::<Vec<_>>().join(", ");
        let pred = by.iter().map(|c| format!("{c} = @{c}")).collect::<Vec<_>>().join(" AND ");
        format!(
            "CREATE OR ALTER PROCEDURE {} {signature}\nAS\nBEGIN\n  DELETE FROM {} WHERE {pred};\n  SELECT @@ROWCOUNT AS affected;\nEND;\nGO",
            statement.name(),
            statement.table(),
        )
    }

    pub fn render_bulk_create(&self, statement: &CreateBulkCreateStoredProcedureStatement) -> String {
        let cols = statement.columns().iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ");
        format!(
            "CREATE OR ALTER PROCEDURE {}\n  @{} NVARCHAR(MAX),\n  @ids NVARCHAR(MAX) OUTPUT\nAS\nBEGIN\n  INSERT INTO {} ({cols})\n  SELECT {cols}\n  FROM OPENJSON(@{}) WITH ({});\n  SET @ids = (SELECT {} FROM {} WHERE {} >= SCOPE_IDENTITY() FOR JSON PATH);\nEND;\nGO",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.payload().name,
            openjson(statement.columns()),
            statement.key(),
            statement.table(),
            statement.key(),
        )
    }

    pub fn render_bulk_update(&self, statement: &CreateBulkUpdateStoredProcedureStatement) -> String {
        let sets = statement.columns().iter().filter(|c| c.name != statement.key()).map(|c| format!("t.{} = payload_rows.{}", c.name, c.name)).collect::<Vec<_>>().join(", ");
        format!(
            "CREATE OR ALTER PROCEDURE {}\n  @{} NVARCHAR(MAX)\nAS\nBEGIN\n  UPDATE t SET {sets}\n  FROM {} t\n  JOIN OPENJSON(@{}) WITH ({}) AS payload_rows\n    ON t.{} = payload_rows.{};\nEND;\nGO",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.payload().name,
            openjson(statement.columns()),
            statement.key(),
            statement.key(),
        )
    }

    pub fn render_bulk_delete(&self, statement: &CreateBulkDeleteStoredProcedureStatement) -> String {
        format!(
            "CREATE OR ALTER PROCEDURE {}\n  @{} NVARCHAR(MAX)\nAS\nBEGIN\n  DELETE t FROM {} t\n  JOIN OPENJSON(@{}) WITH ({} {} '$.{}') AS payload_rows\n    ON t.{} = payload_rows.{};\nEND;\nGO",
            statement.name(),
            statement.payload().name,
            statement.table(),
            statement.payload().name,
            statement.key(),
            statement.key_type(),
            statement.key(),
            statement.key(),
            statement.key(),
        )
    }
}
