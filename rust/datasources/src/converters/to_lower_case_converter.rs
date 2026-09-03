pub struct ToLowerCaseConverter;

impl ToLowerCaseConverter {
    pub fn convert(&self, from: &str) -> String {
        from.to_lowercase()
    }
    pub fn reverse_convert(&self, to: String) -> String {
        to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowers() {
        assert_eq!(ToLowerCaseConverter.convert("ADA"), "ada");
        assert_eq!(ToLowerCaseConverter.reverse_convert("ADA".into()), "ADA");
    }
}
