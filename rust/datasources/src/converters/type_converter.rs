use crate::converters::convert_schema::ConvertSchema;
use crate::converters::row_converter::RowConverter;
use crate::row::Row;
use crate::result::Result;

#[derive(Clone, Debug)]
pub struct TypeConverter {
    pub name: String,
    pub fields: Vec<String>,
    pub rows: RowConverter,
}

impl TypeConverter {
    pub fn of<T: ConvertSchema>(name: impl Into<String>) -> Self {
        Self::new(
            name,
            T::schema().into_iter().map(|(field, _)| field.to_string()).collect(),
        )
    }

    pub fn new(name: impl Into<String>, fields: Vec<String>) -> Self {
        Self {
            name: name.into(),
            fields,
            rows: RowConverter,
        }
    }

    pub fn field_names(&self) -> Vec<String> {
        self.fields.clone()
    }

    pub fn convert(&self, row: &Row) -> Result<Row> {
        self.rows.convert(row)
    }

    pub fn reverse_convert(&self, obj: &Row) -> Result<Row> {
        self.rows.reverse_convert(obj)
    }
}
