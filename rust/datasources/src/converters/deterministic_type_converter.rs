use super::decimal_rule::DecimalRule;
use super::mapping::Mapping;
use super::spec_field::{FieldSize, SpecField};
use super::spec_field_rules::SpecFieldRules;
use crate::datasource_error::DatasourceError;
use crate::result::Result;
use std::collections::HashMap;

pub struct DeterministicTypeConverter {
    pub dictionary: HashMap<String, String>,
    reverse: HashMap<String, Vec<String>>,
    rules: SpecFieldRules,
}

impl DeterministicTypeConverter {
    pub fn new(dictionary: HashMap<String, String>, rules: SpecFieldRules) -> Self {
        let mut reverse = HashMap::<String, Vec<String>>::new();
        for (spec_type, native) in &dictionary {
            reverse.entry(native.clone()).or_default().push(spec_type.clone());
        }
        Self { dictionary, reverse, rules }
    }

    pub fn convert(&self, field: &SpecField) -> Result<String> {
        self.convert_field(field)
    }

    fn convert_field(&self, field: &SpecField) -> Result<String> {
        let row = &self.rules;
        if field.type_name == "string" {
            return Ok(sized(&row.string_unlimited, row.string_sized, field));
        }
        if field.type_name == "character" {
            if let Some(character) = &row.character {
                return Ok(format!("{character}({})", char_len(field)));
            }
        }
        if field.type_name == "decimal" {
            if let Some(rule) = &row.decimal {
            let ps = decimal_precision_scale(field)?;
            return match rule {
                DecimalRule::Required { type_name } => {
                    let (p, s) = ps.ok_or_else(|| {
                        DatasourceError::message(format!(
                            "decimal field \"{}\" requires size: [precision, scale] for {}",
                            field.name.as_deref().unwrap_or(""),
                            row.dialect_name.as_deref().unwrap_or("sql")
                        ))
                    })?;
                    Ok(format!("{type_name}({p}, {s})"))
                }
                DecimalRule::Optional { type_name, fallback } => Ok(match ps {
                    Some((p, s)) => format!("{type_name}({p}, {s})"),
                    None => fallback.clone(),
                }),
            };
            }
        }
        if field.type_name == "binary" {
            if let (Some(unlimited), Some(render)) = (&row.binary_unlimited, row.binary_sized) {
                return Ok(sized(unlimited, render, field));
            }
        }
        self.dictionary
            .get(&field.type_name)
            .cloned()
            .ok_or_else(|| {
                DatasourceError::message(format!("Unknown spec field type: {}", field.type_name))
            })
    }

    pub fn reverse_convert(&self, native: &str) -> Result<String> {
        Mapping::reverse_convert(self, &native.to_string())
    }
}

impl Mapping<String, String> for DeterministicTypeConverter {
    fn convert(&self, from: &String) -> Result<String> {
        self.dictionary.get(from).cloned().ok_or_else(|| {
            DatasourceError::message(format!("Unknown spec field type: {from}"))
        })
    }

    fn reverse_convert(&self, native: &String) -> Result<String> {
        let spec_types = self.reverse.get(native).ok_or_else(|| {
            DatasourceError::message(format!("Unknown native field type: {native}"))
        })?;
        if spec_types.len() > 1 {
            return Err(DatasourceError::message(format!(
                "Native field type {native} maps to multiple spec types: {}",
                spec_types.join(", ")
            )));
        }
        Ok(spec_types[0].clone())
    }
}

fn sized(unlimited: &str, render: fn(i32) -> String, field: &SpecField) -> String {
    match field.size {
        None | Some(FieldSize::Unlimited) => unlimited.to_string(),
        Some(FieldSize::Length(n)) | Some(FieldSize::Decimal(n, _)) => render(n),
    }
}

fn char_len(field: &SpecField) -> i32 {
    match field.size {
        None | Some(FieldSize::Unlimited) => 1,
        Some(FieldSize::Length(n)) | Some(FieldSize::Decimal(n, _)) => n,
    }
}

fn decimal_precision_scale(field: &SpecField) -> Result<Option<(i32, i32)>> {
    match &field.size {
        None | Some(FieldSize::Unlimited) => Ok(None),
        Some(FieldSize::Decimal(p, s)) => Ok(Some((*p, *s))),
        Some(FieldSize::Length(_)) => Err(DatasourceError::message(format!(
            "decimal field \"{}\" size must be [precision, scale] tuple; got {:?}",
            field.name.as_deref().unwrap_or(""),
            field.size
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_sized_string() {
        let converter = DeterministicTypeConverter::new(
            HashMap::from([("string".into(), "TEXT".into())]),
            SpecFieldRules {
                dialect_name: Some("sqlite".into()),
                string_unlimited: "TEXT".into(),
                string_sized: |n| format!("VARCHAR({n})"),
                character: None,
                decimal: None,
                binary_unlimited: None,
                binary_sized: None,
            },
        );
        let field = SpecField {
            type_name: "string".into(),
            size: Some(FieldSize::Length(64)),
            ..SpecField::default()
        };
        assert_eq!(converter.convert(&field).expect("type"), "VARCHAR(64)");
    }
}
