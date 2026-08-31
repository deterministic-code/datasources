use regex::Regex;

pub struct RegexConverter {
    regex: Regex,
    replacement: String,
}

impl RegexConverter {
    pub fn new(pattern: &str, replacement: impl Into<String>) -> Self {
        Self {
            regex: Regex::new(pattern).expect("invalid regex"),
            replacement: replacement.into(),
        }
    }

    pub fn convert(&self, from: &str) -> String {
        self.regex.replace_all(from, self.replacement.as_str()).into_owned()
    }

    pub fn reverse_convert(&self, to: String) -> String { to }
}
