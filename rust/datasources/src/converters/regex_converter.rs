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
        self.regex
            .replace_all(from, self.replacement.as_str())
            .into_owned()
    }

    pub fn reverse_convert(&self, to: String) -> String {
        to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_with_regex() {
        assert_eq!(
            RegexConverter::new(r"\d+", "").convert("Ada 1 Lovelace"),
            "Ada  Lovelace"
        );
        assert_eq!(
            RegexConverter::new(r"id_(\d+)", "id-$1").convert("id_7"),
            "id-7"
        );
        assert_eq!(
            RegexConverter::new(r"id_(\d+)", "id-$1").reverse_convert("id_7".into()),
            "id_7"
        );
    }
}
