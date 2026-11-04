use crate::default_value_converter::DefaultValueConverter;
use crate::default_value_options::DefaultValueOptions;
use crate::type_map::TypeMap;
use datasources::{Result, SpecField};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "`";
pub const QUOTE_RIGHT: &str = "`";
pub const PROCEDURE_PAYLOAD_TYPE: &str = "JSON";

pub static TYPES: LazyLock<TypeMap> = LazyLock::new(|| {
    TypeMap::new(HashMap::from([
        ("string".into(), "TEXT".into()),
        ("character".into(), "CHAR(1)".into()),
        ("integer".into(), "INT".into()),
        ("unsignedinteger".into(), "INT UNSIGNED".into()),
        ("smallinteger".into(), "SMALLINT".into()),
        ("unsignedsmallinteger".into(), "SMALLINT UNSIGNED".into()),
        ("biginteger".into(), "BIGINT".into()),
        ("unsignedbiginteger".into(), "BIGINT UNSIGNED".into()),
        ("float".into(), "DOUBLE".into()),
        ("decimal".into(), "DECIMAL(10, 2)".into()),
        ("boolean".into(), "TINYINT(1)".into()),
        ("datetime".into(), "DATETIME(3)".into()),
        ("binary".into(), "BLOB".into()),
        ("uuid".into(), "CHAR(36)".into()),
        ("reference".into(), "INT".into()),
    ]))
});

pub fn convert_type(field: &SpecField) -> Result<String> {
    TYPES.convert_field(field)
}

pub fn convert_default(spec_type: &str, text: &str) -> Result<Option<String>> {
    DEFAULTS.convert(spec_type, text)
}

pub static DEFAULTS: LazyLock<DefaultValueConverter> = LazyLock::new(|| {
    DefaultValueConverter::new(DefaultValueOptions {
        new_id: || "(UUID())".into(),
        now: || "(NOW(3))".into(),
        utc_now: || "(UTC_TIMESTAMP(3))".into(),
        boolean_on: "1".into(),
        boolean_off: "0".into(),
        hex: |arg| format!("(X'{arg}')"),
        hex_reverse: Regex::new(r"^(?:\(X'(.*)'\)|X'(.*)')$").expect("hex"),
        quote: Some(mysql_quote),
        quote_reverse: Some(Regex::new(r"^(?:\('(.*)'\)|'(.*)')$").expect("quote")),
    })
});

fn mysql_quote(arg: &str) -> String {
    format!("({})", DefaultValueConverter::sql_string_literal(arg))
}

