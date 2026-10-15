use datasources::DropTableStatement;

pub struct OracleDropTableStatementRenderer;

impl OracleDropTableStatementRenderer {
    pub fn render(&self, statement: &DropTableStatement) -> String {
        format!(
            "BEGIN EXECUTE IMMEDIATE 'DROP TABLE {} CASCADE CONSTRAINTS'; EXCEPTION WHEN OTHERS THEN IF SQLCODE != -942 THEN RAISE; END IF; END;\n/",
            statement.table()
        )
    }
}
