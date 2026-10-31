use crate::{SchemaColumn, Value};

pub(crate) fn of(column: &SchemaColumn) -> Value {
    match (column.nullable, column.type_name.as_str()) {
        (true, _) => Value::Null,
        (_, "string") => Value::from(""),
        (_, "boolean") => Value::from(false),
        (_, type_name) if type_name.ends_with("integer") => Value::Int(0),
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SchemaColumn;

    #[test]
    fn nullable_integer_is_null() {
        let column = SchemaColumn {
            type_name: "integer".into(),
            nullable: true,
            has_default: false,
        };
        assert_eq!(of(&column), Value::Null);
    }

    #[test]
    fn required_integer_is_zero() {
        let column = SchemaColumn::new("integer");
        assert_eq!(of(&column), Value::Int(0));
    }

    #[test]
    fn required_string_is_empty() {
        let column = SchemaColumn::new("string");
        assert_eq!(of(&column), Value::from(""));
    }

    #[test]
    fn required_boolean_is_false() {
        let column = SchemaColumn::new("boolean");
        assert_eq!(of(&column), Value::from(false));
    }
}
