use datasources::{SqlSeedColumn, SqlSeedStatement};

pub struct PostgresSqlSeedStatementRenderer;

impl PostgresSqlSeedStatementRenderer {
    pub fn render(&self, statement: &SqlSeedStatement) -> String {
        let table = quote(statement.table());
        if statement.kind() == "delete" {
            let key = &statement.columns()[0];
            return format!(
                "DELETE FROM {table} WHERE {} = {}",
                quote(&key.column),
                literal(key)
            );
        }
        let cols = statement
            .columns()
            .iter()
            .map(|c| quote(&c.column))
            .collect::<Vec<_>>()
            .join(", ");
        let values = statement.columns().iter().map(literal).collect::<Vec<_>>().join(", ");
        let insert = format!("INSERT INTO {table} ({cols}) VALUES ({values})");
        match (statement.identity(), statement.id_column()) {
            (true, Some(id)) => format!(
                "{insert};\nSELECT setval(pg_get_serial_sequence('{}', '{id}'), (SELECT MAX({}) FROM {table}))",
                statement.table(),
                quote(id)
            ),
            _ => insert,
        }
    }
}

fn quote(ident: &str) -> String {
    format!("\"{ident}\"")
}

fn quote_string(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn truthy(value: &str) -> bool {
    value == "true" || value == "True" || value == "1"
}

fn is_numeric(ty: &str) -> bool {
    matches!(
        ty,
        "integer"
            | "unsignedinteger"
            | "smallinteger"
            | "unsignedsmallinteger"
            | "biginteger"
            | "unsignedbiginteger"
            | "reference"
            | "float"
            | "decimal"
    )
}

fn default_sql(ty: &str, def: &str) -> String {
    match (ty, def) {
        ("datetime", "UtcNow") => "(NOW() AT TIME ZONE 'UTC')".into(),
        ("datetime", "Now") => "LOCALTIMESTAMP".into(),
        ("uuid", "NewId") => "gen_random_uuid()".into(),
        ("boolean", text) => {
            if text == "true" || text == "True" {
                "TRUE".into()
            } else {
                "FALSE".into()
            }
        }
        _ => quote_string(def),
    }
}

fn literal(column: &SqlSeedColumn) -> String {
    match &column.value {
        None => column
            .field
            .default_value
            .as_deref()
            .map(|def| default_sql(&column.field.type_name, def))
            .unwrap_or_else(|| "NULL".into()),
        Some(value) if column.field.type_name == "boolean" => {
            if truthy(value) { "TRUE".into() } else { "FALSE".into() }
        }
        Some(value) if is_numeric(&column.field.type_name) => value.clone(),
        Some(value) if column.field.type_name == "binary" => format!("'\\x{value}'"),
        Some(value) => quote_string(value),
    }
}