const KEYWORDS: &[&str] = &[
    "ACCESSIBLE", "ADD", "AFTER", "ALL", "ALTER", "ANALYZE", "AND", "AS", "ASC", "ASENSITIVE",
    "AUTO_INCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BIGINT", "BINARY", "BLOB", "BOOLEAN", "BOTH",
    "BY", "CALL", "CASCADE", "CASE", "CAST", "CHANGE", "CHAR", "CHARACTER", "CHECK", "COLLATE",
    "COLUMN", "CONDITION", "CONSTRAINT", "CONTINUE", "CONVERT", "CREATE", "CROSS", "CUBE",
    "CUME_DIST", "CURRENT_DATE", "CURRENT_TIME", "CURRENT_TIMESTAMP", "CURRENT_USER", "CURSOR",
    "DATABASE", "DATABASES", "DATETIME", "DAY_HOUR", "DAY_MICROSECOND", "DAY_MINUTE", "DAY_SECOND",
    "DEC", "DECIMAL", "DECLARE", "DEFAULT", "DELAYED", "DELETE", "DENSE_RANK", "DESC", "DESCRIBE",
    "DETERMINISTIC", "DISTINCT", "DISTINCTROW", "DIV", "DOUBLE", "DROP", "DUAL", "EACH", "ELSE",
    "ELSEIF", "EMPTY", "ENCLOSED", "ESCAPED", "EXCEPT", "EXISTS", "EXIT", "EXPLAIN", "FALSE",
    "FETCH", "FIRST_VALUE", "FLOAT", "FLOAT4", "FLOAT8", "FOR", "FORCE", "FOREIGN", "FROM",
    "FULLTEXT", "FUNCTION", "GENERATED", "GET", "GRANT", "GROUP", "GROUPING", "GROUPS", "HAVING",
    "HIGH_PRIORITY", "HOUR_MICROSECOND", "HOUR_MINUTE", "HOUR_SECOND", "IF", "IGNORE", "IN",
    "INDEX", "INFILE", "INNER", "INOUT", "INSENSITIVE", "INSERT", "INT", "INT1", "INT2", "INT3",
    "INT4", "INT8", "INTEGER", "INTERSECT", "INTERVAL", "INTO", "IO_AFTER_GTIDS",
    "IO_BEFORE_GTIDS", "IS", "ITERATE", "JOIN", "JSON_TABLE", "KEY", "KEYS", "KILL", "LAG",
    "LAST_INSERT_ID", "LAST_VALUE", "LATERAL", "LEAD", "LEADING", "LEAVE", "LEFT", "LIKE",
    "LIMIT", "LINEAR", "LINES", "LOAD", "LOCALTIME", "LOCALTIMESTAMP", "LOCK", "LONG", "LONGBLOB",
    "LONGTEXT", "LOOP", "LOW_PRIORITY", "MASTER_BIND", "MASTER_SSL_VERIFY_SERVER_CERT", "MATCH",
    "MAXVALUE", "MEDIUMBLOB", "MEDIUMINT", "MEDIUMTEXT", "MIDDLEINT", "MINUTE_MICROSECOND",
    "MINUTE_SECOND", "MOD", "MODIFIES", "NATURAL", "NOT", "NO_WRITE_TO_BINLOG", "NOW", "NTH_VALUE",
    "NTILE", "NULL", "NUMERIC", "OF", "ON", "OPTIMIZE", "OPTIMIZER_COSTS", "OPTION", "OPTIONALLY",
    "OR", "ORDER", "OUT", "OUTER", "OUTFILE", "OVER", "PARTITION", "PERCENT_RANK", "PRECISION",
    "PRIMARY", "PROCEDURE", "PURGE", "RANGE", "RANK", "READ", "READS", "READ_WRITE", "REAL",
    "RECURSIVE", "REFERENCES", "REGEXP", "RELEASE", "RENAME", "REPEAT", "REPLACE", "REQUIRE",
    "RESIGNAL", "RESTRICT", "RETURN", "REVOKE", "RIGHT", "RLIKE", "ROW", "ROW_COUNT", "ROW_NUMBER",
    "ROWS", "SCHEMA", "SCHEMAS", "SECOND_MICROSECOND", "SELECT", "SENSITIVE", "SEPARATOR", "SET",
    "SHOW", "SIGNAL", "SMALLINT", "SPATIAL", "SPECIFIC", "SQL", "SQLEXCEPTION", "SQLSTATE",
    "SQLWARNING", "SQL_BIG_RESULT", "SQL_CALC_FOUND_ROWS", "SQL_SMALL_RESULT", "SSL", "STARTING",
    "STORED", "STRAIGHT_JOIN", "SYSTEM", "TABLE", "TERMINATED", "TEXT", "THEN", "TIMESTAMP",
    "TINYBLOB", "TINYINT", "TINYTEXT", "TO", "TRAILING", "TRIGGER", "TRUE", "UNDO", "UNION",
    "UNIQUE", "UNLOCK", "UNSIGNED", "UPDATE", "USAGE", "USE", "USER", "USING", "UTC_DATE",
    "UTC_TIME", "UTC_TIMESTAMP", "VALUES", "VARBINARY", "VARCHAR", "VARCHARACTER", "VARYING",
    "VIRTUAL", "WHEN", "WHERE", "WHILE", "WINDOW", "WITH", "WRITE", "XOR", "YEAR_MONTH",
    "ZEROFILL",
];

static SNAKE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-z][a-z0-9]*(?:_[a-z][a-z0-9]*)*$").expect("snake")
});

pub fn is_keyword(name: &str) -> bool {
    KEYWORDS.iter().any(|word| word.eq_ignore_ascii_case(name))
}

pub fn is_verbatim(name: &str) -> bool {
    !SNAKE.is_match(name)
}

static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"('(?:''|[^'])*')|(`(?:``|[^`])*`)|("(?:""|[^"])*")|(--[^\n]*)|(/\*[\s\S]*?\*/)|(?P<word>[A-Za-z_][A-Za-z0-9_]*)"#)
        .expect("token")
});

pub fn apply_keywords(sql: &str, keyword: &dyn Fn(&str) -> String) -> String {
    TOKEN
        .replace_all(sql, |caps: &regex::Captures| {
            if let Some(ident) = caps.name("word") {
                if is_keyword(ident.as_str()) {
                    return keyword(ident.as_str());
                }
            }
            caps.get(0).map(|m| m.as_str().to_string()).unwrap_or_default()
        })
        .into_owned()
}
