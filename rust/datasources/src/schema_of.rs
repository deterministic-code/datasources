use crate::converters::{is_column, struct_fields, SchemaObject};
use crate::mappers::{DeterministicTypeMapper, Mapper};
use crate::schema::{Schema, SchemaColumn};
use facet::Def;
use std::collections::HashMap;

pub struct SchemaOf;

impl SchemaOf {
    pub fn from<T: SchemaObject>() -> Schema {
        let mapper = DeterministicTypeMapper::new();
        let Ok(fields) = struct_fields(T::SHAPE) else {
            return Schema::default();
        };
        let mut columns = HashMap::new();
        for field in fields {
            if !is_column(field.shape()) {
                continue;
            }
            let mapped = mapper.map(kind(field.shape()).into());
            if mapped.is_empty() {
                continue;
            }
            columns.insert(field.name.to_string(), SchemaColumn::new(mapped));
        }
        Schema::new(columns)
    }
}

#[cfg(test)]
mod tests {
    use super::SchemaOf;
    use crate::converters::SchemaObject;
    use facet::Facet;

    #[derive(Clone, Default, Facet)]
    struct User {
        id: i64,
        username: String,
        is_enabled: bool,
    }

    impl SchemaObject for User {}

    #[test]
    fn from_maps_facet_fields_to_canonical_types() {
        let schema = SchemaOf::from::<User>();
        assert_eq!(schema.types().get("id").map(String::as_str), Some("biginteger"));
        assert_eq!(schema.types().get("username"), None);
        assert_eq!(schema.types().get("is_enabled").map(String::as_str), Some("boolean"));
    }
}

fn kind(shape: &'static facet::Shape) -> &'static str {
    match shape.def {
        Def::Option(option) => option.t().type_identifier,
        _ => shape.type_identifier,
    }
}
