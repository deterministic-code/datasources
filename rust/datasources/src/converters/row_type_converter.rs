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
        to.iter()
            .map(|(key, value)| {
                Ok((
                    key.clone(),
                    self.fields
                        .get(key)
                        .map(|converter| converter.reverse_convert(value))
                        .transpose()?
                        .unwrap_or_else(|| value.clone()),
                ))
            })
            .collect()
    }
}
