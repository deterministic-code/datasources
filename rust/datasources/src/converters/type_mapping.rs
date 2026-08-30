use crate::converters::field_converter::FieldConverter;
use crate::converters::identity_field_converter::identity_field_converter;
use crate::converters::normalize_sql_type::normalize_sql_type;
use crate::converters::provider_types::ProviderTypes;
use crate::record::Record;
use crate::result::Result;
use std::collections::HashMap;

pub struct TypeMapping {
    pub name: String,
    pub source: String,
    fields: HashMap<String, String>,
    table: ProviderTypes,
    converters: HashMap<String, FieldConverter>,
}

impl TypeMapping {
    pub(crate) fn new(
        name: String,
        source: String,
        fields: HashMap<String, String>,
        table: ProviderTypes,
        converters: HashMap<String, FieldConverter>,
    ) -> Self {
        Self {
            name,
            source,
            fields,
            table,
            converters,
        }
    }

    pub fn field_names(&self) -> Vec<String> {
        self.fields.keys().cloned().collect()
    }

    fn converter_for(&self, key: &str) -> FieldConverter {
        if let Some(converter) = self.converters.get(key) {
            return converter.clone();
        }
        self.fields
            .get(key)
            .and_then(|sql_type| self.table.types.get(&normalize_sql_type(sql_type)))
            .cloned()
            .unwrap_or_else(identity_field_converter)
    }

    pub fn to(&self, row: &Record) -> Result<Record> {
        let mut out = Record::new();
        for (key, value) in row {
            out.insert(key.clone(), self.converter_for(key).to(value)?);
        }
        Ok(out)
    }

    pub fn from(&self, obj: &Record) -> Result<Record> {
        let mut out = Record::new();
        for (key, value) in obj {
            out.insert(key.clone(), self.converter_for(key).from(value)?);
        }
        Ok(out)
    }
}
