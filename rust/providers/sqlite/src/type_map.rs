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
        if field.type_name == "string" {
            return Ok(match field.size {
                None | Some(FieldSize::Unlimited) => "TEXT".into(),
                Some(FieldSize::Length(n)) | Some(FieldSize::Decimal(n, _)) => format!("VARCHAR({n})"),
            });
        }
        Mapping::convert(self, &field.type_name)
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
