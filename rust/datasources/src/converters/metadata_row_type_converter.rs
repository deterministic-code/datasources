use crate::converters::convert_schema::ConvertSchema;
use crate::converters::type_converter::TypeConverter;
use crate::mappers::{IdentityMapper, Mapper};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;

pub struct MetaDataRowTypeConverter {
    columns: Vec<String>,
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    references: Vec<String>,
    reference_columns: Box<dyn Mapper<String, String> + Send + Sync>,
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
            fields: HashMap::new(),
            references: Vec::new(),
            reference_columns: Box::new(IdentityMapper::new()),
        }
    }

    pub fn with_references(
        mut self,
        references: Vec<String>,
        columns: Box<dyn Mapper<String, String> + Send + Sync>,
    ) -> Self {
        self.references = references;
        self.reference_columns = columns;
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
        TypeConverter::convert(self, row)
    }

    pub fn reverse_convert(&self, row: &Row) -> Result<Row> {
        TypeConverter::reverse_convert(self, row)
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
                let key = if reverse && self.references.iter().any(|name| name == &key) {
                    Mapper::map(self.reference_columns.as_ref(), key)
                } else {
                    key
                };
                Ok((key, cell))
            })
            .collect()
    }
}

impl TypeConverter<Row, Row> for MetaDataRowTypeConverter {
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
