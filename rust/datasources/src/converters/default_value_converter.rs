use crate::result::Result;

pub trait DefaultValueConverter: Send + Sync {
    fn convert(&self, spec_type: &str, text: &str) -> Result<Option<String>>;
    fn reverse_convert(&self, spec_type: &str, native: &str) -> Result<String>;
}
