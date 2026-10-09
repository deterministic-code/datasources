use datasources::IsolationLevel;

pub fn without_returning(sql: &str) -> String {
    let upper = sql.to_ascii_uppercase();
    match upper.rfind(" RETURNING ") {
        Some(index) => sql[..index].to_string(),
        None => sql.to_string(),
    }
}

pub fn to_mysql(sql: &str) -> String {
    let sql = without_returning(sql);
    let chars: Vec<char> = sql.chars().collect();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$'
            && i + 1 < chars.len()
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            out.push(':');
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                out.push(chars[i].to_ascii_lowercase());
                i += 1;
            }
            continue;
        }
        if chars[i] == '\'' || chars[i] == '"' || chars[i] == '`' {
            let quote = chars[i];
            out.push(quote);
            i += 1;
            while i < chars.len() {
                out.push(chars[i]);
                if chars[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        if chars[i].is_ascii_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let token: String = chars[start..i].iter().collect();
            let prefix: String = chars[..start].iter().collect();
            let reserved = token.eq_ignore_ascii_case("key")
                && !prefix.to_ascii_uppercase().ends_with("PRIMARY ")
                && !prefix.to_ascii_uppercase().ends_with("FOREIGN ")
                && !prefix.to_ascii_uppercase().ends_with("UNIQUE ");
            let mixed = token.chars().any(|ch| ch.is_ascii_uppercase())
                && token.chars().any(|ch| ch.is_ascii_lowercase());
            if reserved || mixed
            {
                out.push('`');
                out.push_str(&token);
                out.push('`');
            } else {
                out.push_str(&token);
            }
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

pub fn isolation_sql(isolation: IsolationLevel) -> &'static str {
    match isolation {
        IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel::ReadCommitted => "READ COMMITTED",
        IsolationLevel::RepeatableRead | IsolationLevel::Serializable => "REPEATABLE READ",
    }
}
