use datasources::Result;

pub struct MysqlColumnType;

impl MysqlColumnType {
    pub fn map(canonical: &str) -> Result<String> {
        Ok(match canonical {
            "int" => "INT",
            "uuid" => "CHAR(36)",
            "string" | "datetime" => "TEXT",
            "bool" => "TINYINT",
            "decimal" => "DECIMAL(18,4)",
            "float" => "DOUBLE",
            "binary" => "BLOB",
            _ => {
                return Err(datasources::DatasourceError::message(format!(
                    "Unknown mysql type \"{canonical}\""
                )))
            }
        }
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column::MysqlColumn;
    use crate::quote::MysqlQuote;
    use datasources::{Column, CreateTableStatement, Text};

    fn user_columns() -> Vec<MysqlColumn> {
        vec![
            MysqlColumn::new("id", "int", true),
            MysqlColumn::new("uuid", "uuid", false),
            MysqlColumn::new("username", "string", false),
            MysqlColumn::new("is_enabled", "bool", false),
            MysqlColumn::new("balance", "decimal", false),
            MysqlColumn::new("location", "float", false),
            MysqlColumn::new("created", "datetime", false),
            MysqlColumn::new("version", "binary", false),
        ]
    }

    #[test]
    fn maps_known_and_unknown() {
        assert_eq!(MysqlColumnType::map("int").unwrap(), "INT");
        let error = MysqlColumnType::map("xml").unwrap_err();
        assert!(error.to_string().contains("Unknown mysql type \"xml\""));
    }

    #[test]
    fn creates_user_table() {
        let mapped: Vec<Column> = user_columns()
            .iter()
            .map(|column| {
                let affinity = MysqlColumnType::map(&column.type_name).unwrap();
                Column::new(
                    MysqlQuote::quote(&column.name),
                    if column.id {
                        format!("{affinity} PRIMARY KEY")
                    } else {
                        affinity
                    },
                )
            })
            .collect();
        assert_eq!(
            CreateTableStatement::new(MysqlQuote::quote("user"), &mapped, &[]).get_text(),
            "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)"
        );
    }
}
