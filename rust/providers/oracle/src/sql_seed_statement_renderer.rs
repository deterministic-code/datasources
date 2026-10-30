use datasources::{SqlSeedColumn, SqlSeedStatement};

pub struct OracleSqlSeedStatementRenderer;

impl OracleSqlSeedStatementRenderer {
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
        format!("INSERT INTO {table} ({cols}) VALUES ({values})")
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
        ("datetime", "UtcNow") => "SYS_EXTRACT_UTC(SYSTIMESTAMP)".into(),
        ("datetime", "Now") => "LOCALTIMESTAMP".into(),
        ("uuid", "NewId") => "SYS_GUID()".into(),
        ("boolean", text) => {
            if text == "true" || text == "True" {
                "1".into()
            } else {
                "0".into()
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
            if truthy(value) { "1".into() } else { "0".into() }
        }
        Some(value) if is_numeric(&column.field.type_name) => value.clone(),
        Some(value) if column.field.type_name == "binary" => format!("HEXTORAW('{value}')"),
        Some(value) => quote_string(value),
    }
}
