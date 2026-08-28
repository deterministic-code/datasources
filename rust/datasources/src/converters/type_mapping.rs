use crate::converters::field_converter::FieldConverter;
use crate::converters::identity_field_converter::identity_field_converter;
use crate::converters::normalize_sql_type::normalize_sql_type;
use crate::converters::provider_types::ProviderTypes;
use crate::mappers::DatasourceMapper;
use crate::record::Record;
use crate::result::Result;
use std::collections::HashMap;

pub struct TypeMapping {
    pub name: String,
    pub source: String,
    pub destination: String,
    pub schema: HashMap<String, String>,
    names: DatasourceMapper,
    table: ProviderTypes,
    converters: HashMap<String, FieldConverter>,
}

impl TypeMapping {
    pub(crate) fn new(
        name: String,
        source: String,
        destination: String,
        schema: HashMap<String, String>,
        names: DatasourceMapper,
        table: ProviderTypes,
        converters: HashMap<String, FieldConverter>,
    ) -> Self {
        Self {
            name,
            source,
            destination,
            schema,
            names,
            table,
            converters,
        }
    }

    fn converter_for(&self, sql_name: &str) -> FieldConverter {
        if let Some(converter) = self.converters.get(sql_name) {
            return converter.clone();
        }
        self.schema
            .get(sql_name)
            .and_then(|sql_type| self.table.types.get(&normalize_sql_type(sql_type)))
            .cloned()
            .unwrap_or_else(identity_field_converter)
    }

    pub fn to(&self, row: &Record) -> Result<Record> {
        let mut out = Record::new();
        for (sql_name, value) in row {
            out.insert(
                self.names.to(sql_name),
                self.converter_for(sql_name).to(value)?,
            );
        }
        Ok(out)
    }

    pub fn from(&self, obj: &Record) -> Result<Record> {
        let mut out = Record::new();
        for (dest_name, value) in obj {
            let sql_name = self.names.from(dest_name);
            out.insert(sql_name.clone(), self.converter_for(&sql_name).from(value)?);
        }
        Ok(out)
    }
}
