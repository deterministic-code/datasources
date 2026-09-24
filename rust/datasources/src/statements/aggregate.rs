pub struct Aggregate;

impl Aggregate {
    pub fn count(column: &str, alias: Option<&str>) -> String {
        sql("COUNT", column, alias)
    }

    pub fn sum(column: &str, alias: Option<&str>) -> String {
        sql("SUM", column, alias)
    }

    pub fn min(column: &str, alias: Option<&str>) -> String {
        sql("MIN", column, alias)
    }

    pub fn max(column: &str, alias: Option<&str>) -> String {
        sql("MAX", column, alias)
    }
}

fn sql(op: &str, column: &str, alias: Option<&str>) -> String {
    match alias {
        Some(name) => format!("{op}({column}) AS {name}"),
        None => format!("{op}({column})"),
    }
}
