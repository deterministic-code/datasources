use datasources::{Casing, DatasourceError, KeywordCasing, PluralizerConverter, Result, StatementNaming};
use regex::Regex;
use std::collections::HashMap;
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
    match name.rfind('_') {
        None => PluralizerConverter.convert(name),
        Some(at) => format!("{}{}", &name[..=at], PluralizerConverter.convert(&name[at + 1..])),
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
    pub strategy: StatementNaming,
}

impl PackCasing {
    pub fn of(settings: &HashMap<String, String>) -> Result<Self> {
        let mut always = settings.clone();
        always.insert("datasource.pluralize_datatable_names".into(), "true".into());
        let objects = parse_upper_lower(get(settings, "datasource.casing.objects"), "datasource.casing.objects", "preserve")?;
        let keywords = parse_upper_lower(get(settings, "datasource.casing.keywords"), "datasource.casing.keywords", "upper")?;
        let types = identifier(settings, "datasource.casing.types")?;
        Ok(Self {
            naming: DatasourceNaming::of(settings)?,
            always_plural: DatasourceNaming::of(&always)?,
            strategy: StatementNaming {
                table_name_casing: types,
                field_name_casing: identifier(settings, "datasource.casing.fields")?,
                constraint_name_casing: types,
                trigger_name_casing: types,
                keyword_casing: if keywords == "lower" { KeywordCasing::Lower } else { KeywordCasing::Upper },
                pluralize_tables: settings.get("datasource.pluralize_datatable_names").map(String::as_str) != Some("false"),
                objects,
            },
        })
    }

    pub fn is_verbatim(&self, mapping: &str) -> bool {
        self.naming.is_verbatim(mapping)
    }

    pub fn plural_table_name(&self, entity: &str) -> String {
        self.object_ident(entity, &self.always_plural.resolve_table(entity))
    }

    pub fn plural_stem(&self, entity: &str) -> String {
        self.always_plural.physical_stem(entity)
    }

    pub fn routine_name(&self, stem: &str) -> String {
        self.object_ident(stem, &self.naming.table_name(stem))
    }

    fn object_ident(&self, stem: &str, converted: &str) -> String {
        if self.naming.is_verbatim(stem) {
            converted.into()
        } else {
            self.strategy.letter(converted)
        }
    }
}

fn identifier(settings: &HashMap<String, String>, path: &str) -> Result<Casing> {
    let raw = get(settings, path);
    let Some(raw) = raw.filter(|s| !s.is_empty()) else {
        return Ok(Casing::Snake);
    };
    if raw.eq_ignore_ascii_case("auto") {
        return Ok(Casing::Snake);
    }
    match raw.to_ascii_lowercase().as_str() {
        "camel" => Ok(Casing::Camel),
        "pascal" => Ok(Casing::Pascal),
        "snake" => Ok(Casing::Snake),
        "kebab" => Ok(Casing::Kebab),
        _ => Err(DatasourceError::message(format!(
            "{path} must be one of [Camel, Pascal, Snake, Kebab] (got \"{raw}\")."
        ))),
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
