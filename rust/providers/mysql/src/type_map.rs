use datasources::{DatasourceError, FieldSize, Mapping, Result, SpecField};
use std::collections::HashMap;

pub struct TypeMap {
    dictionary: HashMap<String, String>,
    reverse: HashMap<String, String>,
}

impl TypeMap {
    pub fn new(dictionary: HashMap<String, String>) -> Self {
        let mut reverse = HashMap::new();
        for (spec, native) in &dictionary {
            reverse.entry(native.clone()).or_insert_with(|| spec.clone());
        }
        Self { dictionary, reverse }
    }

    pub fn convert_field(&self, field: &SpecField) -> Result<String> {
        match field.type_name.as_str() {
            "string" => Ok(match field.size {
                None | Some(FieldSize::Unlimited) => "TEXT".into(),
                Some(FieldSize::Length(n)) | Some(FieldSize::Decimal(n, _)) if n <= 16383 => {
                    format!("VARCHAR({n})")
                }
                Some(_) => "LONGTEXT".into(),
            }),
            "character" => Ok(format!("CHAR({})", char_len(field))),
            "decimal" => decimal(field, "DECIMAL", "DECIMAL(10, 2)"),
            "binary" => Ok(match field.size {
                None | Some(FieldSize::Unlimited) => "BLOB".into(),
                Some(FieldSize::Length(n)) | Some(FieldSize::Decimal(n, _)) if n <= 65535 => {
                    format!("VARBINARY({n})")
                }
                Some(_) => "LONGBLOB".into(),
            }),
            _ => Mapping::convert(self, &field.type_name),
        }
    }
}

fn char_len(field: &SpecField) -> i32 {
    match field.size {
        None | Some(FieldSize::Unlimited) => 1,
        Some(FieldSize::Length(n)) | Some(FieldSize::Decimal(n, _)) => n,
    }
}

fn decimal(field: &SpecField, type_name: &str, fallback: &str) -> Result<String> {
    match field.size {
        None | Some(FieldSize::Unlimited) => Ok(fallback.into()),
        Some(FieldSize::Decimal(p, s)) => Ok(format!("{type_name}({p}, {s})")),
        Some(FieldSize::Length(_)) => Err(DatasourceError::message(format!(
            "decimal field \"{}\" size must be [precision, scale] tuple; got {:?}",
            field.name.as_deref().unwrap_or(""),
            field.size
        ))),
    }
}

impl Mapping<String, String> for TypeMap {
    fn convert(&self, from: &String) -> Result<String> {
        self.dictionary
            .get(from)
            .cloned()
            .ok_or_else(|| DatasourceError::message(format!("Unknown spec field type: {from}")))
    }

    fn reverse_convert(&self, native: &String) -> Result<String> {
        self.reverse
            .get(native)
            .cloned()
            .ok_or_else(|| DatasourceError::message(format!("Unknown native field type: {native}")))
    }
}
