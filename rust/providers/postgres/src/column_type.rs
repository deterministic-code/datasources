use datasources::{Column, CreateTableStatement, Result, Text};

use crate::column::PostgresColumn;
use crate::quote::PostgresQuote;

pub struct PostgresColumnType;

impl PostgresColumnType {
    pub fn map(canonical: &str) -> Result<String> {
        Ok(match canonical {
            "int" => "INTEGER",
            "uuid" => "UUID",
            "string" => "TEXT",
            "bool" => "BOOLEAN",
            "decimal" => "NUMERIC",
            "float" => "DOUBLE PRECISION",
            "datetime" => "TIMESTAMPTZ",
            "binary" => "BYTEA",
            _ => {
                return Err(datasources::DatasourceError::message(format!(
                    "Unknown postgres type \"{canonical}\""
                )))
            }
        }
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_columns() -> Vec<PostgresColumn> {
        vec![
            PostgresColumn::new("id", "int", true),
            PostgresColumn::new("uuid", "uuid", false),
            PostgresColumn::new("username", "string", false),
            PostgresColumn::new("is_enabled", "bool", false),
            PostgresColumn::new("balance", "decimal", false),
            PostgresColumn::new("location", "float", false),
            PostgresColumn::new("created", "datetime", false),
            PostgresColumn::new("version", "binary", false),
        ]
    }

    #[test]
    fn maps_known_and_unknown() {
        assert_eq!(PostgresColumnType::map("int").unwrap(), "INTEGER");
        let error = PostgresColumnType::map("xml").unwrap_err();
        assert!(error.to_string().contains("Unknown postgres type \"xml\""));
    }

    #[test]
    fn creates_user_table() {
        let mapped: Vec<Column> = user_columns()
            .iter()
            .map(|column| {
                let affinity = PostgresColumnType::map(&column.type_name).unwrap();
                Column::new(
                    PostgresQuote::quote(&column.name),
                    if column.id {
                        format!("{affinity} PRIMARY KEY")
                    } else {
                        affinity
                    },
                )
            })
            .collect();
        assert_eq!(
            CreateTableStatement::new(PostgresQuote::quote("user"), &mapped).get_text(),
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" UUID, \"username\" TEXT, \"is_enabled\" BOOLEAN, \"balance\" NUMERIC, \"location\" DOUBLE PRECISION, \"created\" TIMESTAMPTZ, \"version\" BYTEA)"
        );
    }
}
