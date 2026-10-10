use crate::converters::convert_schema::ConvertSchema;
use crate::converters::type_converter::TypeConverter;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;

pub struct RowTypeConverter {
    columns: Vec<String>,
    pub types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl Clone for RowTypeConverter {
    fn clone(&self) -> Self {
        Self::new(self.columns.clone())
    }
}

impl RowTypeConverter {
    pub fn of<T: ConvertSchema>() -> Self {
        let schema = T::schema();
        Self::new(schema.iter().map(|(field, _)| field.to_string()).collect())
    }

    pub fn new(columns: Vec<String>) -> Self {
        Self {
            columns,
            types: HashMap::new(),
            fields: HashMap::new(),
        }
    }

    pub fn with_types(
        mut self,
        types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    ) -> Self {
        self.types.extend(types);
        self
    }

    pub fn with_fields(
        mut self,
        fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    ) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn from(
        mut catalog: impl FnMut() -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
        columns: &HashMap<String, String>,
    ) -> Self {
        let names: Vec<String> = columns.keys().cloned().collect();
        let mut fields = HashMap::new();
        for (name, ty) in columns {
            if let Some(converter) = catalog().remove(ty) {
                fields.insert(name.clone(), converter);
            }
        }
        Self::new(names).with_fields(fields)
    }

    pub(crate) fn column_names(&self) -> &[String] {
        &self.columns
    }

    pub fn convert(&self, row: &Row) -> Result<Row> {
        TypeConverter::convert(self, row)
    }

    pub fn reverse_convert(&self, row: &Row) -> Result<Row> {
        TypeConverter::reverse_convert(self, row)
    }
}

impl TypeConverter<Row, Row> for RowTypeConverter {
    fn convert(&self, from: &Row) -> Result<Row> {
        from.iter()
            .map(|(key, value)| {
                Ok((
                    key.clone(),
                    self.fields
                        .get(key)
                        .map(|converter| converter.convert(value))
                        .transpose()?
                        .unwrap_or_else(|| value.clone()),
                ))
            })
            .collect()
    }

    fn reverse_convert(&self, to: &Row) -> Result<Row> {
        let keys = if self.columns.is_empty() {
            to.keys().cloned().collect::<Vec<_>>()
        } else {
            self.columns.clone()
        };
        let mut row = Row::new();
        for key in keys {
            let Some(value) = to.get(&key) else { continue };
            row.insert(
                key.clone(),
                self.fields
                    .get(&key)
                    .map(|converter| converter.reverse_convert(value))
                    .transpose()?
                    .unwrap_or_else(|| value.clone()),
            );
        }
        Ok(row)
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
        let converter = RowTypeConverter::new(vec!["id".into(), "name".into()]);
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
    fn field_override_beats_type_default() {
        let converter =
            RowTypeConverter::new(vec!["is_enabled".into()]).with_fields(HashMap::from([(
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
