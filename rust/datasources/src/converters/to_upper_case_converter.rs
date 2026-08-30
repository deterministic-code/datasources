pub struct ToUpperCaseConverter;

impl ToUpperCaseConverter {
    pub fn convert(&self, from: &str) -> String { from.to_uppercase() }
    pub fn reverse_convert(&self, to: String) -> String { to }
}
