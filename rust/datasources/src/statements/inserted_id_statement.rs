use super::statement::Statement;
use super::text::Text;

pub struct InsertedIdStatement {
    pub table: String,
    pub column: String,
}

impl InsertedIdStatement {
    pub fn new(table: impl Into<String>, column: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            column: column.into(),
        }
    }

    pub fn select_sql(provider: &str, column: &str) -> String {
        match provider {
            "sqlite" => format!("SELECT last_insert_rowid() AS {column}"),
            "mysql" => format!("SELECT LAST_INSERT_ID() AS {column}"),
            "postgres" => format!("SELECT lastval() AS {column}"),
            _ => String::new(),
        }
    }
}

impl Text for InsertedIdStatement {
    fn get_text(&self) -> String {
        format!("{}.{}", self.table, self.column)
    }
}

impl Statement for InsertedIdStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
