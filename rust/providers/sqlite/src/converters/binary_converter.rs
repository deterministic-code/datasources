use datasources::{Result, TypeConverter, Value};

pub struct BinaryConverter;

impl TypeConverter<Value, Value> for BinaryConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(from.clone())
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(to.clone())
    }
}
