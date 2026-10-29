use super::Mapper;

pub struct DeterministicTypeMapper;

impl DeterministicTypeMapper {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DeterministicTypeMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl Mapper<String, String> for DeterministicTypeMapper {
    fn map(&self, value: String) -> String {
        match value.as_str() {
            "bool" | "Boolean" => "boolean".into(),
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "isize" | "usize" => {
                "integer".into()
            }
            "Uuid" => "uuid".into(),
            "DateTime<Utc>" | "DateTime" => "datetime".into(),
            "Decimal" => "decimal".into(),
            "Vec<u8>" => "binary".into(),
            _ => String::new(),
        }
    }

    fn reverse_map(&self, value: String) -> String {
        match value.as_str() {
            "boolean" => "bool".into(),
            "integer" => "i32".into(),
            "uuid" => "Uuid".into(),
            "datetime" => "DateTime<Utc>".into(),
            "decimal" => "Decimal".into(),
            "binary" => "Vec<u8>".into(),
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DeterministicTypeMapper;
    use crate::mappers::Mapper;

    #[test]
    fn maps_language_type_names_to_types_yaml_names() {
        let types = DeterministicTypeMapper::new();
        assert_eq!(types.map("bool".into()), "boolean");
        assert_eq!(types.map("i32".into()), "integer");
        assert_eq!(types.map("DateTime<Utc>".into()), "datetime");
        assert_eq!(types.map("Uuid".into()), "uuid");
        assert_eq!(types.map("Decimal".into()), "decimal");
        assert_eq!(types.map("Vec<u8>".into()), "binary");
        assert_eq!(types.map("String".into()), "");
        assert_eq!(types.reverse_map("boolean".into()), "bool");
        assert_eq!(types.reverse_map("integer".into()), "i32");
    }
}
