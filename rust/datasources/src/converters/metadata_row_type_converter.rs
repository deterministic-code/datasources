use crate::converters::convert_schema::ConvertSchema;
use crate::converters::type_converter::TypeConverter;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;

pub struct MetaDataRowTypeConverter {
    columns: Vec<String>,
    pub types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
}

impl MetaDataRowTypeConverter {
    pub fn of<T: ConvertSchema>() -> Self {
        Self::new(
            T::schema()
                .into_iter()
                .map(|(field, _)| field.to_string())
                .collect(),
        )
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

    pub fn convert(&self, row: &Row) -> Result<Row> {
        self.apply(row, false)
    }

    pub fn reverse_convert(&self, row: &Row) -> Result<Row> {
        self.apply(row, true)
    }

    fn apply(&self, row: &Row, reverse: bool) -> Result<Row> {
        let names = match self.columns.is_empty() {
            true => row.keys().cloned().collect(),
            false => self.columns.clone(),
        };
        names
            .into_iter()
            .filter_map(|key| row.get(&key).cloned().map(|value| (key, value)))
            .map(|(key, value)| {
                let cell = match self.fields.get(&key) {
                    Some(field) if reverse => field.reverse_convert(&value)?,
                    Some(field) => field.convert(&value)?,
                    None => value,
                };
                Ok((key, cell))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(pairs: &[(&str, Value)]) -> Row {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), value.clone()))
            .collect()
    }

    struct User;

    impl ConvertSchema for User {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![("id", "INTEGER"), ("name", "TEXT")]
        }
    }

    #[test]
    fn copies_schema_columns_and_drops_the_rest() {
        let converter = MetaDataRowTypeConverter::of::<User>();
        let row = record(&[
            ("id", Value::Int(1)),
            ("name", Value::from("ada")),
            ("extra", Value::from("no")),
        ]);
        let converted = converter.convert(&row).unwrap();
        assert_eq!(converted.get("id"), Some(&Value::Int(1)));
        assert_eq!(converted.get("name"), Some(&Value::from("ada")));
        assert_eq!(converted.get("extra"), None);
        assert_eq!(converter.reverse_convert(&converted).unwrap(), converted);
    }

    #[test]
    fn empty_schema_copies_every_cell() {
        let converter = MetaDataRowTypeConverter::new(Vec::new());
        let row = record(&[("id", Value::Int(1)), ("label", Value::from("a"))]);
        assert_eq!(converter.convert(&row).unwrap(), row);
    }
}
