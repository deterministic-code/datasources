use datasources::{Column, CreateTableStatement, Result, Text};

use crate::column::SqliteColumn;
use crate::quote::SqliteQuote;

pub struct SqliteColumnType;

impl SqliteColumnType {
    pub fn map(canonical: &str) -> Result<String> {
        Ok(match canonical {
            "int" => "INTEGER",
            "uuid" | "string" | "datetime" => "TEXT",
            "bool" => "INTEGER",
            "decimal" => "NUMERIC",
            "float" => "REAL",
            "binary" => "BLOB",
            _ => {
                return Err(datasources::DatasourceError::message(format!(
                    "Unknown sqlite type \"{canonical}\""
                )))
            }
        }
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_columns() -> Vec<SqliteColumn> {
        vec![
            SqliteColumn::new("id", "int", true),
            SqliteColumn::new("uuid", "uuid", false),
            SqliteColumn::new("username", "string", false),
            SqliteColumn::new("is_enabled", "bool", false),
            SqliteColumn::new("balance", "decimal", false),
            SqliteColumn::new("location", "float", false),
            SqliteColumn::new("created", "datetime", false),
            SqliteColumn::new("version", "binary", false),
        ]
    }

    #[test]
    fn maps_known_and_unknown() {
        assert_eq!(SqliteColumnType::map("int").unwrap(), "INTEGER");
        let error = SqliteColumnType::map("xml").unwrap_err();
        assert!(error.to_string().contains("Unknown sqlite type \"xml\""));
    }

    #[test]
    fn creates_user_table() {
        let mapped: Vec<Column> = user_columns()
            .iter()
            .map(|column| {
                let affinity = SqliteColumnType::map(&column.type_name).unwrap();
                Column::new(
                    SqliteQuote::quote(&column.name),
                    if column.id {
                        format!("{affinity} PRIMARY KEY")
                    } else {
                        affinity
                    },
                )
            })
            .collect();
        assert_eq!(
            CreateTableStatement::new(SqliteQuote::quote("user"), &mapped, &[]).get_text(),
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" TEXT, \"username\" TEXT, \"is_enabled\" INTEGER, \"balance\" NUMERIC, \"location\" REAL, \"created\" TEXT, \"version\" BLOB)"
        );
    }
}
