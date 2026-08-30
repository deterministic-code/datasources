pub struct ToLowerCaseConverter;

impl ToLowerCaseConverter {
    pub fn convert(&self, from: &str) -> String { from.to_lowercase() }
    pub fn reverse_convert(&self, to: String) -> String { to }
}
