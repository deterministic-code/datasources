pub struct ReplaceConverter {
    search: String,
    replacement: String,
}

impl ReplaceConverter {
    pub fn new(search: impl Into<String>, replacement: impl Into<String>) -> Self {
        Self {
            search: search.into(),
            replacement: replacement.into(),
        }
    }

    pub fn convert(&self, from: &str) -> String {
        from.replace(&self.search, &self.replacement)
    }

    pub fn reverse_convert(&self, to: String) -> String {
        to.replace(&self.replacement, &self.search)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_literals() {
        assert_eq!(
            ReplaceConverter::new("_", " ").convert("Ada_Lovelace"),
            "Ada Lovelace"
        );
        assert_eq!(
            ReplaceConverter::new("_", " ").reverse_convert("Ada Lovelace".into()),
            "Ada_Lovelace"
        );
    }
}
