use super::keyword_casing::KeywordCasing;
use crate::converters::{Casing, ChangeCasingConverter, PluralizerConverter};
use regex::Regex;
use std::sync::LazyLock;

static SNAKE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z_][a-z0-9_]*$").expect("snake"));
static CAMEL_BOUNDARY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([a-z0-9])([A-Z])").expect("camel"));
static ACRONYM_BOUNDARY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([A-Z]+)([A-Z][a-z])").expect("acronym"));
static SEPARATORS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[_\-.]+").expect("sep"));

#[derive(Clone, Debug)]
pub struct StatementNaming {
    pub table_name_casing: Casing,
    pub field_name_casing: Casing,
    pub constraint_name_casing: Casing,
    pub trigger_name_casing: Casing,
    pub keyword_casing: KeywordCasing,
    pub pluralize_tables: bool,
    pub objects: String,
}

impl Default for StatementNaming {
    fn default() -> Self {
        Self {
            table_name_casing: Casing::Snake,
            field_name_casing: Casing::Snake,
            constraint_name_casing: Casing::Snake,
            trigger_name_casing: Casing::Snake,
            keyword_casing: KeywordCasing::Upper,
            pluralize_tables: true,
            objects: "preserve".into(),
        }
    }
}

impl StatementNaming {
    pub fn sql() -> Self {
        Self::default()
    }

    pub fn is_verbatim(&self, mapping: &str) -> bool {
        !SNAKE.is_match(mapping)
    }

    pub fn physical_stem(&self, name: &str) -> String {
        if self.is_verbatim(name) {
            name.into()
        } else if self.pluralize_tables {
            last_token(name)
        } else {
            name.into()
        }
    }

    pub fn convert(&self, text: &str, casing: Casing) -> String {
        let words = identifier_words(text);
        if words.is_empty() {
            return text.into();
        }
        match casing {
            Casing::Camel => words
                .iter()
                .enumerate()
                .map(|(i, word)| if i == 0 { word.clone() } else { cap(word) })
                .collect(),
            Casing::Pascal => words.iter().map(|word| cap(word)).collect(),
            Casing::Snake => words.join("_"),
            Casing::Kebab => words.join("-"),
            other => ChangeCasingConverter::new(other).convert(text),
        }
    }

    pub fn letter(&self, text: &str) -> String {
        match self.objects.as_str() {
            "upper" => text.to_ascii_uppercase(),
            "lower" => text.to_ascii_lowercase(),
            _ => text.into(),
        }
    }

    pub fn keyword(&self, text: &str) -> String {
        if self.keyword_casing == KeywordCasing::Lower {
            text.to_ascii_lowercase()
        } else {
            text.to_ascii_uppercase()
        }
    }
}

fn identifier_words(name: &str) -> Vec<String> {
    let first = CAMEL_BOUNDARY.replace_all(name, "$1 $2");
    let second = ACRONYM_BOUNDARY.replace_all(&first, "$1 $2");
    let stepped = SEPARATORS.replace_all(&second, " ");
    stepped
        .trim()
        .split_whitespace()
        .map(|word| word.to_ascii_lowercase())
        .collect()
}

fn cap(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn last_token(name: &str) -> String {
    match name.rfind('_') {
        None => PluralizerConverter.convert(name),
        Some(at) => format!(
            "{}{}",
            &name[..=at],
            PluralizerConverter.convert(&name[at + 1..])
        ),
    }
}
