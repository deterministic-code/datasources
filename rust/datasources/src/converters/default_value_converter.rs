use super::default_value_options::DefaultValueOptions;
use crate::datasource_error::DatasourceError;
use crate::result::Result;
use regex::Regex;
use std::collections::HashMap;

pub const EMPTY_UUID: &str = "00000000-0000-0000-0000-000000000000";

struct DefaultEntry {
    spec: Regex,
    reverse: Regex,
    native: Box<dyn Fn(&str) -> String + Send + Sync>,
    spec_text: Option<fn(&str) -> String>,
}

pub struct DefaultValueConverter {
    dictionary: HashMap<String, HashMap<String, DefaultEntry>>,
}

impl DefaultValueConverter {
    pub fn new(options: DefaultValueOptions) -> Self {
        Self { dictionary: dictionary_of(options) }
    }

    pub fn convert(&self, spec_type: &str, text: &str) -> Result<Option<String>> {
        let (token, arg) = parse(spec_type, Some(text));
        if token == "None" {
            return Ok(None);
        }
        let entries = self.dictionary.get(spec_type).ok_or_else(|| {
            DatasourceError::message(format!("Unknown spec field type: {spec_type}"))
        })?;
        let Some(entry) = entries.get(&token) else {
            return Ok(None);
        };
        if !entry.spec.is_match(text) {
            return Err(DatasourceError::message(format!(
                "default_value '{text}' is not a valid {spec_type} {token} default"
            )));
        }
        let expr = (entry.native)(arg.as_deref().unwrap_or(""));
        Ok(if expr.is_empty() { None } else { Some(expr) })
    }

    pub fn reverse_convert(&self, spec_type: &str, native: &str) -> Result<String> {
        let entries = self.dictionary.get(spec_type).ok_or_else(|| {
            DatasourceError::message(format!("Unknown spec field type: {spec_type}"))
        })?;
        for (name, entry) in entries {
            if let Some(caps) = entry.reverse.captures(native) {
                let arg = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                return Ok(entry.spec_text.map(|f| f(arg)).unwrap_or_else(|| name.clone()));
            }
        }
        Err(DatasourceError::message(format!(
            "Native default '{native}' is not a valid {spec_type} default"
        )))
    }
}

fn parse(spec_type: &str, value: Option<&str>) -> (String, Option<String>) {
    let Some(v) = value else {
        return ("None".into(), None);
    };
    if let Some(caps) = Regex::new(r"^([A-Za-z]+)\('(.*)'\)$")
        .expect("call")
        .captures(v)
    {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let token = if name == "uuid" { "Uuid" } else { name };
        return (token.into(), caps.get(2).map(|m| m.as_str().to_string()));
    }
    if spec_type == "boolean" {
        return ("Boolean".into(), Some(if v == "true" { "true" } else { "false" }.into()));
    }
    if matches!(
        spec_type,
        "number"
            | "integer"
            | "unsignedinteger"
            | "smallinteger"
            | "unsignedsmallinteger"
            | "biginteger"
            | "unsignedbiginteger"
            | "float"
            | "decimal"
            | "reference"
    ) {
        return ("Numeric".into(), Some(v.into()));
    }
    if spec_type == "datetime" {
        return if matches!(v, "Now" | "UtcNow") {
            (v.into(), None)
        } else {
            ("DateTime".into(), Some(v.into()))
        };
    }
    if spec_type == "uuid" {
        return if matches!(v, "NewId" | "Empty") {
            (v.into(), None)
        } else {
            ("Uuid".into(), Some(v.into()))
        };
    }
    if spec_type == "binary" {
        return ("Hex".into(), Some(v.into()));
    }
    ("String".into(), Some(v.into()))
}

fn sql_string_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn exact(text: &str) -> Regex {
    Regex::new(&format!("^{}$", regex::escape(text))).expect("exact")
}

fn dictionary_of(opts: DefaultValueOptions) -> HashMap<String, HashMap<String, DefaultEntry>> {
    let new_id = (opts.new_id)();
    let now = (opts.now)();
    let utc_now = (opts.utc_now)();
    let quoted = Regex::new(r"^'(.*)'$").expect("quoted");
    let mut dictionary = HashMap::new();
    let quoted_string = HashMap::from([
        entry("String", r"^.*$", quoted.clone(), |arg| sql_string_literal(arg), Some(|arg| arg.to_string())),
        entry("Numeric", r"^.*$", quoted.clone(), |arg| sql_string_literal(arg), Some(|arg| arg.to_string())),
    ]);
    dictionary.insert("string".into(), quoted_string);
    dictionary.insert(
        "boolean".into(),
        HashMap::from([entry(
            "Boolean",
            r"^(true|false)$",
            Regex::new(&format!(
                "^({}|{})$",
                regex::escape(&opts.boolean_on),
                regex::escape(&opts.boolean_off)
            ))
            .expect("bool"),
            {
                let on = opts.boolean_on.clone();
                let off = opts.boolean_off.clone();
                move |arg| if arg == "true" { on.clone() } else { off.clone() }
            },
            None,
        )]),
    );
    dictionary.insert(
        "datetime".into(),
        HashMap::from([
            entry("Now", r"^Now$", exact(&now), {
                let now = now.clone();
                move |_| now.clone()
            }, None),
            entry("UtcNow", r"^UtcNow$", exact(&utc_now), {
                let utc_now = utc_now.clone();
                move |_| utc_now.clone()
            }, None),
        ]),
    );
    dictionary.insert(
        "uuid".into(),
        HashMap::from([entry("NewId", r"^NewId$", exact(&new_id), {
            let new_id = new_id.clone();
            move |_| new_id.clone()
        }, None)]),
    );
    dictionary
}

fn entry(
    name: &str,
    spec: &str,
    reverse: Regex,
    native: impl Fn(&str) -> String + Send + Sync + 'static,
    spec_text: Option<fn(&str) -> String>,
) -> (String, DefaultEntry) {
    (
        name.into(),
        DefaultEntry {
            spec: Regex::new(spec).expect("spec"),
            reverse,
            native: Box::new(native),
            spec_text,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_utc_now() {
        let converter = DefaultValueConverter::new(DefaultValueOptions {
            new_id: || "gen_random_uuid()".into(),
            now: || "LOCALTIMESTAMP".into(),
            utc_now: || "(NOW() AT TIME ZONE 'UTC')".into(),
            boolean_on: "TRUE".into(),
            boolean_off: "FALSE".into(),
            hex: |arg| format!("'\\x{arg}'"),
            hex_reverse: Regex::new(r"^'\\x(.*)'$").expect("hex"),
        });
        assert_eq!(
            converter.convert("datetime", "UtcNow").expect("ok"),
            Some("(NOW() AT TIME ZONE 'UTC')".into())
        );
    }
}
