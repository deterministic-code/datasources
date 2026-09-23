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
    let mut out = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(next) = chars.peek() {
                if next.is_ascii_alphabetic() || *next == '_' {
                    out.push(':');
                    continue;
                }
            }
        }
        out.push(ch);
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
