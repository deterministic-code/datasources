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
            "i8" | "i16" => "smallinteger".into(),
            "u8" | "u16" => "unsignedsmallinteger".into(),
            "i32" | "isize" => "integer".into(),
            "u32" | "usize" => "unsignedinteger".into(),
            "i64" => "biginteger".into(),
            "u64" => "unsignedbiginteger".into(),
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
            "smallinteger" => "i16".into(),
            "unsignedsmallinteger" => "u16".into(),
            "integer" => "i32".into(),
            "unsignedinteger" => "u32".into(),
            "biginteger" => "i64".into(),
            "unsignedbiginteger" => "u64".into(),
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
        let types = DeterministicTypeMapper::default();
        assert_eq!(types.map("bool".into()), "boolean");
        assert_eq!(types.map("Boolean".into()), "boolean");
        assert_eq!(types.map("i8".into()), "smallinteger");
        assert_eq!(types.map("i16".into()), "smallinteger");
        assert_eq!(types.map("u8".into()), "unsignedsmallinteger");
        assert_eq!(types.map("u16".into()), "unsignedsmallinteger");
        assert_eq!(types.map("i32".into()), "integer");
        assert_eq!(types.map("isize".into()), "integer");
        assert_eq!(types.map("u32".into()), "unsignedinteger");
        assert_eq!(types.map("usize".into()), "unsignedinteger");
        assert_eq!(types.map("i64".into()), "biginteger");
        assert_eq!(types.map("u64".into()), "unsignedbiginteger");
        assert_eq!(types.map("DateTime<Utc>".into()), "datetime");
        assert_eq!(types.map("DateTime".into()), "datetime");
        assert_eq!(types.map("Uuid".into()), "uuid");
        assert_eq!(types.map("Decimal".into()), "decimal");
        assert_eq!(types.map("Vec<u8>".into()), "binary");
        assert_eq!(types.map("String".into()), "");
        assert_eq!(types.reverse_map("boolean".into()), "bool");
        assert_eq!(types.reverse_map("smallinteger".into()), "i16");
        assert_eq!(types.reverse_map("unsignedsmallinteger".into()), "u16");
        assert_eq!(types.reverse_map("integer".into()), "i32");
        assert_eq!(types.reverse_map("unsignedinteger".into()), "u32");
        assert_eq!(types.reverse_map("biginteger".into()), "i64");
        assert_eq!(types.reverse_map("unsignedbiginteger".into()), "u64");
        assert_eq!(types.reverse_map("uuid".into()), "Uuid");
        assert_eq!(types.reverse_map("datetime".into()), "DateTime<Utc>");
        assert_eq!(types.reverse_map("decimal".into()), "Decimal");
        assert_eq!(types.reverse_map("binary".into()), "Vec<u8>");
        assert_eq!(types.reverse_map("string".into()), "");
    }
}
