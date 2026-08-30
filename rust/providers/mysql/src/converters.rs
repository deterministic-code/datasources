use datasources::converters::{
    sql_provider_types, use_provider, ProviderTypes,
};
use std::collections::HashMap;

pub fn types() -> ProviderTypes {
    sql_provider_types(
        HashMap::from([
            ("VARCHAR".into(), "TEXT".into()),
            ("CHAR".into(), "TEXT".into()),
            ("TINYTEXT".into(), "TEXT".into()),
            ("MEDIUMTEXT".into(), "TEXT".into()),
            ("LONGTEXT".into(), "TEXT".into()),
            ("TINYINT".into(), "INTEGER".into()),
            ("SMALLINT".into(), "INTEGER".into()),
            ("MEDIUMINT".into(), "INTEGER".into()),
            ("INT".into(), "INTEGER".into()),
            ("BIGINT".into(), "INTEGER".into()),
            ("FLOAT".into(), "REAL".into()),
            ("DOUBLE".into(), "REAL".into()),
            ("DECIMAL".into(), "NUMERIC".into()),
            ("TINYINT(1)".into(), "BOOLEAN".into()),
            ("TIMESTAMP".into(), "DATETIME".into()),
            ("DATE".into(), "DATETIME".into()),
            ("BINARY".into(), "BLOB".into()),
            ("CHAR(36)".into(), "UUID".into())
        ]),
        true,
        "integer",
    )
}

pub fn register() {
    use_provider("mysql", types());
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::converters::{mapping, reset, Cells};
    use datasources::{Record, Value};

    #[test]
    fn registers_every_mapped_type() {
        reset();
        register();
        let mapping = mapping::<Cells>("mysql").unwrap();
        assert_eq!(mapping.source, "mysql");
        for sql_type in types().types.keys() {
            let mut row = Record::new();
            row.insert("col".into(), sample(sql_type));
            let dest = mapping.to(&row).unwrap();
            mapping.from(&dest).unwrap();
        }
    }

    fn sample(sql_type: &str) -> Value {
        let key = sql_type.to_ascii_uppercase();
        if key.contains("BOOL") || key == "BIT" || key == "TINYINT(1)" {
            return Value::Int(1);
        }
        if key.contains("DATE") || key.contains("TIME") {
            return Value::from("2020-01-02T03:04:05.000Z");
        }
        if key.contains("BLOB") || key.contains("BINARY") || key == "BYTEA" || key == "RAW" || key == "VARBINARY" {
            return Value::from("aGk=");
        }
        if key.contains("UUID") || key == "UNIQUEIDENTIFIER" || key == "CHAR(36)" {
            return Value::from("550e8400-e29b-41d4-a716-446655440000");
        }
        if key.contains("NUMERIC") || key.contains("DECIMAL") || key == "NUMBER" {
            return Value::from("1.25");
        }
        if key.contains("INT") || key.contains("REAL") || key.contains("FLOAT") || key.contains("DOUBLE") {
            return Value::Int(1);
        }
        Value::from("x")
    }
}
