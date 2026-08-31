use crate::result::Result;

pub trait TypeConverter<To, From> {
    fn convert(&self, from: &From) -> Result<To>;
    fn reverse_convert(&self, to: &To) -> Result<From>;
}
