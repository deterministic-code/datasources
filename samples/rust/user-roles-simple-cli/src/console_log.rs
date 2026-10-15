use regex::{Captures, Regex};
use std::sync::OnceLock;

fn regex(source: &str) -> &'static Regex {
    static DATE: OnceLock<Regex> = OnceLock::new();
    static UUID: OnceLock<Regex> = OnceLock::new();
    static IDENT: OnceLock<Regex> = OnceLock::new();
    static PASSWORD: OnceLock<Regex> = OnceLock::new();
    static UPDATED: OnceLock<Regex> = OnceLock::new();
    static TOKEN: OnceLock<Regex> = OnceLock::new();
    match source {
        "date" => DATE.get_or_init(|| {
            Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}\.\d{3}Z?").expect("date")
        }),
        "uuid" => UUID.get_or_init(|| {
            Regex::new(
                r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b",
            )
            .expect("uuid")
        }),
        "ident" => IDENT
            .get_or_init(|| Regex::new(r#"[\"`]([A-Za-z_][A-Za-z0-9_]*)[\"`]"#).expect("ident")),
        "password" => PASSWORD.get_or_init(|| Regex::new(r"password: [^,}\]]+").expect("password")),
        "updated" => UPDATED.get_or_init(|| Regex::new(r"updated: [^,}\]]+").expect("updated")),
        "cast" => {
            static CAST: OnceLock<Regex> = OnceLock::new();
            CAST.get_or_init(|| Regex::new(r"::[A-Z][A-Z0-9_]*").expect("cast"))
        }
        _ => TOKEN.get_or_init(|| Regex::new(r"[A-Za-z0-9+/]{4,}={0,2}").expect("token")),
    }
}

pub fn normalize_console_line(line: &str) -> String {
    let signed = match line.rfind("] ") {
        Some(duration) if line.ends_with("ms") || line.ends_with("μs") => {
            line[..=duration].to_string()
        }
        _ => line.to_string(),
    };
    let dated = regex("date").replace_all(&signed, "<date>");
    let unique = regex("uuid").replace_all(&dated, "<uuid>");
    let quoted = regex("ident").replace_all(&unique, "$1");
    let uncast = regex("cast").replace_all(&quoted, "");
    let secret = regex("password").replace_all(&uncast, "password: <token>");
    let stamped = regex("updated").replace_all(&secret, "updated: <token>");
    regex("token")
        .replace_all(&stamped, |caps: &Captures| {
            let value = &caps[0];
            if value.chars().any(|c| c.is_ascii_digit())
                && value.chars().any(|c| c.is_ascii_alphabetic())
            {
                "<token>".to_string()
            } else {
                value.to_string()
            }
        })
        .into_owned()
}

pub fn sql_text(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter(|line| line.contains("][Start]-") || line.contains("][Finish]-"))
        .map(|line| normalize_console_line(line))
        .collect()
}

fn mask_update_result(line: &str) -> String {
    let Some(at) = line.find("result: ") else {
        return line.to_string();
    };
    let rest = &line[at + "result: ".len()..];
    let end = rest
        .find(|c: char| c == ',' || c == '}')
        .unwrap_or(rest.len());
    format!("{}result: <id>{}", &line[..at], &rest[end..])
}

pub fn dialect_transcript(lines: &[String]) -> Vec<String> {
    let mut update = false;
    lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let text = normalize_console_line(line)
                .replace(": True", ": 1")
                .replace(": False", ": 0");
            if text.contains("][Start]-") {
                update = text.contains("][Start]-[UPDATE ");
            }
            if update && text.contains("][AfterCommand]-") {
                mask_update_result(&text)
            } else {
                text
            }
        })
        .collect()
}
