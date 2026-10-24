pub struct PathParts {
    pub owner: String,
    pub last: String,
    pub root: bool,
}

pub fn select_field_aliases(
    table: &str,
    fields: &[String],
    aliases: &[Option<String>],
) -> Vec<Option<String>> {
    let mut result: Vec<Option<String>> = vec![None; fields.len()];
    let mut used = std::collections::HashSet::new();
    for (index, alias) in aliases.iter().enumerate().take(fields.len()) {
        if let Some(name) = alias {
            used.insert(name.clone());
            result[index] = Some(name.clone());
        }
    }
    for (index, field) in fields.iter().enumerate() {
        if result[index].is_none() {
            if let Some(path) = path_of(field, table) {
                if path.root {
                    used.insert(path.last.clone());
                    result[index] = Some(path.last);
                }
            }
        }
    }
    for (index, field) in fields.iter().enumerate() {
        if result[index].is_none() {
            if let Some(path) = path_of(field, table) {
                if !path.root {
                    let alias = if used.contains(&path.last) {
                        format!("{}{}", path.owner, path.last)
                    } else {
                        path.last
                    };
                    used.insert(alias.clone());
                    result[index] = Some(alias);
                }
            }
        }
    }
    result
}

pub fn apply_select_field_alias(field: &str, alias: Option<&str>) -> String {
    match alias {
        Some(name) => format!("{field} AS {name}"),
        None => field.to_string(),
    }
}

pub fn path_of(field: &str, table: &str) -> Option<PathParts> {
    if as_index(field).is_some() || field.contains('(') {
        return None;
    }
    let parts = segments(field);
    if parts.len() < 2 {
        return None;
    }
    Some(PathParts {
        owner: parts[parts.len() - 2].to_string(),
        last: parts[parts.len() - 1].to_string(),
        root: parts.len() == 2 && parts[0] == table,
    })
}

pub fn function_of(field: &str) -> Option<(&str, &str)> {
    let open = field.find('(')?;
    if open == 0 || !field.ends_with(')') {
        return None;
    }
    Some((field[..open].trim(), field[open + 1..field.len() - 1].trim()))
}

pub fn as_index(field: &str) -> Option<usize> {
    field.to_uppercase().find(" AS ")
}

pub fn segments(name: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut quoted = false;
    let mut start = 0;
    for (index, ch) in name.char_indices() {
        if ch == '"' {
            quoted = !quoted;
        }
        if ch == '.' && !quoted {
            parts.push(&name[start..index]);
            start = index + 1;
        }
    }
    parts.push(&name[start..]);
    parts
}
