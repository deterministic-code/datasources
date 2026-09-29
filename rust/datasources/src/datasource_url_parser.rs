use std::collections::HashMap;

use crate::DatasourceUrl;

pub struct DatasourceUrlParser;

struct Authority {
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<i32>,
    file: Option<String>,
}

struct Parts {
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<i32>,
    path: String,
}

impl DatasourceUrlParser {
    pub fn parse(url: &str) -> Option<DatasourceUrl> {
        if url.chars().all(char::is_whitespace) {
            return None;
        }
        let colon = scheme_colon(url)?;
        let mut rest = &url[colon + 1..];
        if let Some(hash) = rest.find('#') {
            rest = &rest[..hash];
        }
        let (rest, query) = match rest.find('?') {
            Some(at) => (&rest[..at], &rest[at + 1..]),
            None => (rest, ""),
        };
        let parts = parts_of(rest)?;
        let (schema, options) = query_of(query)?;
        Some(DatasourceUrl {
            provider: url[..colon].to_ascii_lowercase(),
            username: parts.username,
            password: parts.password,
            host: parts.host,
            port: parts.port,
            path: parts.path,
            schema,
            options,
        })
    }
}

fn parts_of(rest: &str) -> Option<Parts> {
    if !rest.starts_with("//") {
        return Some(Parts {
            username: None,
            password: None,
            host: None,
            port: None,
            path: rest.to_string(),
        });
    }
    let body = &rest[2..];
    let slash = body.find('/');
    let authority = authority_of(slash.map(|at| &body[..at]).unwrap_or(body), slash.is_some())?;
    Some(Parts {
        username: authority.username,
        password: authority.password,
        host: authority.host,
        port: authority.port,
        path: authority
            .file
            .unwrap_or_else(|| slash.map(|at| body[at..].to_string()).unwrap_or_default()),
    })
}

fn authority_of(authority: &str, has_path: bool) -> Option<Authority> {
    if authority.is_empty() {
        return Some(Authority {
            username: None,
            password: None,
            host: None,
            port: None,
            file: None,
        });
    }
    let at = authority.rfind('@');
    let (username, password, hostport) = match at {
        Some(at) => {
            let (username, password) = user_of(&authority[..at])?;
            (username, password, &authority[at + 1..])
        }
        None => (None, None, authority),
    };
    if hostport.is_empty() {
        return Some(Authority {
            username,
            password,
            host: None,
            port: None,
            file: None,
        });
    }
    if hostport.starts_with('[') {
        let (host, port) = host_literal(hostport)?;
        return Some(Authority {
            username,
            password,
            host,
            port,
            file: None,
        });
    }
    if let Some(colon) = hostport.rfind(':') {
        if let Some(port) = port_of(&hostport[colon + 1..]) {
            let name = &hostport[..colon];
            let host = if name.is_empty() {
                None
            } else {
                Some(decode(name)?)
            };
            return Some(Authority {
                username,
                password,
                host,
                port: Some(port),
                file: None,
            });
        }
    }
    if !has_path && at.is_none() && !hostport.starts_with(':') {
        return Some(Authority {
            username,
            password,
            host: None,
            port: None,
            file: Some(decode(hostport)?),
        });
    }
    Some(Authority {
        username,
        password,
        host: Some(decode(hostport)?),
        port: None,
        file: None,
    })
}

fn user_of(user: &str) -> Option<(Option<String>, Option<String>)> {
    let colon = user.find(':');
    let username = decode(colon.map(|at| &user[..at]).unwrap_or(user))?;
    let username = if username.is_empty() { None } else { Some(username) };
    let password = match colon {
        Some(at) => Some(decode(&user[at + 1..])?),
        None => None,
    };
    Some((username, password))
}

fn host_literal(hostport: &str) -> Option<(Option<String>, Option<i32>)> {
    let end = hostport.find(']')?;
    if end < 1 {
        return None;
    }
    let host = Some(decode(&hostport[1..end])?);
    if end + 1 == hostport.len() {
        return Some((host, None));
    }
    if !hostport[end + 1..].starts_with(':') {
        return None;
    }
    Some((host, Some(port_of(&hostport[end + 2..])?)))
}

fn query_of(query: &str) -> Option<(Option<String>, HashMap<String, String>)> {
    if query.is_empty() {
        return Some((None, HashMap::new()));
    }
    let mut options = HashMap::new();
    let mut schema = None;
    for part in query.split('&') {
        if part.is_empty() {
            continue;
        }
        let (raw_key, raw_value) = match part.find('=') {
            Some(eq) => (&part[..eq], &part[eq + 1..]),
            None => (part, ""),
        };
        let key = decode(raw_key)?;
        let value = decode(raw_value)?;
        if key == "schema" {
            schema = Some(value);
        } else {
            options.insert(key, value);
        }
    }
    Some((schema, options))
}

fn scheme_colon(url: &str) -> Option<usize> {
    let mut chars = url.chars();
    if !chars.next().is_some_and(scheme_start) {
        return None;
    }
    let mut i = 1;
    for c in chars {
        if c == ':' {
            return Some(i);
        }
        if !scheme_char(c) {
            return None;
        }
        i += c.len_utf8();
    }
    None
}

fn scheme_start(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn scheme_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+' || c == '.' || c == '-'
}

fn port_of(text: &str) -> Option<i32> {
    if text.is_empty() || !text.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

fn decode(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut raw = Vec::new();
    let mut decoded = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'%' {
            if bytes[i].is_ascii() {
                raw.push(bytes[i]);
                i += 1;
            } else {
                flush(&mut raw, &mut decoded)?;
                let rest = std::str::from_utf8(&bytes[i..]).ok()?;
                let ch = rest.chars().next()?;
                decoded.push(ch);
                i += ch.len_utf8();
            }
            continue;
        }
        if i + 2 >= bytes.len() {
            return None;
        }
        let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok()?;
        raw.push(u8::from_str_radix(hex, 16).ok()?);
        i += 3;
    }
    flush(&mut raw, &mut decoded)?;
    Some(decoded)
}

fn flush(raw: &mut Vec<u8>, decoded: &mut String) -> Option<()> {
    if raw.is_empty() {
        return Some(());
    }
    decoded.push_str(std::str::from_utf8(raw).ok()?);
    raw.clear();
    Some(())
}
