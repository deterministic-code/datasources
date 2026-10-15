use crate::spec::SpecType;
use datasources::{DatasourceError, Result};
use regex::Regex;
use std::sync::OnceLock;

const ALLOWED: &[&str] = &[
    "__name", "__tags", "__inherits", "includes", "true", "false", "null", "undefined", "type",
    "tag", "inherits",
];

pub fn types_filter(filter: Option<&str>) -> Result<Box<dyn Fn(&SpecType) -> bool>> {
    of(filter, "types.filter")
}

pub fn of(filter: Option<&str>, context: &str) -> Result<Box<dyn Fn(&SpecType) -> bool>> {
    let Some(filter) = filter.filter(|f| !f.is_empty()) else {
        return Ok(Box::new(|_| true));
    };
    static STR: OnceLock<Regex> = OnceLock::new();
    let str_re = STR.get_or_init(|| Regex::new(r#""[^"]*""#).expect("string"));
    let mut placeholders = Vec::new();
    let with_strings = str_re.replace_all(filter, |caps: &regex::Captures| {
        let inner = &caps.get(0).unwrap().as_str()[1..caps.get(0).unwrap().as_str().len() - 1];
        placeholders.push(inner.to_string());
        format!("__STR{}__", placeholders.len() - 1)
    });
    let rewritten = rewrite(&with_strings);
    static IDENT: OnceLock<Regex> = OnceLock::new();
    let ident_re = IDENT.get_or_init(|| Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").expect("ident"));
    for ident in ident_re.find_iter(&rewritten) {
        if ident.as_str().starts_with("__STR") || ALLOWED.contains(&ident.as_str()) {
            continue;
        }
        return Err(DatasourceError::message(format!(
            "{context}: unknown identifier or syntax near \"{}\". Supported: `type == \"name\"`, `tag == \"kind\"`, `inherits == \"parent\"`, logical && / ||, parens.",
            ident.as_str()
        )));
    }
    let expr = restore(&rewritten, &placeholders);
    Ok(Box::new(move |type_: &SpecType| {
        eval(&expr, &type_.name, &type_.tags, type_.inherits.as_deref().unwrap_or(""))
    }))
}

fn rewrite(input: &str) -> String {
    static TYPE: OnceLock<Regex> = OnceLock::new();
    static TAG_EQ: OnceLock<Regex> = OnceLock::new();
    static TAG_NE: OnceLock<Regex> = OnceLock::new();
    static INH: OnceLock<Regex> = OnceLock::new();
    let type_re = TYPE.get_or_init(|| Regex::new(r"\btype\s*(==|!=)\s*(__STR\d+__)").expect("type"));
    let tag_eq = TAG_EQ.get_or_init(|| Regex::new(r"\btag\s*==\s*(__STR\d+__)").expect("tag eq"));
    let tag_ne = TAG_NE.get_or_init(|| Regex::new(r"\btag\s*!=\s*(__STR\d+__)").expect("tag ne"));
    let inh = INH.get_or_init(|| Regex::new(r"\binherits\s*(==|!=)\s*(__STR\d+__)").expect("inh"));
    let type_ = type_re.replace_all(input, |m: &regex::Captures| {
        format!("( __name {} {} )", m.get(1).unwrap().as_str(), m.get(2).unwrap().as_str())
    });
    let tag_eq_s = tag_eq.replace_all(&type_, |m: &regex::Captures| {
        format!("__tags.includes({})", m.get(1).unwrap().as_str())
    });
    let tag_ne_s = tag_ne.replace_all(&tag_eq_s, |m: &regex::Captures| {
        format!("!__tags.includes({})", m.get(1).unwrap().as_str())
    });
    inh.replace_all(&tag_ne_s, |m: &regex::Captures| {
        format!("( __inherits {} {} )", m.get(1).unwrap().as_str(), m.get(2).unwrap().as_str())
    })
    .into_owned()
}

fn restore(rewritten: &str, placeholders: &[String]) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"__STR(\d+)__").expect("str idx"));
    re.replace_all(rewritten, |m: &regex::Captures| {
        let i: usize = m.get(1).unwrap().as_str().parse().unwrap_or(0);
        format!("\"{}\"", placeholders.get(i).cloned().unwrap_or_default())
    })
    .into_owned()
}

fn eval(expr: &str, name: &str, tags: &[String], inherits: &str) -> bool {
    or(&expr.replace(' ', ""), name, tags, inherits)
}

fn or(expr: &str, name: &str, tags: &[String], inherits: &str) -> bool {
    let parts = split(expr, "||");
    parts.is_empty() || parts.iter().any(|part| and(part, name, tags, inherits))
}

fn and(expr: &str, name: &str, tags: &[String], inherits: &str) -> bool {
    split(expr, "&&").iter().all(|part| atom(part, name, tags, inherits))
}

fn atom(expr: &str, name: &str, tags: &[String], inherits: &str) -> bool {
    if expr.starts_with('!') && !expr.starts_with("!=") {
        return !atom(&expr[1..], name, tags, inherits);
    }
    if expr.starts_with('(') && expr.ends_with(')') {
        return or(&expr[1..expr.len() - 1], name, tags, inherits);
    }
    static INCLUDE: OnceLock<Regex> = OnceLock::new();
    static NOT_INCLUDE: OnceLock<Regex> = OnceLock::new();
    static CMP: OnceLock<Regex> = OnceLock::new();
    let include = INCLUDE.get_or_init(|| Regex::new(r#"^__tags\.includes\("([^"]*)"\)$"#).expect("inc"));
    if let Some(m) = include.captures(expr) {
        return tags.iter().any(|t| t == &m[1]);
    }
    let not_include = NOT_INCLUDE.get_or_init(|| Regex::new(r#"^!__tags\.includes\("([^"]*)"\)$"#).expect("ninc"));
    if let Some(m) = not_include.captures(expr) {
        return !tags.iter().any(|t| t == &m[1]);
    }
    let cmp = CMP.get_or_init(|| Regex::new(r#"^\(__(name|inherits)(==|!=)"([^"]*)"\)$"#).expect("cmp"));
    let Some(m) = cmp.captures(expr) else {
        panic!("types.filter is not a valid expression: {expr}");
    };
    let left = if &m[1] == "name" { name } else { inherits };
    if &m[2] == "==" {
        left == &m[3]
    } else {
        left != &m[3]
    }
}

fn split(expr: &str, op: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    let bytes = expr.as_bytes();
    let mut i = 0;
    while i + op.len() <= bytes.len() {
        let c = bytes[i] as char;
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
        } else if depth == 0 && &expr[i..i + op.len()] == op {
            parts.push(expr[start..i].to_string());
            i += op.len() - 1;
            start = i + 1;
        }
        i += 1;
    }
    parts.push(expr[start..].to_string());
    parts
}
