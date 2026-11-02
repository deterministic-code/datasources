use crate::converters::{MetaDataRowTypeConverter, SchemaObject, TypeConverter};
use crate::enrichment_payload::{
    payload_from_row, payload_from_struct, set_payload_on_row, set_payload_on_struct,
};
use crate::result::Result;
use crate::row::Row;

pub trait Entity: Clone + Default + Send + Sync + 'static {
    fn object_converter() -> Box<dyn TypeConverter<Self, Row> + Send + Sync>;

    fn payload(&self, name: &str) -> Vec<Row> {
        let _ = name;
        Vec::new()
    }

    fn set_payload(&mut self, name: &str, rows: Vec<Row>) -> Result<()> {
        let _ = (name, rows);
        Ok(())
    }
}

impl Entity for Row {
    fn object_converter() -> Box<dyn TypeConverter<Row, Row> + Send + Sync> {
        Box::new(MetaDataRowTypeConverter::<Row>::new())
    }

    fn payload(&self, name: &str) -> Vec<Row> {
        payload_from_row(self, name)
    }

    fn set_payload(&mut self, name: &str, rows: Vec<Row>) -> Result<()> {
        set_payload_on_row(self, name, rows);
        Ok(())
    }
}

impl<T: SchemaObject> Entity for T {
    fn object_converter() -> Box<dyn TypeConverter<T, Row> + Send + Sync> {
        let mut converter = MetaDataRowTypeConverter::<T>::new();
        converter.should_coerce_id = true;
        Box::new(converter)
    }

    fn payload(&self, name: &str) -> Vec<Row> {
        payload_from_struct(self, name)
    }

    fn set_payload(&mut self, name: &str, rows: Vec<Row>) -> Result<()> {
        *self = set_payload_on_struct(self, name, rows)?;
        Ok(())
    }
}
