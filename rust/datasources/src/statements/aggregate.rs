pub struct Aggregate;

impl Aggregate {
    pub fn count(column: &str) -> String {
        sql("COUNT", column)
    }

    pub fn sum(column: &str) -> String {
        sql("SUM", column)
    }

    pub fn min(column: &str) -> String {
        sql("MIN", column)
    }

    pub fn max(column: &str) -> String {
        sql("MAX", column)
    }
}

fn sql(op: &str, column: &str) -> String {
    format!("{op}({column})")
}
