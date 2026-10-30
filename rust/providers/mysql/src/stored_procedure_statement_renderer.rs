use datasources::{
    CreateBulkCreateStoredProcedureStatement, CreateBulkDeleteStoredProcedureStatement,
    CreateBulkUpdateStoredProcedureStatement, CreateCreateStoredProcedureStatement,
    CreateDeleteStoredProcedureStatement, CreateNestedReadStoredProcedureStatement,
    CreateReadStoredProcedureStatement, CreateUpdateStoredProcedureStatement,
    DefaultStatementTextRenderer, RoutineParameter,
};

pub struct MysqlStoredProcedureStatementRenderer;

fn params(list: &[RoutineParameter]) -> String {
    list.iter()
        .map(|p| format!("  IN {} {}", p.name, crate::MysqlDriver::native_param(p)))
        .collect::<Vec<_>>()
        .join(",\n")
}

fn where_cols(columns: &[String]) -> String {
    columns
        .iter()
        .map(|c| format!("t.{c} = {c}"))
        .collect::<Vec<_>>()
        .join(" AND ")
}

fn json_columns(columns: &[RoutineParameter]) -> String {
    columns
        .iter()
        .map(|c| format!("{} {} PATH '$.{}'", c.name, c.r#type, c.name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn procedure(name: &str, signature: &str, body: &str) -> String {
    format!(
        "DROP PROCEDURE IF EXISTS {name};\nGO\nCREATE PROCEDURE {name}({signature})\nBEGIN\n{body}\nEND;\nGO"
    )
}

fn by_or_key(by: &[String], key: &str) -> Vec<String> {
    if by.is_empty() {
        vec![key.to_string()]
    } else {
        by.to_vec()
    }
}

fn type_of(parameters: &[RoutineParameter], name: &str, fallback: &str) -> String {
    parameters
        .iter()
        .find(|p| p.name == name)
        .map(|p| p.r#type.clone())
        .unwrap_or_else(|| fallback.to_string())
}

impl MysqlStoredProcedureStatementRenderer {
    pub fn render_create(&self, statement: &CreateCreateStoredProcedureStatement) -> String {
        let generated = statement.generated_key();
        let insert_cols = if generated {
            statement.columns().join(", ")
        } else {
            statement
                .parameters()
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
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
        let body = if generated {
            format!(
                "  DECLARE new_id VARCHAR(36);\n  SET new_id = UUID();\n  INSERT INTO {} ({insert_cols})\n  VALUES ({vals});\n  SELECT new_id AS {};",
                statement.table(),
                statement.key()
            )
        } else {
            format!(
                "  INSERT INTO {} ({insert_cols})\n  VALUES ({vals});\n  SELECT LAST_INSERT_ID() AS {};",
                statement.table(),
                statement.key()
            )
        };
        procedure(
            statement.name(),
            &format!("\n{}\n", params(statement.parameters())),
            &body,
        )
    }

    pub fn render_nested_read(&self, statement: &CreateNestedReadStoredProcedureStatement) -> String {
        let select = DefaultStatementTextRenderer::instance().render(&statement.select());
        let signature = statement
            .arguments()
            .iter()
            .map(|argument| format!("IN {} {}", argument.parameter.name, argument.parameter.r#type))
            .collect::<Vec<_>>()
            .join(", ");
        procedure(statement.name(), &signature, &format!("  {select};"))
    }

    pub fn render_read(&self, statement: &CreateReadStoredProcedureStatement) -> String {
        let aliased = statement
            .columns()
            .iter()
            .map(|c| format!("t.{c}"))
            .collect::<Vec<_>>()
            .join(", ");
        if statement.by_properties().is_empty() {
            return procedure(
                statement.name(),
                "",
                &format!(
                    "  SELECT {aliased} FROM {} t ORDER BY t.{};",
                    statement.table(),
                    statement.key()
                ),
            );
        }
        let signature = statement
            .by_properties()
            .iter()
            .map(|c| format!("IN {c} {}", type_of(statement.parameters(), c, statement.key_type())))
            .collect::<Vec<_>>()
            .join(", ");
        procedure(
            statement.name(),
            &signature,
            &format!(
                "  SELECT {aliased} FROM {} t WHERE {};",
                statement.table(),
                where_cols(statement.by_properties())
            ),
        )
    }

    pub fn render_update(&self, statement: &CreateUpdateStoredProcedureStatement) -> String {
        let mut sets: Vec<_> = statement
            .writable()
            .iter()
            .map(|c| format!("    t.{c}     = {c}"))
            .collect();
        if let Some(audit) = statement.audit_updated() {
            if let Some(param) = statement.parameters().iter().find(|p| p.name.contains("new_updated")) {
                sets.push(format!("    t.{audit} = {}", param.name));
            }
        }
        let by = by_or_key(statement.by_properties(), statement.key());
        procedure(
            statement.name(),
            &format!("\n{}\n", params(statement.parameters())),
            &format!(
                "  UPDATE {} t\n  SET\n{}\n  WHERE {};\n  SELECT ROW_COUNT() AS affected;",
                statement.table(),
                sets.join(",\n"),
                where_cols(&by)
            ),
        )
    }

    pub fn render_delete(&self, statement: &CreateDeleteStoredProcedureStatement) -> String {
        let by = by_or_key(statement.by_properties(), statement.key());
        let signature = by
            .iter()
            .map(|c| format!("IN {c} {}", type_of(statement.parameters(), c, statement.key_type())))
            .collect::<Vec<_>>()
            .join(", ");
        procedure(
            statement.name(),
            &signature,
            &format!(
                "  DELETE t FROM {} t WHERE {};\n  SELECT ROW_COUNT() AS affected;",
                statement.table(),
                where_cols(&by)
            ),
        )
    }

    pub fn render_bulk_create(&self, statement: &CreateBulkCreateStoredProcedureStatement) -> String {
        let cols = statement
            .columns()
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        procedure(
            statement.name(),
            &format!(
                "IN {} {}, OUT ids JSON",
                statement.payload().name,
                statement.payload().r#type
            ),
            &format!(
                "  INSERT INTO {} ({cols})\n  SELECT {cols}\n  FROM JSON_TABLE({}, '$[*]' COLUMNS (\n    {}\n  )) AS payload_rows;\n  GET DIAGNOSTICS @n = ROW_COUNT;\n  SET @first = LAST_INSERT_ID();\n  SET ids = JSON_ARRAY();\n  SET @i = 0;\n  WHILE @i < @n DO\n    SET ids = JSON_ARRAY_APPEND(ids, '$', @first + @i);\n    SET @i = @i + 1;\n  END WHILE;",
                statement.table(),
                statement.payload().name,
                json_columns(statement.columns()),
            ),
        )
    }

    pub fn render_bulk_update(&self, statement: &CreateBulkUpdateStoredProcedureStatement) -> String {
        let sets = statement
            .columns()
            .iter()
            .filter(|c| c.name != statement.key())
            .map(|c| format!("c.{} = payload_rows.{}", c.name, c.name))
            .collect::<Vec<_>>()
            .join(", ");
        procedure(
            statement.name(),
            &format!("IN {} {}", statement.payload().name, statement.payload().r#type),
            &format!(
                "  UPDATE {} c\n  JOIN JSON_TABLE({}, '$[*]' COLUMNS (\n    {}\n  )) AS payload_rows ON c.{} = payload_rows.{}\n  SET {sets};",
                statement.table(),
                statement.payload().name,
                json_columns(statement.columns()),
                statement.key(),
                statement.key(),
            ),
        )
    }

    pub fn render_bulk_delete(&self, statement: &CreateBulkDeleteStoredProcedureStatement) -> String {
        let key = statement
            .columns()
            .iter()
            .find(|c| c.name == statement.key())
            .cloned()
            .unwrap_or(RoutineParameter {
                name: statement.key().into(),
                r#type: statement.key_type().into(),
                field: None,
            });
        procedure(
            statement.name(),
            &format!("IN {} {}", statement.payload().name, statement.payload().r#type),
            &format!(
                "  DELETE c FROM {} c JOIN JSON_TABLE({}, '$[*]' COLUMNS ({} {} PATH '$.{}')) AS payload_rows ON c.{} = payload_rows.{};",
                statement.table(),
                statement.payload().name,
                key.name,
                key.r#type,
                key.name,
                statement.key(),
                statement.key(),
            ),
        )
    }
}
