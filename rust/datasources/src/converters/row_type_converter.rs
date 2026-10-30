use crate::converters::type_converter::TypeConverter;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;

pub struct RowTypeConverter {
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl Clone for RowTypeConverter {
    fn clone(&self) -> Self {
        Self::new(HashMap::new())
    }
}

impl RowTypeConverter {
    pub fn new(fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>) -> Self {
        Self { fields }
    }

    pub fn from(
        mut catalog: impl FnMut() -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
        columns: &HashMap<String, String>,
    ) -> Self {
        let mut fields = HashMap::new();
        for (name, ty) in columns {
            if let Some(converter) = catalog().remove(ty).or_else(|| {
                ty.ends_with("integer").then(|| catalog().remove("integer")).flatten()
            }) {
                fields.insert(name.clone(), converter);
            }
        }
        Self::new(fields)
    }

    pub fn convert(&self, row: &Row) -> Result<Row> {
        TypeConverter::convert(self, row)
    }

    pub fn reverse_convert(&self, row: &Row) -> Result<Row> {
        TypeConverter::reverse_convert(self, row)
    }

    fn apply(&self, row: &Row, reverse: bool) -> Result<Row> {
        row.iter()
            .map(|(key, value)| {
                let cell = match self.fields.get(key) {
                    Some(field) if reverse => field.reverse_convert(value)?,
                    Some(field) => field.convert(value)?,
                    None => value.clone(),
                };
                Ok((key.clone(), cell))
            })
            .collect()
    }
}

impl TypeConverter<Row, Row> for RowTypeConverter {
    fn convert(&self, from: &Row) -> Result<Row> {
        self.apply(from, false)
    }

    fn reverse_convert(&self, to: &Row) -> Result<Row> {
        self.apply(to, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    fn record(pairs: &[(&str, Value)]) -> Row {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn type_converter_round_trips_rows() {
        let converter = RowTypeConverter::new(HashMap::new());
        let row = record(&[("id", Value::Int(7)), ("name", Value::from("ada"))]);
        let dest = converter.convert(&row).unwrap();
        assert_eq!(dest, row);
        assert_eq!(converter.reverse_convert(&dest).unwrap(), row);
    }

    struct OnlyOne;

    impl TypeConverter<Value, Value> for OnlyOne {
        fn convert(&self, from: &Value) -> Result<Value> {
            Ok(match from {
                Value::Int(number) => Value::Bool(*number == 1),
                other => other.clone(),
            })
        }

        fn reverse_convert(&self, to: &Value) -> Result<Value> {
            Ok(to.clone())
        }
    }

    #[test]
    fn from_binds_catalog_key_to_field() {
        struct TextInt;
        impl TypeConverter<Value, Value> for TextInt {
            fn convert(&self, from: &Value) -> Result<Value> {
                Ok(match from {
                    Value::Text(text) => text.parse::<i64>().map(Value::Int).unwrap_or_else(|_| from.clone()),
                    other => other.clone(),
                })
            }
            fn reverse_convert(&self, to: &Value) -> Result<Value> {
                Ok(to.clone())
            }
        }
        let converter = RowTypeConverter::from(
            || HashMap::from([(
                "integer".into(),
                Box::new(TextInt) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
            )]),
            &HashMap::from([("id".into(), "unsignedinteger".into())]),
        );
        assert_eq!(
            converter.convert(&record(&[("id", Value::from("9"))])).unwrap()["id"],
            Value::Int(9)
        );
        assert_eq!(
            converter.convert(&record(&[("name", Value::from("9"))])).unwrap()["name"],
            Value::from("9")
        );
    }

    #[test]
    fn field_override_beats_type_default() {
        let converter = RowTypeConverter::new(HashMap::from([(
            "is_enabled".into(),
            Box::new(OnlyOne) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        )]));
        let row = record(&[("is_enabled", Value::Int(2))]);
        assert_eq!(
            converter.convert(&row).unwrap()["is_enabled"],
            Value::Bool(false)
        );
        assert_eq!(
            converter
                .convert(&record(&[("is_enabled", Value::Int(1))]))
                .unwrap()["is_enabled"],
            Value::Bool(true)
        );
    }
}
