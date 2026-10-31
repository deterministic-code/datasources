use super::r#where::Where;
use super::statement_text_renderer::StatementTextRenderer;
use super::where_text::WhereText;

pub trait ConditionVisitor: Send + Sync {
    fn visit_where(&self, where_clause: &Where, text: &StatementTextRenderer) -> String;
    fn visit_where_text(&self, where_clause: &WhereText, text: &StatementTextRenderer) -> String;
}
