use datasources::{
    DeleteStatement, InsertStatement, SelectInsertedIdStatement, SelectStatement, Statement,
    StatementTextRenderer, StatementVisitor, UpdateStatement,
};

pub struct MemoryStatementRenderer;

impl StatementVisitor for MemoryStatementRenderer {
    fn visit_insert(&self, statement: &InsertStatement, text: &StatementTextRenderer) -> String {
        if statement.table().is_empty() {
            return "insert".into();
        }
        datasources::StatementRenderer.visit_insert(statement, text)
    }

    fn visit_update(&self, statement: &UpdateStatement, text: &StatementTextRenderer) -> String {
        if !statement.table().is_empty() {
            return datasources::StatementRenderer.visit_update(statement, text);
        }
        let where_text = statement
            .where_clause()
            .map(|predicate| text.render_where(predicate))
            .unwrap_or_default();
        if where_text.trim().is_empty() {
            "update".into()
        } else if where_text.trim_start().starts_with('{') {
            format!("update {where_text}")
        } else {
            datasources::StatementRenderer.visit_update(
                &UpdateStatement::new("mem", statement.fields(), statement.where_clause()),
                text,
            )
        }
    }

    fn visit_delete(&self, statement: &DeleteStatement, text: &StatementTextRenderer) -> String {
        if !statement.table().is_empty() {
            return datasources::StatementRenderer.visit_delete(statement, text);
        }
        let where_text = statement
            .where_clause()
            .map(|predicate| text.render_where(predicate))
            .unwrap_or_default();
        if where_text.trim().is_empty() {
            "delete".into()
        } else if where_text.trim_start().starts_with('{') {
            format!("delete {where_text}")
        } else {
            datasources::StatementRenderer.visit_delete(
                &DeleteStatement::new("mem", statement.where_clause()),
                text,
            )
        }
    }

    fn visit_select(&self, statement: &SelectStatement, text: &StatementTextRenderer) -> String {
        let where_text = statement
            .predicate()
            .map(|predicate| text.render_where(predicate))
            .unwrap_or_default();
        if where_text.trim_start().starts_with('{') {
            return where_text;
        }
        if statement.table.is_empty() {
            String::new()
        } else {
            datasources::StatementRenderer.visit_select(statement, text)
        }
    }

    fn visit_select_inserted_id(
        &self,
        statement: &SelectInsertedIdStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        format!(
            "{{\"__inserted\":\"{}\",\"__column\":\"{}\"}}",
            statement.table, statement.column
        )
    }
}
