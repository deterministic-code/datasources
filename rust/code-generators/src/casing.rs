use crate::sql_keywords;
use datasources::{DatasourceError, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

#[derive(Clone)]
pub struct CasingStrategy {
    file_names: String,
    types: String,
    fields: String,
    directories: String,
}

impl CasingStrategy {
    pub fn sql(settings: &HashMap<String, String>, prefix: &str) -> Result<Self> {
        Ok(Self {
            file_names: leaf(parse(get(settings, &format!("{prefix}.file_names")), &format!("{prefix}.file_names"))?, "Snake"),
            types: leaf(parse(get(settings, &format!("{prefix}.types")), &format!("{prefix}.types"))?, "Snake"),
            fields: leaf(parse(get(settings, &format!("{prefix}.fields")), &format!("{prefix}.fields"))?, "Snake"),
            directories: leaf(parse(get(settings, &format!("{prefix}.directories")), &format!("{prefix}.directories"))?, "Snake"),
        })
    }

    pub fn convert_types(&self, text: &str) -> String {
        to_case(text, &self.types)
    }

    pub fn convert_fields(&self, text: &str) -> String {
        to_case(text, &self.fields)
    }
}

fn leaf(override_format: String, fallback: &str) -> String {
    if override_format == "Auto" {
        fallback.into()
    } else {
        override_format
    }
}

fn parse(raw: Option<&str>, path: &str) -> Result<String> {
    let Some(raw) = raw.filter(|s| !s.is_empty()) else {
        return Ok("Auto".into());
    };
    let map: HashMap<&str, &str> = [
        ("camel", "Camel"),
        ("pascal", "Pascal"),
        ("snake", "Snake"),
        ("kebab", "Kebab"),
        ("auto", "Auto"),
    ]
    .into_iter()
    .collect();
    map.get(raw.to_ascii_lowercase().as_str())
        .map(|s| (*s).to_string())
        .ok_or_else(|| {
            DatasourceError::message(format!(
                "{path} must be one of [Camel, Pascal, Snake, Kebab, Auto] (got \"{raw}\")."
            ))
        })
}

fn get<'a>(settings: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    settings.get(key).map(String::as_str)
}

fn to_case(name: &str, format: &str) -> String {
    let words = words(name);
    if words.is_empty() {
        return name.into();
    }
    match format {
        "Camel" => words
            .iter()
            .enumerate()
            .map(|(i, w)| if i == 0 { w.clone() } else { cap(w) })
            .collect(),
        "Pascal" => words.iter().map(|w| cap(w)).collect(),
        "Snake" => words.join("_"),
        "Kebab" => words.join("-"),
        _ => name.into(),
    }
}

fn words(name: &str) -> Vec<String> {
    static A: OnceLock<Regex> = OnceLock::new();
    static B: OnceLock<Regex> = OnceLock::new();
    static C: OnceLock<Regex> = OnceLock::new();
    let a = A.get_or_init(|| Regex::new(r"([a-z0-9])([A-Z])").expect("a"));
    let b = B.get_or_init(|| Regex::new(r"([A-Z]+)([A-Z][a-z])").expect("b"));
    let c = C.get_or_init(|| Regex::new(r"[_\-.]+").expect("c"));
    let first = a.replace_all(name, "$1 $2");
    let second = b.replace_all(&first, "$1 $2");
    let stepped = c.replace_all(&second, " ");
    stepped
        .trim()
        .split_whitespace()
        .map(|w| w.to_ascii_lowercase())
        .collect()
}

fn cap(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn last_token_pluralize(name: &str) -> String {
    if name.is_empty() {
        return name.into();
    }
    static LAST: OnceLock<Regex> = OnceLock::new();
    LAST.get_or_init(|| Regex::new(r"[^_]+$").expect("last"))
        .replace(name, |caps: &regex::Captures| token(caps.get(0).unwrap().as_str()))
        .into_owned()
}

fn token(token: &str) -> String {
    let irregular: HashMap<&str, &str> = [
        ("person", "people"),
        ("child", "children"),
        ("man", "men"),
        ("woman", "women"),
        ("mouse", "mice"),
        ("goose", "geese"),
        ("foot", "feet"),
        ("tooth", "teeth"),
    ]
    .into_iter()
    .collect();
    let uncountable: HashSet<&str> = [
        "settings", "preferences", "news", "information", "series", "species", "equipment",
    ]
    .into_iter()
    .collect();
    let lower = token.to_ascii_lowercase();
    if uncountable.contains(lower.as_str()) || irregular.values().any(|v| v.eq_ignore_ascii_case(token)) {
        return token.into();
    }
    if let Some(plural) = irregular.get(lower.as_str()) {
        return match_case(token, plural);
    }
    let chars: Vec<char> = token.chars().collect();
    if chars.len() > 1 && chars.last() == Some(&'y') {
        let prev = chars[chars.len() - 2].to_ascii_lowercase();
        if !"aeiou".contains(prev) {
            return format!("{}ies", token[..token.len() - 1].to_string());
        }
    }
    if ends(token, &["s", "x", "z", "ch", "sh"]) {
        return format!("{token}es");
    }
    format!("{token}s")
}

fn ends(token: &str, suffixes: &[&str]) -> bool {
    suffixes.iter().any(|s| token.to_ascii_lowercase().ends_with(s))
}

fn match_case(source: &str, plural: &str) -> String {
    if source.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
        let mut chars = plural.chars();
        chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
    } else {
        plural.into()
    }
}

