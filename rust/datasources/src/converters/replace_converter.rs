pub struct ReplaceConverter {
    search: String,
    replacement: String,
}

impl ReplaceConverter {
    pub fn new(search: impl Into<String>, replacement: impl Into<String>) -> Self {
        Self { search: search.into(), replacement: replacement.into() }
    }

    pub fn convert(&self, from: &str) -> String { from.replace(&self.search, &self.replacement) }

    pub fn reverse_convert(&self, to: String) -> String {
        to.replace(&self.replacement, &self.search)
    }
}
