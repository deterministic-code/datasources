#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Casing {
    Camel,
    Capital,
    Constant,
    Dot,
    Kebab,
    No,
    Pascal,
    PascalSnake,
    Path,
    Sentence,
    Snake,
    Train,
}

pub struct ChangeCasingConverter(Casing);

impl ChangeCasingConverter {
    pub fn new(casing: Casing) -> Self {
        Self(casing)
    }

    pub fn convert(&self, from: &str) -> String {
        join(&words(from), self.0)
    }

    pub fn reverse_convert(&self, to: String) -> String {
        to
    }
}

fn cap(word: &str) -> String {
    let mut chars = word.chars();
    chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
}

fn sep(c: char) -> bool {
    matches!(c, '_' | '.' | '-' | '/') || c.is_whitespace()
}

fn words(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut parts = Vec::new();
    let mut start = 0;
    for i in 0..chars.len() {
        let split = sep(chars[i])
            || (i > 0 && chars[i - 1].is_lowercase() && chars[i].is_uppercase())
            || (i > 0
                && i + 1 < chars.len()
                && chars[i - 1].is_uppercase()
                && chars[i].is_uppercase()
                && chars[i + 1].is_lowercase())
            || (i > 0
                && chars[i - 1].is_alphabetic() != chars[i].is_alphabetic()
                && chars[i - 1].is_alphanumeric()
                && chars[i].is_alphanumeric());
        if !split {
            continue;
        }
        if i > start {
            parts.push(chars[start..i].iter().collect::<String>());
        }
        start = if sep(chars[i]) { i + 1 } else { i };
    }
    if start < chars.len() {
        parts.push(chars[start..].iter().collect::<String>());
    }
    parts
        .into_iter()
        .map(|word| word.to_lowercase())
        .filter(|word| !word.is_empty())
        .collect()
}

fn join(parts: &[String], casing: Casing) -> String {
    match casing {
        Casing::Camel => parts
            .iter()
            .enumerate()
            .map(|(i, word)| if i == 0 { word.clone() } else { cap(word) })
            .collect(),
        Casing::Capital => parts
            .iter()
            .map(|word| cap(word))
            .collect::<Vec<_>>()
            .join(" "),
        Casing::Constant => parts
            .iter()
            .map(|word| word.to_uppercase())
            .collect::<Vec<_>>()
            .join("_"),
        Casing::Dot => parts.join("."),
        Casing::Kebab => parts.join("-"),
        Casing::No => parts.join(" "),
        Casing::Pascal => parts.iter().map(|word| cap(word)).collect(),
        Casing::PascalSnake => parts
            .iter()
            .map(|word| cap(word))
            .collect::<Vec<_>>()
            .join("_"),
        Casing::Path => parts.join("/"),
        Casing::Sentence => parts
            .iter()
            .enumerate()
            .map(|(i, word)| if i == 0 { cap(word) } else { word.clone() })
            .collect::<Vec<_>>()
            .join(" "),
        Casing::Snake => parts.join("_"),
        Casing::Train => parts
            .iter()
            .map(|word| cap(word))
            .collect::<Vec<_>>()
            .join("-"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_casing() {
        let forms = [
            (Casing::Camel, "twoWords"),
            (Casing::Capital, "Two Words"),
            (Casing::Constant, "TWO_WORDS"),
            (Casing::Dot, "two.words"),
            (Casing::Kebab, "two-words"),
            (Casing::No, "two words"),
            (Casing::Pascal, "TwoWords"),
            (Casing::PascalSnake, "Two_Words"),
            (Casing::Path, "two/words"),
            (Casing::Sentence, "Two words"),
            (Casing::Snake, "two_words"),
            (Casing::Train, "Two-Words"),
        ];
        for (_, from) in forms {
            for (casing, expected) in forms {
                assert_eq!(ChangeCasingConverter::new(casing).convert(from), expected);
            }
            assert_eq!(
                ChangeCasingConverter::new(Casing::Snake).reverse_convert(from.into()),
                from
            );
        }
        assert_eq!(
            ChangeCasingConverter::new(Casing::Camel).convert("XMLParser"),
            "xmlParser"
        );
        assert_eq!(
            ChangeCasingConverter::new(Casing::Camel).convert("html5"),
            "html5"
        );
        assert_eq!(
            ChangeCasingConverter::new(Casing::Camel).convert("v2Name"),
            "v2Name"
        );
        assert_eq!(ChangeCasingConverter::new(Casing::Snake).convert(""), "");
        assert_eq!(ChangeCasingConverter::new(Casing::Snake).convert("___"), "");
        assert_eq!(
            ChangeCasingConverter::new(Casing::Snake).convert(" two  words\t"),
            "two_words"
        );
        assert_eq!(ChangeCasingConverter::new(Casing::Camel).convert(""), "");
    }
}
