use crate::converters::core_types::core_types;
use crate::converters::value_converter::ValueConverter;
use crate::result::Result;
use crate::row::Row;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RowConverter {
    pub types: HashMap<String, ValueConverter>,
    pub fields: HashMap<String, ValueConverter>,
    pub field_types: HashMap<String, String>,
}

impl Default for RowConverter {
    fn default() -> Self {
        Self::new(HashMap::new(), HashMap::new())
    }
}

impl RowConverter {
    pub fn new(
        types: HashMap<String, ValueConverter>,
        fields: HashMap<String, ValueConverter>,
    ) -> Self {
        let mut merged = core_types();
        merged.extend(types);
        Self {
            types: merged,
            fields,
            field_types: HashMap::new(),
        }
    }

    pub fn with_field_types(mut self, field_types: HashMap<String, String>) -> Self {
        self.field_types = field_types;
        self
    }

    pub fn convert(&self, row: &Row) -> Result<Row> {
        self.convert_typed(row, &self.field_types)
    }

    pub fn convert_typed(
        &self,
        row: &Row,
        field_types: &HashMap<String, String>,
    ) -> Result<Row> {
        row.iter()
            .map(|(key, value)| {
                Ok((
                    key.clone(),
                    self.pick(key, field_types.get(key).map(String::as_str))
                        .map(|converter| converter.convert(value))
                        .transpose()?
                        .unwrap_or_else(|| value.clone()),
                ))
            })
            .collect()
    }

    pub fn reverse_convert(&self, row: &Row) -> Result<Row> {
        self.reverse_convert_typed(row, &self.field_types)
    }

    pub fn reverse_convert_typed(
        &self,
        row: &Row,
        field_types: &HashMap<String, String>,
    ) -> Result<Row> {
        row.iter()
            .map(|(key, value)| {
                Ok((
                    key.clone(),
                    self.pick(key, field_types.get(key).map(String::as_str))
                        .map(|converter| converter.reverse_convert(value))
                        .transpose()?
                        .unwrap_or_else(|| value.clone()),
                ))
            })
            .collect()
    }

    fn pick(&self, key: &str, type_name: Option<&str>) -> Option<&ValueConverter> {
        self.fields
            .get(key)
            .or_else(|| type_name.and_then(|name| self.types.get(name)))
    }
}
