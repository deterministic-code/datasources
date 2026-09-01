pub struct ToUpperCaseConverter;

impl ToUpperCaseConverter {
    pub fn convert(&self, from: &str) -> String { from.to_uppercase() }
    pub fn reverse_convert(&self, to: String) -> String { to }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppers() {
        assert_eq!(ToUpperCaseConverter.convert("ada"), "ADA");
        assert_eq!(ToUpperCaseConverter.reverse_convert("ada".into()), "ada");
    }
}
