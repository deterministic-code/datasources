use crate::converters::convert_schema::ConvertSchema;
use crate::converters::row_converter::RowConverter;
use crate::converters::value_converter::ValueConverter;
use crate::row::Row;
use crate::result::Result;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TypeConverter {
    pub fields: Vec<String>,
    pub field_types: HashMap<String, String>,
    rows: RowConverter,
}

impl TypeConverter {
    pub fn of<T: ConvertSchema>() -> Self {
        let schema = T::schema();
        Self::new(
            schema
                .iter()
                .map(|(field, _)| field.to_string())
                .collect(),
        )
        .with_field_types(
            schema
                .iter()
                .map(|(field, type_name)| (field.to_string(), type_name.to_string()))
                .collect(),
        )
    }

    pub fn new(fields: Vec<String>) -> Self {
        Self {
            fields,
            field_types: HashMap::new(),
            rows: RowConverter::default(),
        }
    }

    pub fn with_field_types(mut self, field_types: HashMap<String, String>) -> Self {
        self.field_types = field_types.clone();
        self.rows.field_types = field_types;
        self
    }

    pub fn with_types(mut self, types: HashMap<String, ValueConverter>) -> Self {
        self.rows.types.extend(types);
        self
    }

    pub fn with_fields(mut self, fields: HashMap<String, ValueConverter>) -> Self {
        self.rows.fields.extend(fields);
        self
    }

    pub fn convert(&self, row: &Row) -> Result<Row> {
        self.rows.convert(row)
    }

    pub fn reverse_convert(&self, row: &Row) -> Result<Row> {
        self.rows.reverse_convert(row)
    }
}