#[derive(Clone)]
pub struct DatasourceNaming {
    casing: CasingStrategy,
    pluralize_names: bool,
}

impl DatasourceNaming {
    pub fn of(settings: &HashMap<String, String>) -> Result<Self> {
        Ok(Self {
            casing: CasingStrategy::sql(settings, "datasource.casing")?,
            pluralize_names: settings.get("datasource.pluralize_datatable_names").map(String::as_str) != Some("false"),
        })
    }

    pub fn is_verbatim(&self, mapping: &str) -> bool {
        static SNAKE: OnceLock<Regex> = OnceLock::new();
        !SNAKE
            .get_or_init(|| Regex::new(r"^[a-z_][a-z0-9_]*$").expect("snake"))
            .is_match(mapping)
    }

    pub fn physical_stem(&self, name: &str) -> String {
        if self.is_verbatim(name) {
            name.into()
        } else if self.pluralize_names {
            last_token_pluralize(name)
        } else {
            name.into()
        }
    }

    pub fn table_name(&self, stem: &str) -> String {
        self.casing.convert_types(stem)
    }

    pub fn column_name(&self, stem: &str) -> String {
        self.casing.convert_fields(stem)
    }

    pub fn resolve_table(&self, logical: &str) -> String {
        self.resolve(|stem| self.table_name(&self.physical_stem(stem)), logical)
    }

    pub fn resolve_column(&self, logical: &str) -> String {
        self.resolve(|stem| self.column_name(stem), logical)
    }

    fn resolve(&self, convert: impl Fn(&str) -> String, logical: &str) -> String {
        if self.is_verbatim(logical) {
            logical.into()
        } else {
            convert(logical)
        }
    }
}

#[derive(Clone)]
pub struct PackCasing {
    naming: DatasourceNaming,
    always_plural: DatasourceNaming,
    keyword_format: String,
    object_format: String,
}

impl PackCasing {
    pub fn of(settings: &HashMap<String, String>) -> Result<Self> {
        let mut always = settings.clone();
        always.insert("datasource.pluralize_datatable_names".into(), "true".into());
        Ok(Self {
            naming: DatasourceNaming::of(settings)?,
            always_plural: DatasourceNaming::of(&always)?,
            keyword_format: parse_upper_lower(get(settings, "datasource.casing.keywords"), "datasource.casing.keywords", "upper")?,
            object_format: parse_upper_lower(get(settings, "datasource.casing.objects"), "datasource.casing.objects", "preserve")?,
        })
    }

    pub fn table_name(&self, entity: &str) -> String {
        self.object_ident(entity, &self.naming.resolve_table(entity))
    }

    pub fn plural_table_name(&self, entity: &str) -> String {
        self.object_ident(entity, &self.always_plural.resolve_table(entity))
    }

    pub fn column_name(&self, field: &str) -> String {
        self.naming.resolve_column(field)
    }

    pub fn keyword(&self, text: &str) -> String {
        letter(text, &self.keyword_format)
    }

    pub fn apply_keywords(&self, sql: &str) -> String {
        if self.keyword_format == "lower" {
            sql_keywords::apply(sql, |k| self.keyword(k))
        } else {
            sql.into()
        }
    }

    pub fn constraint_name(&self, entity: &str, parts: &[&str]) -> String {
        if self.naming.is_verbatim(entity) {
            let mut bits = vec![entity.to_string()];
            bits.extend(parts.iter().map(|p| (*p).to_string()));
            bits.join("_")
        } else {
            let mut bits = vec![self.naming.physical_stem(entity)];
            bits.extend(parts.iter().map(|p| (*p).to_string()));
            self.object_name(&self.naming.table_name(&bits.join("_")))
        }
    }

    pub fn trigger_name(&self, entity: &str) -> String {
        let stem = self.naming.physical_stem(entity);
        if self.naming.is_verbatim(entity) {
            format!("trg_{entity}_updated_at")
        } else {
            self.object_name(&self.naming.table_name(&format!("trg_{stem}_updated_at")))
        }
    }

    pub fn routine_name(&self, stem: &str) -> String {
        self.object_ident(stem, &self.naming.table_name(stem))
    }

    fn object_ident(&self, stem: &str, converted: &str) -> String {
        if self.naming.is_verbatim(stem) {
            converted.into()
        } else {
            self.object_name(converted)
        }
    }

    fn object_name(&self, text: &str) -> String {
        letter(text, &self.object_format)
    }
}

fn letter(text: &str, format: &str) -> String {
    match format {
        "upper" => text.to_ascii_uppercase(),
        "lower" => text.to_ascii_lowercase(),
        _ => text.into(),
    }
}

fn parse_upper_lower(raw: Option<&str>, path: &str, fallback: &str) -> Result<String> {
    let Some(raw) = raw.filter(|s| !s.is_empty()) else {
        return Ok(fallback.into());
    };
    let parsed = raw.to_ascii_lowercase();
    if parsed == "auto" {
        return Ok(fallback.into());
    }
    if parsed == "upper" || parsed == "lower" {
        Ok(parsed)
    } else {
        Err(DatasourceError::message(format!(
            "{path} must be one of [upper, lower] (got \"{raw}\")."
        )))
    }
}
