use super::statement::Statement;
use super::text::Text;

pub struct CreateIndexStatement {
    text: String,
}

impl CreateIndexStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn new(name: impl AsRef<str>, table: impl AsRef<str>, columns: &[&str]) -> Self {
        Self::compile(name, table, columns, false)
    }

    pub fn unique(name: impl AsRef<str>, table: impl AsRef<str>, columns: &[&str]) -> Self {
        Self::compile(name, table, columns, true)
    }

    fn compile(
        name: impl AsRef<str>,
        table: impl AsRef<str>,
        columns: &[&str],
        unique: bool,
    ) -> Self {
        Self {
            text: format!(
                "CREATE {}INDEX {} ON {} ({})",
                if unique { "UNIQUE " } else { "" },
                name.as_ref(),
                table.as_ref(),
                columns.join(", ")
            ),
        }
    }
}

impl Text for CreateIndexStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for CreateIndexStatement {}
